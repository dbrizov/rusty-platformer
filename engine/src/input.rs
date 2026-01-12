use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct InputConfig {
    pub action_mappings: HashMap<String, Vec<String>>,
    pub axis_mappings: HashMap<String, AxisConfig>,
}

#[derive(Debug, Deserialize)]
pub struct AxisConfig {
    pub acceleration: f32,
    pub deceleration: f32,
    pub positive: Vec<String>,
    pub negative: Vec<String>,
}

pub fn load_input_config(path: &str) -> Result<InputConfig, Box<dyn std::error::Error>> {
    let json: String = fs::read_to_string(path)?;
    let config: InputConfig = serde_json::from_str(&json)?;
    Ok(config)
}

fn get_input_settings_path() -> String {
    String::from(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("config")
            .join("input_settings.json")
            .to_str()
            .unwrap(),
    )
}

pub fn get_input_config() -> InputConfig {
    let input_settings_path: String = get_input_settings_path();
    let input_config: InputConfig = load_input_config(&input_settings_path).unwrap();
    dbg!(&input_config);

    return input_config;
}
