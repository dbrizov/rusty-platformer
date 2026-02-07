use std::env;
use std::path::PathBuf;

pub fn get_root_path() -> PathBuf {
    let is_debug_build = cfg!(debug_assertions);
    let root_path;
    if is_debug_build {
        // Return the root directory of the Cargo.toml file
        root_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("Failed to find the root of the project")
            .to_path_buf();
    } else {
        // Return the current directory of the executable
        root_path = env::current_exe()
            .expect("Failed to get executable path")
            .parent()
            .expect("Exe has no parent")
            .to_path_buf();
    }

    root_path
}

pub fn get_input_config_path() -> PathBuf {
    let is_debug_build = cfg!(debug_assertions);
    let root_path = get_root_path();
    let config_file_path;

    if is_debug_build {
        config_file_path = root_path.join("engine/config/input_config.json");
    } else {
        config_file_path = root_path.join("config/input_config.json");
    }

    println!("input_config_path: '{}'", config_file_path.display());

    config_file_path
}

pub fn get_assets_root_path() -> PathBuf {
    let is_debug_build = cfg!(debug_assertions);
    let root_path = get_root_path();
    let assets_root_path;

    if is_debug_build {
        assets_root_path = root_path.join("game/assets");
    } else {
        assets_root_path = root_path.join("assets");
    }

    println!("assets_root_path: '{}'", assets_root_path.display());

    assets_root_path
}
