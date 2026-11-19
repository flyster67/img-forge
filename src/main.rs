#[cfg(feature = "pro")]
mod config;
#[cfg(feature = "pro")]
mod output;
mod pipeline;
#[cfg(feature = "pro")]
mod watch;

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::{bail, Context, Result};
use clap::Parser;
use pipeline::{process_file, ProcessConfig, ProcessedAsset};
use rayon::prelude::*;
use walkdir::WalkDir;

/// Re-encode PNG/JPEG images to WebP/AVIF at several widths, stripping metadata.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    input_dir: PathBuf,
    output_dir: Option<PathBuf>,
    /// Emit WebP
    #[arg(long)]
    webp: bool,
    /// Emit AVIF
    #[arg(long)]
    avif: bool,
    /// Encoder quality (1-100)
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=100))]
    quality: Option<u8>,
    /// Resize widths, comma separated (default: original width)
    #[arg(long, value_delimiter = ',')]
    widths: Vec<u32>,
    /// Path to an img-forge.toml preset file (Pro)
    #[cfg(feature = "pro")]
    #[arg(long)]
    preset: Option<PathBuf>,
    /// Watch input_dir for new or modified images (Pro)
    #[cfg(feature = "pro")]
    #[arg(long)]
    watch: bool,
}

pub(crate) fn is_supported_image(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|s| s.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some("png" | "jpg" | "jpeg")
    )
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    #[cfg(feature = "pro")]
    let (cfg, emit_html, emit_manifest) = {
        let preset = match &cli.preset {
            Some(path) => Some(config::PresetConfig::load(path)?),
            None => None,
        };
        let webp = cli.webp || preset.as_ref().is_some_and(|p| p.has_format("webp"));
        let avif = cli.avif || preset.as_ref().is_some_and(|p| p.has_format("avif"));
        if !webp && !avif {
            bail!("pass --webp, --avif, or a --preset with formats");
        }
        let output_dir = cli
            .output_dir
            .clone()
            .or_else(|| preset.as_ref().and_then(|p| p.output_dir.clone()))
            .context("pass <OUTPUT_DIR> or set output_dir in --preset")?;
        let quality = cli
            .quality
            .unwrap_or_else(|| preset.as_ref().map_or(80, |p| p.quality));
        let widths = if !cli.widths.is_empty() {
            cli.widths.clone()
        } else {
            preset.as_ref().map_or_else(Vec::new, |p| p.widths.clone())
        };
        let emit_html = preset.as_ref().is_some_and(|p| p.emit_html);
        let emit_manifest = preset.as_ref().is_some_and(|p| p.emit_manifest);
        (
            ProcessConfig {
                input_dir: cli.input_dir.clone(),
                output_dir,
                webp,
                avif,
                quality,
                widths,
            },
            emit_html,
            emit_manifest,
        )
    };

    #[cfg(not(feature = "pro"))]
    let cfg = {
        if !cli.webp && !cli.avif {
            bail!("pass --webp, --avif, or both");
        }
        let output_dir = cli.output_dir.clone().context("missing <OUTPUT_DIR>")?;
        ProcessConfig {
            input_dir: cli.input_dir.clone(),
            output_dir,
            webp: cli.webp,
            avif: cli.avif,
            quality: cli.quality.unwrap_or(80),
            widths: cli.widths,
        }
    };

    fs::create_dir_all(&cfg.output_dir)
        .with_context(|| format!("create {}", cfg.output_dir.display()))?;
    let out_canon = cfg.output_dir.canonicalize().ok();
    let files: Vec<PathBuf> = WalkDir::new(&cli.input_dir)
        .into_iter()
        .filter_entry(|e| {
            if let Some(ref out) = out_canon {
                if let Ok(canon) = e.path().canonicalize() {
                    if canon.starts_with(out) {
                        return false;
                    }
                }
            }
            true
        })
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .filter(|p| is_supported_image(p))
        .collect();

    if !files.is_empty() {
        let started = Instant::now();
        let results: Vec<_> = files
            .par_iter()
            .map(|p| (p, process_file(p, &cfg)))
            .collect();

        let (mut count, mut before, mut after) = (0usize, 0u64, 0u64);
        let mut assets: Vec<ProcessedAsset> = Vec::new();
        for (path, result) in results {
            match result {
                Ok(asset) => {
                    count += 1;
                    before += asset.original_bytes;
                    after += asset.outputs.iter().map(|o| o.bytes).sum::<u64>();
                    assets.push(asset);
                }
                Err(err) => eprintln!("skip {}: {err:#}", path.display()),
            }
        }
        println!(
            "{count} files, {:.2} MB -> {:.2} MB, {} ms",
            before as f64 / 1e6,
            after as f64 / 1e6,
            started.elapsed().as_millis()
        );

        #[cfg(feature = "pro")]
        output::write_outputs(&cfg, &assets, emit_html, emit_manifest)?;
    } else {
        #[cfg(not(feature = "pro"))]
        bail!("no PNG/JPEG files under {}", cli.input_dir.display());
        #[cfg(feature = "pro")]
        if !cli.watch {
            bail!("no PNG/JPEG files under {}", cli.input_dir.display());
        }
    }

    #[cfg(feature = "pro")]
    if cli.watch {
        watch::watch_dir(&cli.input_dir, &cfg)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_supported_image() {
        assert!(is_supported_image(Path::new("photo.png")));
        assert!(is_supported_image(Path::new("PHOTO.PNG")));
        assert!(is_supported_image(Path::new("photo.jpg")));
        assert!(is_supported_image(Path::new("PHOTO.JPG")));
        assert!(is_supported_image(Path::new("photo.jpeg")));
        assert!(is_supported_image(Path::new("PHOTO.JPEG")));
        assert!(is_supported_image(Path::new("path/to/my image.PNG")));
        assert!(!is_supported_image(Path::new("photo.webp")));
        assert!(!is_supported_image(Path::new("photo.gif")));
        assert!(!is_supported_image(Path::new("photo.txt")));
        assert!(!is_supported_image(Path::new("photo")));
    }
}
