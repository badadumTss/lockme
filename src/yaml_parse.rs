use std::fs::File;
use std::io::prelude::*;
use yaml_rust::yaml::{Hash, Yaml};
use yaml_rust::YamlLoader;

pub fn load_file(mut file: &File) -> Option<Hash> {
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let rules = YamlLoader::load_from_str(&contents).unwrap();

    if rules.is_empty() {
        return None;
    }
    return rules[0].clone().into_hash();
}

pub fn get_mode(rule: &Yaml) -> Yaml {
    return rule["mode"].clone();
}

pub fn get_sites(rule: &Yaml) -> Yaml {
    return rule["sites"].clone();
}
