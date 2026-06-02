use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;

const FONTS: &[(&str, &str)] = &[
    (
        "Nunito-Regular.ttf",
        "https://raw.githubusercontent.com/google/fonts/main/ofl/nunito/static/Nunito-Regular.ttf",
    ),
    (
        "Nunito-Bold.ttf",
        "https://raw.githubusercontent.com/google/fonts/main/ofl/nunito/static/Nunito-Bold.ttf",
    ),
    (
        "Nunito-SemiBold.ttf",
        "https://raw.githubusercontent.com/google/fonts/main/ofl/nunito/static/Nunito-SemiBold.ttf",
    ),
];

pub fn fonts_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("aries")
        .join("fonts")
}

/// Download any missing fonts into the cache directory.
/// `set` receives (message, progress 0..1) for the loading screen.
/// Returns Ok even if a download fails — the app falls back to egui's default font.
pub fn ensure(set: impl Fn(&str, f32)) {
    let dir = fonts_dir();
    if let Err(e) = std::fs::create_dir_all(&dir) {
        log::warn!("[fonts] could not create cache dir: {e}");
        return;
    }

    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(15))
        .build();

    let missing: Vec<_> = FONTS
        .iter()
        .filter(|(name, _)| !dir.join(name).exists())
        .collect();

    if missing.is_empty() { return; }

    for (i, (name, url)) in missing.iter().enumerate() {
        let progress = 0.05 + (i as f32 / missing.len() as f32) * 0.10;
        set(&format!("Descargando fuente {}…", name.replace(".ttf", "")), progress);
        log::info!("[fonts] downloading {name}");

        match agent.get(url).call() {
            Err(e) => {
                log::warn!("[fonts] download failed for {name}: {e}");
            }
            Ok(resp) => {
                let mut bytes = Vec::new();
                if let Err(e) = resp.into_reader().read_to_end(&mut bytes) {
                    log::warn!("[fonts] read failed for {name}: {e}");
                    continue;
                }
                if let Err(e) = std::fs::write(dir.join(name), &bytes) {
                    log::warn!("[fonts] write failed for {name}: {e}");
                } else {
                    log::info!("[fonts] saved {name} ({} KB)", bytes.len() / 1024);
                }
            }
        }
    }
}

/// Read a cached font file. Returns empty Vec if missing (egui uses its default).
pub fn load(name: &str) -> Vec<u8> {
    std::fs::read(fonts_dir().join(name)).unwrap_or_default()
}
