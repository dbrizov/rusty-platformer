use sdl2::keyboard::Scancode;
use serde::Deserialize;
use std::collections::HashMap;

fn get_input_settings_path() -> String {
    String::from(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("config")
            .join("input_settings.json")
            .to_str()
            .unwrap(),
    )
}

pub fn get_input_mappings() -> InputMappings {
    let input_settings_path: String = get_input_settings_path();
    let input_config: InputConfig = InputConfig::from_file(&input_settings_path).unwrap();
    let input_mappings: InputMappings = InputMappings::from_config(input_config).unwrap();
    dbg!(&input_mappings);

    return input_mappings;
}

#[derive(Debug, Deserialize)]
struct AxisConfig {
    pub acceleration: f32,
    pub deceleration: f32,
    pub positive: Vec<String>,
    pub negative: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct InputConfig {
    pub action_mappings: HashMap<String, Vec<String>>,
    pub axis_mappings: HashMap<String, AxisConfig>,
}

impl InputConfig {
    fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json: String = std::fs::read_to_string(path)?;
        let config: InputConfig = serde_json::from_str(&json)?;
        Ok(config)
    }
}

#[derive(Debug)]
pub struct AxisMapping {
    pub acceleration: f32,
    pub deceleration: f32,
    pub positive: Vec<Scancode>,
    pub negative: Vec<Scancode>,
}

#[derive(Debug)]
pub struct InputMappings {
    pub actions: HashMap<String, Vec<Scancode>>,
    pub axes: HashMap<String, AxisMapping>,
}

impl InputMappings {
    fn from_config(config: InputConfig) -> Result<Self, String> {
        let mut actions: HashMap<String, Vec<Scancode>> = HashMap::new();
        let mut axes: HashMap<String, AxisMapping> = HashMap::new();

        // Actions
        for (action, keys) in config.action_mappings {
            let mut scancodes: Vec<Scancode> = Vec::new();
            for key in keys {
                let sc: Scancode =
                    Scancode::from_name(&key).ok_or_else(|| format!("Unknown key: {}", key))?;
                scancodes.push(sc);
            }

            actions.insert(action, scancodes);
        }

        // Axes
        for (axis, cfg) in config.axis_mappings {
            let positive: Vec<Scancode> = cfg
                .positive
                .iter()
                .map(|k| Scancode::from_name(k).ok_or(k.clone()))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|k| format!("Unknown key: {}", k))?;

            let negative: Vec<Scancode> = cfg
                .negative
                .iter()
                .map(|k| Scancode::from_name(k).ok_or(k.clone()))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|k| format!("Unknown key: {}", k))?;

            axes.insert(
                axis,
                AxisMapping {
                    acceleration: cfg.acceleration,
                    deceleration: cfg.deceleration,
                    positive,
                    negative,
                },
            );
        }

        Ok(Self { actions, axes })
    }
}
