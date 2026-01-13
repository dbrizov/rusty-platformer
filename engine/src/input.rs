use sdl2::keyboard::Scancode;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

// Public
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum InputEventType {
    Pressed,
    Released,
    Axis,
}

#[derive(Debug)]
pub struct InputEvent<'a> {
    pub ev_name: &'a str,
    pub ev_type: InputEventType,
    pub axis_value: f32,
}

pub struct Input {
    pub on_input_event: Vec<Box<dyn Fn(&InputEvent)>>,

    m_input_mappings: InputMappings,
    m_axis_values: HashMap<String, f32>,
    m_relevant_keys: Vec<Scancode>,
    m_pressed_keys_this_frame: HashSet<Scancode>,
    m_pressed_keys_last_frame: HashSet<Scancode>,
}

impl Input {
    pub fn new() -> Result<Self, String> {
        let input_mappings: InputMappings = get_input_mappings()?;
        let relevant_keys: Vec<Scancode> = input_mappings.relevant_keys();
        let axis_values: HashMap<String, f32> = input_mappings
            .axes
            .keys()
            .map(|k| (k.clone(), 0.0))
            .collect();

        Ok(Input {
            on_input_event: Vec::new(),
            m_input_mappings: input_mappings,
            m_axis_values: axis_values,
            m_relevant_keys: relevant_keys,
            m_pressed_keys_this_frame: HashSet::new(),
            m_pressed_keys_last_frame: HashSet::new(),
        })
    }

    pub fn tick(&mut self, delta_time: f32, keyboard_state: &sdl2::keyboard::KeyboardState) {
        self.update_pressed_keys(keyboard_state);

        // Dispatch action events
        for (action, keys) in &self.m_input_mappings.actions {
            for key in keys {
                let pressed_now: bool = self.m_pressed_keys_this_frame.contains(key);
                let pressed_before: bool = self.m_pressed_keys_last_frame.contains(key);

                if pressed_now && !pressed_before {
                    self.dispatch_event(InputEvent {
                        ev_name: action,
                        ev_type: InputEventType::Pressed,
                        axis_value: 0.0,
                    });
                } else if pressed_before && !pressed_now {
                    self.dispatch_event(InputEvent {
                        ev_name: action,
                        ev_type: InputEventType::Released,
                        axis_value: 0.0,
                    });
                }
            }
        }

        // Dispatch axis events
        for (axis, axis_mapping) in &self.m_input_mappings.axes {
            let any_positive: bool = axis_mapping
                .positive
                .iter()
                .any(|key: &Scancode| self.m_pressed_keys_this_frame.contains(key));
            let any_negative: bool = axis_mapping
                .negative
                .iter()
                .any(|key: &Scancode| self.m_pressed_keys_this_frame.contains(key));

            let new_axis_value = {
                let axis_value: &mut f32 = self.m_axis_values.get_mut(axis).unwrap();

                if (any_positive && any_negative) || (!any_positive && !any_negative) {
                    if *axis_value < 0.0 {
                        *axis_value =
                            (*axis_value + axis_mapping.deceleration * delta_time).clamp(-1.0, 0.0);
                    } else if *axis_value > 0.0 {
                        *axis_value =
                            (*axis_value - axis_mapping.deceleration * delta_time).clamp(0.0, 1.0);
                    }
                } else if any_positive {
                    *axis_value =
                        (*axis_value + axis_mapping.acceleration * delta_time).clamp(-1.0, 1.0);
                } else if any_negative {
                    *axis_value =
                        (*axis_value - axis_mapping.acceleration * delta_time).clamp(-1.0, 1.0);
                }

                *axis_value
            };

            self.dispatch_event(InputEvent {
                ev_name: axis,
                ev_type: InputEventType::Axis,
                axis_value: new_axis_value,
            });
        }
    }

    fn update_pressed_keys(&mut self, keyboard_state: &sdl2::keyboard::KeyboardState) {
        self.m_pressed_keys_last_frame.clear();
        for key in &self.m_pressed_keys_this_frame {
            self.m_pressed_keys_last_frame.insert(*key);
        }

        self.m_pressed_keys_this_frame.clear();
        for key in &self.m_relevant_keys {
            if keyboard_state.is_scancode_pressed(*key) {
                self.m_pressed_keys_this_frame.insert(*key);
            }
        }
    }

    fn dispatch_event(&self, event: InputEvent) {
        for handler in &self.on_input_event {
            handler(&event);
        }
    }
}

#[derive(Debug, Deserialize)]
struct AxisConfig {
    acceleration: f32,
    deceleration: f32,
    positive: Vec<String>,
    negative: Vec<String>,
}

// Private
#[derive(Debug, Deserialize)]
struct InputConfig {
    action_mappings: HashMap<String, Vec<String>>,
    axis_mappings: HashMap<String, AxisConfig>,
}

impl InputConfig {
    fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json: String = std::fs::read_to_string(path)?;
        let config: InputConfig = serde_json::from_str(&json)?;
        Ok(config)
    }
}

#[derive(Debug)]
struct AxisMapping {
    acceleration: f32,
    deceleration: f32,
    positive: Vec<Scancode>,
    negative: Vec<Scancode>,
}

#[derive(Debug)]
struct InputMappings {
    actions: HashMap<String, Vec<Scancode>>,
    axes: HashMap<String, AxisMapping>,
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
                .collect::<Result<Vec<Scancode>, String>>()
                .map_err(|k| format!("Unknown key: {}", k))?;

            let negative: Vec<Scancode> = cfg
                .negative
                .iter()
                .map(|k| Scancode::from_name(k).ok_or(k.clone()))
                .collect::<Result<Vec<Scancode>, String>>()
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

    fn relevant_keys(&self) -> Vec<Scancode> {
        let mut keys_set: HashSet<Scancode> = HashSet::new();
        let mut keys_vec: Vec<Scancode> = Vec::new();
        for pair in &self.actions {
            for key in pair.1 {
                if keys_set.insert(*key) {
                    keys_vec.push(*key);
                }
            }
        }

        for pair in &self.axes {
            for key in &pair.1.positive {
                if keys_set.insert(*key) {
                    keys_vec.push(*key);
                }
            }

            for key in &pair.1.negative {
                if keys_set.insert(*key) {
                    keys_vec.push(*key);
                }
            }
        }

        keys_vec
    }
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

fn get_input_mappings() -> Result<InputMappings, String> {
    let input_settings_path: String = get_input_settings_path();
    let input_config: InputConfig = InputConfig::from_file(&input_settings_path)
        .map_err(|err| format!("Failed to create InputConfig from file: {}", err))?;
    let input_mappings: InputMappings = InputMappings::from_config(input_config)?;

    Ok(input_mappings)
}
