use getopts::Options;
use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

pub fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} -r RULE -t TIME [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn print_rule_not_defined() {
    println!("the selected rule was not specified in the configuration file");
}

pub fn print_rules(rules: Hash) {
    for key in rules {
        println!("Rule: {}", key.0.as_str().unwrap());
        println!(
            "Mode: {}",
            key.1.as_hash().unwrap()[&Yaml::from_str("mode")]
                .as_str()
                .unwrap()
        );
        println!("Sites:");
        for site in key.1.as_hash().unwrap()[&Yaml::from_str("sites")].clone() {
            println!("\t{}", site.as_str().unwrap());
        }
        println!("");
    }
}
