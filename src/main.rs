// rust searches for yaml_parse.rs or ./yaml_parse/mod.rs
mod configs;
mod messages;
mod opts;
mod yaml_parse;

use getopts::Options;
use indicatif::{ProgressBar, ProgressStyle};
use std::{env, thread, time};
use yaml_rust::Yaml;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let opts = opts::init_opts(Options::new(), &program);

    let file = configs::get_config_file();
    let rules = yaml_parse::load_file(&file);

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("h") {
        messages::print_usage(&program, opts);
        return Ok(());
    }

    if matches.opt_present("p") {
        messages::print_rules(rules.unwrap());
        return Ok(());
    }

    let rule_to_apply = if matches.opt_present("r") {
        matches.opt_str("r").unwrap().clone()
    } else {
        messages::print_usage(&program, opts);
        return Ok(());
    };

    let time_left = if matches.opt_present("t") {
        matches
            .opt_str("t")
            .unwrap()
            .clone()
            .parse::<u64>()
            .unwrap()
    } else {
        messages::print_usage(&program, opts);
        return Ok(());
    };

    match rules {
        Some(x) => {
            if x.contains_key(&Yaml::from_str(&rule_to_apply)) {
                let rule = &x[&Yaml::from_str(&rule_to_apply)];
                let mode = yaml_parse::get_mode(rule);
                let sites = yaml_parse::get_sites(rule);

                let config = match mode.as_str().unwrap() {
                    "blacklist" => (configs::build_blacklist(sites)),

                    "whitelist" => (configs::build_whitelist(sites)),

                    _ => panic!(
                        "Error reading configurations, unknown mode {}",
                        mode.as_str().unwrap()
                    ),
                };

                match configs::write_to_etc_hosts(config) {
                    Ok(()) => {
                        println!("Wrote hosts file, domains blocked");

                        ctrlc::set_handler(|| {
                            println!("\nHaha, nice try\n");
                        })
                        .expect("Error setting Ctrl-C handler");
                        let handler = thread::spawn(move || {
                            let bar = ProgressBar::new(time_left);
                            bar.set_style(
                                ProgressStyle::default_bar()
                                    .template("[{elapsed}] {bar:40.cyan/blue}")
                                    .progress_chars("##-"),
                            );
                            for _i in 1..time_left {
                                bar.inc(1);
                                thread::sleep(time::Duration::from_secs(1));
                            }
                            bar.finish();
                            println!("{} seconds passed, restoring hosts", time_left);
                            configs::wipe_all();
                        });

                        return Ok(handler.join().unwrap());
                    }

                    Err(e) => {
                        println!("Not able to edit configurations. This program needs to be executed as root in order to block connections");
                        return Err(e);
                    }
                };
            } else {
                messages::print_rule_not_defined();
            }
        }
        None => {
            println!("The file /etc/lockme/rules.yaml is empty, define some rules to apply!")
        }
    }
    return Ok(());
}
