use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use notify::event::{AccessKind, AccessMode, ModifyKind};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::is_supported_image;
use crate::pipeline::{process_file, ProcessConfig};

fn is_write_event(kind: &EventKind) -> bool {
    matches!(
        kind,
        EventKind::Create(_)
            | EventKind::Modify(ModifyKind::Data(_) | ModifyKind::Name(_) | ModifyKind::Any)
            | EventKind::Access(AccessKind::Close(AccessMode::Write))
    )
}

fn fmt_bytes(b: u64) -> String {
    if b >= 1_000_000 {
        format!("{:.2} MB", b as f64 / 1e6)
    } else {
        format!("{:.1} KB", b as f64 / 1e3)
    }
}

pub fn watch_dir(input_dir: &Path, cfg: &ProcessConfig) -> Result<()> {
    fs::create_dir_all(&cfg.output_dir)
        .with_context(|| format!("create {}", cfg.output_dir.display()))?;

    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())
        .context("initialize file watcher")?;
    watcher
        .watch(input_dir, RecursiveMode::Recursive)
        .with_context(|| format!("watch {}", input_dir.display()))?;

    let out_canon = cfg.output_dir.canonicalize().ok();
    let debounce = Duration::from_millis(250);
    let mut pending: HashMap<PathBuf, Instant> = HashMap::new();

    println!(
        "Watching {} -> {} (Ctrl+C to stop)",
        input_dir.display(),
        cfg.output_dir.display()
    );

    loop {
        let event = match rx.recv_timeout(Duration::from_millis(50)) {
            Ok(res) => Some(res),
            Err(mpsc::RecvTimeoutError::Timeout) => None,
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        };

        if let Some(first_res) = event {
            let mut events = vec![first_res];
            while let Ok(next_res) = rx.try_recv() {
                events.push(next_res);
            }
            for res in events {
                match res {
                    Ok(event) => match event.kind {
                        EventKind::Remove(_) => {
                            for path in event.paths {
                                pending.remove(&path);
                            }
                        }
                        ref kind if is_write_event(kind) => {
                            for path in event.paths {
                                if !is_supported_image(&path) {
                                    continue;
                                }
                                let in_output = if let Some(ref out) = out_canon {
                                    if let Ok(p_canon) = path.canonicalize() {
                                        p_canon.starts_with(out)
                                    } else {
                                        path.starts_with(&cfg.output_dir)
                                    }
                                } else {
                                    path.starts_with(&cfg.output_dir)
                                };
                                if in_output {
                                    continue;
                                }
                                pending.insert(path, Instant::now());
                            }
                        }
                        _ => {}
                    },
                    Err(err) => eprintln!("[watch] error: {err}"),
                }
            }
        }

        let ready: Vec<PathBuf> = pending
            .iter()
            .filter(|(_, ts)| ts.elapsed() >= debounce)
            .map(|(p, _)| p.clone())
            .collect();

        for path in ready {
            pending.remove(&path);
            if !path.is_file() {
                continue;
            }
            let t0 = Instant::now();
            match process_file(&path, cfg) {
                Ok(asset) => {
                    let out_bytes: u64 = asset.outputs.iter().map(|o| o.bytes).sum();
                    println!(
                        "[watch] {} -> {} files ({} -> {}) in {} ms",
                        path.display(),
                        asset.outputs.len(),
                        fmt_bytes(asset.original_bytes),
                        fmt_bytes(out_bytes),
                        t0.elapsed().as_millis()
                    );
                }
                Err(err) => eprintln!("[watch] skip {}: {err:#}", path.display()),
            }
        }
    }

    Ok(())
}
