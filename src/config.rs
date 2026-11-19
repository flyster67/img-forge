use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

fn default_quality() -> u8 {
    80
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct PresetConfig {
    #[serde(default)]
    pub formats: Vec<String>,
    #[serde(default = "default_quality")]
    pub quality: u8,
    #[serde(default)]
    pub widths: Vec<u32>,
    pub output_dir: Option<PathBuf>,
    #[serde(default = "default_true")]
    pub emit_html: bool,
    #[serde(default = "default_true")]
    pub emit_manifest: bool,
}

impl PresetConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let raw =
            fs::read_to_string(path).with_context(|| format!("read preset {}", path.display()))?;
        toml::from_str(&raw).with_context(|| format!("parse preset {}", path.display()))
    }

    pub fn has_format(&self, name: &str) -> bool {
        self.formats.iter().any(|f| f.eq_ignore_ascii_case(name))
    }
}
