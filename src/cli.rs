pub struct Config {
    pub show_all: bool,
    pub long: bool,
    pub human: bool,
    pub path: String,
}

pub fn parse_args(args: &[String]) -> Config {
    let mut show_all = false;
    let mut long = false;
    let mut human = false;
    let mut path = ".".to_string();

for arg in args.iter().skip(1) {
    if let Some(flags) = arg.strip_prefix('-') {
        for flag in flags.chars() {
            match flag {
                'a' => show_all = true,
                'l' => long = true,
                'h' => human = true,
                _ => eprintln!("Unknown option: -{}", flag),
            }
        }
    } else {
        path = arg.clone();
    }
}

    Config { show_all, long, human, path }
}
