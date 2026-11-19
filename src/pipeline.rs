use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use image::imageops::FilterType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Webp,
    Avif,
}

#[allow(dead_code)] // public API: callers read widths/formats/paths
#[derive(Debug, Clone)]
pub struct OutputFile {
    pub path: PathBuf,
    pub width: u32,
    pub format: Format,
    pub bytes: u64,
}

#[allow(dead_code)] // public API
#[derive(Debug, Clone)]
pub struct ProcessedAsset {
    pub original: PathBuf,
    pub original_bytes: u64,
    pub outputs: Vec<OutputFile>,
}

#[derive(Debug, Clone)]
pub struct ProcessConfig {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub webp: bool,
    pub avif: bool,
    pub quality: u8,
    pub widths: Vec<u32>,
}

pub fn process_file(path: &Path, cfg: &ProcessConfig) -> Result<ProcessedAsset> {
    let original_bytes = fs::metadata(path)?.len();
    if original_bytes == 0 {
        anyhow::bail!("file is empty (0 bytes)");
    }
    // `image` yields pixels only; re-encoding below never carries EXIF/GPS/XMP across.
    let img = image::open(path).with_context(|| format!("open {}", path.display()))?;
    if img.width() == 0 || img.height() == 0 {
        anyhow::bail!("image has zero dimensions: {}x{}", img.width(), img.height());
    }

    let mut widths = if cfg.widths.is_empty() {
        vec![img.width()]
    } else {
        cfg.widths.clone()
    };
    widths.retain(|w| *w > 0 && *w <= img.width());
    if widths.is_empty() {
        widths.push(img.width());
    }
    widths.sort_unstable();
    widths.dedup();

    let rel_parent: Option<PathBuf> = path
        .strip_prefix(&cfg.input_dir)
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .or_else(|| {
            if let (Ok(p_canon), Ok(in_canon)) = (path.canonicalize(), cfg.input_dir.canonicalize()) {
                p_canon
                    .strip_prefix(&in_canon)
                    .ok()
                    .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            } else {
                None
            }
        });

    let target_dir = match rel_parent {
        Some(parent) if !parent.as_os_str().is_empty() => cfg.output_dir.join(parent),
        _ => cfg.output_dir.clone(),
    };

    fs::create_dir_all(&target_dir)
        .with_context(|| format!("create {}", target_dir.display()))?;
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("image");
    let mut outputs = Vec::new();

    for width in widths {
        let height =
            (u64::from(img.height()) * u64::from(width) / u64::from(img.width())).max(1) as u32;
        let rgba = if width == img.width() && height == img.height() {
            img.to_rgba8()
        } else {
            img.resize_exact(width, height, FilterType::Lanczos3)
                .into_rgba8()
        };

        if cfg.webp {
            let config = zenwebp::LossyConfig::new().with_quality(f32::from(cfg.quality));
            let data = zenwebp::EncodeRequest::lossy(
                &config,
                rgba.as_raw(),
                zenwebp::PixelLayout::Rgba8,
                width,
                height,
            )
            .encode()
            .with_context(|| format!("encode webp {}", path.display()))?;
            emit(
                &target_dir,
                stem,
                width,
                "webp",
                Format::Webp,
                &data,
                &mut outputs,
            )?;
        }

        if cfg.avif {
            let raw = rgba.as_raw();
            // SAFETY: ravif::RGBA8 is rgb::RGBA8 which is #[repr(C)] struct RGBA<u8> { r: u8, g: u8, b: u8, a: u8 }
            // with size 4 and alignment 1. raw is a multiple of 4 bytes with length width * height * 4.
            let pixels: &[ravif::RGBA8] = unsafe {
                std::slice::from_raw_parts(raw.as_ptr() as *const ravif::RGBA8, raw.len() / 4)
            };
            let encoded = ravif::Encoder::new()
                .with_quality(f32::from(cfg.quality))
                .with_speed(6)
                .encode_rgba(ravif::Img::new(pixels, width as usize, height as usize))
                .with_context(|| format!("encode avif {}", path.display()))?;
            emit(
                &target_dir,
                stem,
                width,
                "avif",
                Format::Avif,
                &encoded.avif_file,
                &mut outputs,
            )?;
        }
    }

    Ok(ProcessedAsset {
        original: path.to_path_buf(),
        original_bytes,
        outputs,
    })
}

fn emit(
    dir: &Path,
    stem: &str,
    width: u32,
    ext: &str,
    format: Format,
    data: &[u8],
    outputs: &mut Vec<OutputFile>,
) -> Result<()> {
    let path = dir.join(format!("{stem}-{width}.{ext}"));
    fs::write(&path, data).with_context(|| format!("write {}", path.display()))?;
    outputs.push(OutputFile {
        path,
        width,
        format,
        bytes: data.len() as u64,
    });
    Ok(())
}
