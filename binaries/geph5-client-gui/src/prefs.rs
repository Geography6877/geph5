use std::path::PathBuf;

use anyhow::Context as _;
use moka::sync::Cache;
use once_cell::sync::Lazy;
use smol_str::SmolStr;

static PREF_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = dirs::config_dir()
        .context("no config dir")
        .unwrap()
        .join("geph5-prefs");
    std::fs::create_dir_all(&dir).unwrap();
    dir
});

static PREF_CACHE: Lazy<Cache<SmolStr, SmolStr>> = Lazy::new(|| Cache::new(10000));

pub fn pref_write(key: &str, val: &str) -> anyhow::Result<()> {
    PREF_CACHE.remove(key);
    let key_path = PREF_DIR.join(key);
    std::fs::write(key_path, val.as_bytes())?;
    Ok(())
}

pub fn pref_read(key: &str) -> anyhow::Result<SmolStr> {
    PREF_CACHE
        .try_get_with(key.into(), || {
            let key_path = PREF_DIR.join(key);
            let contents = std::fs::read_to_string(key_path)?;
            anyhow::Ok(SmolStr::from(contents))
        })
        .map_err(|e| anyhow::anyhow!("{e}"))
}
