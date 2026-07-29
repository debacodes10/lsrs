mod cli;
mod fs_ops;
mod format;
mod output;

use std::env;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = cli::parse_args(&args);

    let entries = fs_ops::read_entries(&config.path)?;
    let entries = fs_ops::filter_entries(entries, config.show_all);

    if config.long {
        output::print_long(&entries,config.human)?;
    } else {
        output::print_entries(&entries);
    }

    Ok(())
}
