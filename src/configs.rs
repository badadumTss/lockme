use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::Write;
use std::path::Path;
use yaml_rust::Yaml;

macro_rules! config_file {
    () => {
        "/etc/lockme/rules.yaml"
    };
}

macro_rules! etc_hosts {
    () => {
        "/etc/hosts"
    };
}

macro_rules! etc_hosts_bu {
    () => {
        etc_hosts!().to_owned() + ".lockme.backup"
    };
}

pub fn get_config_file() -> File {
    let file = Path::new(config_file!());
    if !file.exists() {
        let path = file.parent().unwrap();
        if !path.exists() {
            std::fs::create_dir_all(path).unwrap();
            return File::create(file).expect("Unable to create file");
        }
    }
    return File::open(file).expect("Unable to open file");
}

pub fn build_blacklist(sites: Yaml) -> String {
    let mut contents = String::from("");
    for site in sites {
        contents += &("0.0.0.0\t".to_owned() + &(site.clone()).into_string().unwrap() + "\n");
        contents += &("0.0.0.0\twww.".to_owned() + &(site.clone()).into_string().unwrap() + "\n");
    }
    return wrap(contents);
}

pub fn build_whitelist(sites: Yaml) -> String {
    let mut contents = String::from("");
    for site in sites {
        contents += &("0.0.0.0\t".to_owned() + &(site.clone()).into_string().unwrap() + "\n");
        contents += &("0.0.0.0\twww.".to_owned() + &(site.clone()).into_string().unwrap() + "\n");
    }
    return wrap(contents);
}

pub fn wrap(contents: String) -> String {
    let mut to_return = String::from("### lockme configs, DO NOT TOUCH ###\n\n");
    to_return += &contents;
    to_return += "\n### end lockme configs ###\n";
    return to_return;
}

pub fn write_to_etc_hosts(config: String) -> Result<(), Error> {
    // copy old file to backup location
    match fs::copy(etc_hosts!(), etc_hosts_bu!()) {
        Ok(_) => {
            // write in new copied file
            let mut etc_hosts = std::fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(etc_hosts!())
                .unwrap();

            return write!(etc_hosts, "{}", config);
        }
        Err(e) => return Err(e),
    };
}

pub fn restore_etc_hosts() -> Result<(), Error> {
    return fs::rename(etc_hosts_bu!(), etc_hosts!());
}

pub fn etc_hosts_exists() -> bool {
    return Path::new(etc_hosts!()).exists();
}

pub fn wipe_all() {
    match restore_etc_hosts() {
        Ok(_) => {
            println!("Hosts restored");
        }
        Err(e) => {
            let presence = etc_hosts_exists();
            panic!(
                "Unable to restore host files, something went wrong. Backed up file is {} at /etc/hosts.lockme.bu \nError: {}",
                presence, e
            )
        }
    }
}

// pub fn build_whitelist(sites: Yaml) -> Vec<Yaml> {
//     return sites.as_vec().unwrap().clone();
// }
