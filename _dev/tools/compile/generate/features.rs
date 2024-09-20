use crate::toml;
use crate::util;

#[allow(unused_variables)]
pub fn generate(feature_list: &mut Vec<String>) {
  let mut behaviors = vec![];
  for file in util::list_files("src/orbit/features/behaviors") {
    let behavior = util::filename_no_ext(&file);
    behaviors.push(format!("behavior_{}_enabled", behavior));
  }

  let mut actions = vec![];
  for file in util::list_files("src/orbit/features/actions") {
    let action = util::filename_no_ext(&file);
    actions.push(format!("action_{}_enabled", action));
  }

  let mut flavors = vec![];
  for file in util::list_files("src/orbit/features/flavors") {
    let flavor = util::filename_no_ext(&file);
    flavors.push(format!("flavor_{}_enabled", flavor));
  }

  let config = toml::read("keyboard.toml", true);
  let chip: String = toml::get(&config, "keyboard/chip", true);
  let family = util::get_chip_family(&chip);
  feature_list.push(format!("family_{}", family));

  let families: Vec<String> = vec![
    "family_NONE".to_string(),
    "family_STM32".to_string(),
    "family_NRF".to_string(),
    "family_ESP".to_string(),
    "family_RP".to_string(),
    "family_CH".to_string(),
    "family_EMULATOR".to_string(),
  ];

  let mut cargo = toml::read_as_value("Cargo.toml");

  if let toml::Value::Table(ref mut root_table) = cargo {
    let features = root_table
      .entry("features")
      .or_insert_with(|| toml::Value::Table(toml::Map::new()));

    if let toml::Value::Table(ref mut features) = features {
      for behavior in &behaviors {
        features.insert(behavior.to_string(), toml::Value::Array(vec![]));
      }

      for action in &actions {
        features.insert(action.to_string(), toml::Value::Array(vec![]));
      }

      for flavor in &flavors {
        features.insert(flavor.to_string(), toml::Value::Array(vec![]));
      }

      for family in &families {
        features.insert(family.to_string(), toml::Value::Array(vec![]));
      }

      features.insert(
        "default".to_string(),
        toml::Value::Array(feature_list.iter().map(|f| toml::Value::String(f.clone())).collect()),
      );
    }
  }

  toml::write("Cargo.toml", &cargo);
}
