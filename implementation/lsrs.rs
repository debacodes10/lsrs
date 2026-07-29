use std::env;
use std::fs::{self, DirEntry, Metadata};
use std::io;
use std::time::UNIX_EPOCH;
use std::os::unix::fs::{MetadataExt, PermissionsExt};

struct Config{
    show_all: bool,
    long: bool,
    path: String, 
}

fn parse_args(args: &[String]) -> Config {
    let mut show_all = false;
    let mut path = ".".to_string();
    let mut long = false;

    for arg in args.iter().skip(1){
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

fn format_permissions(meta: &Metadata) -> String {
    let mode = meta.permissions().mode();
    let file_type = if meta.is_dir() { 'd' } else { '-' };

    let perms = ["r", "w", "x"];
    let mut s = String::new();
    s.push(file_type);

    for shift in [6,3,0]{
        for (i,p) in perms.iter().enumerate() {
            let bit = 1 << (2-i);
            if (mode >> shift) & bit != 0 {
                s.push_str(p);
            } else {
                s.push('-');
            }
        }
    }
    s
}

fn format_modified(meta: &Metadata) -> String {
    match meta.modified() {
        Ok(time) => {
            let secs = time
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            format!("{}", secs)
        }
        Err(_) => "-".to_string(),
    }
}

fn print_long(entries: &[DirEntry]) -> io::Result<()> {
    for entry in entries{
        let meta = entry.metadata()?;
        let perms = format_permissions(&meta);
        let size = meta.size();
        let modified = format_modified(&meta);
        let name = entry.file_name().to_string_lossy().to_string();

        println!("{:<11}   {:>8}   {:>12}   {}", perms, size, modified, name);
    }
    Ok(())
}


fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args);

    let entries = read_entries(&config.path)?;
    let entries = filter_entries(entries, config.show_all);

    if config.long{
        print_long(&entries)?;
    } else {
        print_entries(&entries);
    }

    Ok(())

}
