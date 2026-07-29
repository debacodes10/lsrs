pub struct Config {
    pub show_all: bool,
    pub long: bool,
    pub path: String,
}

pub fn parse_args(args: &[String]) -> Config {
    let mut show_all = false;
    let mut long = false;
    let mut path = ".".to_string();

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-a" => show_all = true,
            "-l" => long = true,
            "-al" | "-la" => {
                show_all = true;
                long = true;
            }
            other => path = other.to_string(),
        }
    }

    Config { show_all, long, path }
}
