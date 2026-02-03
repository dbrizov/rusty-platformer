use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn dir_stats(p: &Path) -> (usize, usize) {
    // (files, dirs) best-effort; if unreadable, returns (0,0)
    fn walk(p: &Path, files: &mut usize, dirs: &mut usize) {
        let Ok(rd) = fs::read_dir(p) else {
            return;
        };

        for e in rd.flatten() {
            let Ok(t) = e.file_type() else {
                continue;
            };
            if t.is_dir() {
                *dirs += 1;
                walk(&e.path(), files, dirs);
            } else {
                *files += 1;
            }
        }
    }

    let mut files = 0;
    let mut dirs = 0;

    if p.exists() {
        walk(p, &mut files, &mut dirs);
    }

    (files, dirs)
}

fn warn_kv(key: &str, val: impl std::fmt::Display) {
    println!("cargo:warning={key} = {val}");
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            // create parent dirs just in case
            if let Some(p) = to.parent() {
                fs::create_dir_all(p)?;
            }
            fs::copy(&from, &to)?;
        }
    }

    Ok(())
}

fn main() {
    // Re-run triggers
    println!("cargo:rerun-if-changed=assets");
    println!("cargo:rerun-if-changed=../engine/config");

    // Core env
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR missing"));
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR missing"));
    warn_kv(
        "CARGO_MANIFEST_DIR",
        format!("'{}'", manifest_dir.display()),
    );
    warn_kv("OUT_DIR", format!("'{}'", out_dir.display()));

    // Helpful build context (these are often set; log if present)
    for k in [
        "PROFILE",
        "TARGET",
        "HOST",
        "OPT_LEVEL",
        "DEBUG",
        "CARGO_CFG_TARGET_OS",
        "CARGO_CFG_TARGET_ARCH",
        "CARGO_CFG_TARGET_ENV",
        "CARGO_CFG_TARGET_FAMILY",
        "CARGO_TARGET_DIR",
    ] {
        if let Ok(v) = env::var(k) {
            warn_kv(k, format!("'{}'", v));
        }
    }

    // --- Show OUT_DIR ancestor chain so you can pick the right nth()
    // warn_kv("OUT_DIR_ANCESTORS", "(closest first)");
    // for (i, a) in out_dir.ancestors().take(8).enumerate() {
    //     println!("cargo:warning=  [{i}] '{}'", a.display());
    // }

    let target_profile_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("Failed to locate target/<profile> dir from OUT_DIR")
        .to_path_buf();
    warn_kv(
        "TARGET_PROFILE_DIR",
        format!("'{}'", target_profile_dir.display()),
    );
    warn_kv("TARGET_PROFILE_DIR_EXISTS", target_profile_dir.exists());

    // --- Source/destination paths
    let config_src = manifest_dir.join("../engine/config");
    let config_dst = target_profile_dir.join("engine").join("config");
    warn_kv("CONFIG_SRC", format!("'{}'", config_src.display()));
    warn_kv("CONFIG_DST", format!("'{}'", config_dst.display()));
    warn_kv("CONFIG_SRC_EXISTS", config_src.exists());

    let assets_src = manifest_dir.join("assets");
    let assets_dst = target_profile_dir.join("game").join("assets");
    warn_kv("ASSETS_SRC", format!("'{}'", assets_src.display()));
    warn_kv("ASSETS_DST", format!("'{}'", assets_dst.display()));
    warn_kv("ASSETS_SRC_EXISTS", assets_src.exists());

    // Stats before copy
    let (cfg_files, cfg_dirs) = dir_stats(&config_src);
    let (ast_files, ast_dirs) = dir_stats(&assets_src);
    warn_kv(
        "CONFIG_SRC_STATS",
        format!("{cfg_files} files, {cfg_dirs} dirs"),
    );
    warn_kv(
        "ASSETS_SRC_STATS",
        format!("{ast_files} files, {ast_dirs} dirs"),
    );

    // --- Copy work (and log completion)
    // You might want to early-fail if src missing:
    if !config_src.exists() {
        println!("cargo:warning=ERROR: Missing config directory");
        panic!("CONFIG_SRC not found: {}", config_src.display());
    }
    if !assets_src.exists() {
        println!("cargo:warning=ERROR: Missing assets directory");
        panic!("ASSETS_SRC not found: {}", assets_src.display());
    }

    copy_dir_recursive(&config_src, &config_dst).expect("Copy engine/config failed");
    warn_kv("COPIED_CONFIG_TO", format!("'{}'", config_dst.display()));

    copy_dir_recursive(&assets_src, &assets_dst).expect("Copy game/assets failed");
    warn_kv("COPIED_ASSETS_TO", format!("'{}'", assets_dst.display()));

    // Optional: stats after copy
    let (cfg_files2, cfg_dirs2) = dir_stats(&config_dst);
    let (ast_files2, ast_dirs2) = dir_stats(&assets_dst);
    warn_kv(
        "CONFIG_DST_STATS",
        format!("{cfg_files2} files, {cfg_dirs2} dirs"),
    );
    warn_kv(
        "ASSETS_DST_STATS",
        format!("{ast_files2} files, {ast_dirs2} dirs"),
    );
}
