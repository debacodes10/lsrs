use std::env;
use std::fs::{self, DirEntry};
use std::io;

struct Config{
    show_all: bool,
    // show_list: bool,
    path: String, 
}

fn parse_args(args: &[String]) -> Config {
    let mut show_all = false;
    let mut path = ".".to_string();

    for arg in args.iter().skip(1){
        if arg == "-a" {
            show_all = true;
        } else {
            path = arg.clone();
        }
    }

    Config { show_all, path }
}

fn read_entries(path: &str) -> io::Result<Vec<DirEntry>> {
    fs::read_dir(path)?.collect()
}

fn filter_entries(entries: Vec<DirEntry>, show_all: bool) -> Vec<DirEntry>{
   if show_all {
       return entries;
   } 

   entries.into_iter().filter(|e| {
       e.file_name()
           .to_str()
           .map(|s| !s.starts_with('.'))
           .unwrap_or(true)
   })
   .collect()
}

fn print_entries(entries: &[DirEntry]) {
    for entry in entries {
        print!("{}   ", entry.file_name().to_string_lossy());
    }
    println!("");
}


fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args);

    let entries = read_entries(&config.path)?;
    let entries = filter_entries(entries, config.show_all);

    print_entries(&entries);

    Ok(())

}
