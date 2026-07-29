use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    
    let mut show_all = false;
    let mut path = ".";

    for arg in args.iter().skip(1){
        if arg == "-a"{
            show_all = true;
        } else {
            path = arg;
        }
    }

    let entries = fs::read_dir(path)?;

    let mut entry_iter: Box<dyn Iterator<Item = _>> = Box::new(entries);

    if !show_all{
        entry_iter = Box::new(entry_iter.filter(|entry| {
            entry.as_ref()
                .map(|e| {
                    e.file_name()
                        .to_str()
                        .map(|s| !s.starts_with('.'))
                        .unwrap_or(true)
                })
            .unwrap_or(true)
        }))
    }

    for entry in entry_iter{
        let entry = entry?;
        print!("{}   ", entry.file_name().to_string_lossy());
    }
    println!();
    Ok(())

}
