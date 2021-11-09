use getopts::Options;

pub fn init_opts(mut opts: Options, program: &str) -> Options {
    opts.optflag(
        "w",
        "wipe",
        format!(
            "Wipe old {} configurations from configuration files",
            program
        )
        .as_str()
        .clone(),
    );

    opts.optflag("h", "help", "show this help menu");

    opts.optopt("r", "rule", "set rule to apply", "RULE");

    opts.optopt(
        "t",
        "time",
        "set time after wich restore old configurations",
        "TIME",
    );

    opts.optflag("p", "print", "print available rules");

    return opts;
}
