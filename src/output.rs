use std::fs::DirEntry;
use std::io;
use std::os::unix::fs::MetadataExt;

use crate::format::{format_modified, format_permissions, groupname_from_gid, username_from_uid};

pub fn print_entries(entries: &[DirEntry]) {
    for entry in entries {
        print!("{}   ", entry.file_name().to_string_lossy());
    }
    println!();
}

pub fn print_long(entries: &[DirEntry]) -> io::Result<()> {
    println!("Permissions   Hard Links   Owner   Group    Modified   Name");
    for entry in entries {
        let meta = entry.metadata()?;
        let perms = format_permissions(&meta);
        let nlink = meta.nlink();
        let owner = username_from_uid(meta.uid());
        let group = groupname_from_gid(meta.gid());
        let size = meta.size();
        let modified = format_modified(&meta);
        let name = entry.file_name().to_string_lossy().to_string();

        println!(
            "{}   {}   {}   {}   {}   {}   {}",
            perms, nlink, owner, group, size, modified, name
        );
    }
    Ok(())
}
