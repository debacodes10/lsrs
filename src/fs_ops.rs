use std::fs::{self, DirEntry};
use std::io;

pub fn read_entries(path: &str) -> io::Result<Vec<DirEntry>> {
    fs::read_dir(path)?.collect()
}

pub fn filter_entries(entries: Vec<DirEntry>, show_all: bool) -> Vec<DirEntry> {
    if show_all {
        return entries;
    }

    entries
        .into_iter()
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|s| !s.starts_with('.'))
                .unwrap_or(true)
        })
        .collect()
}
