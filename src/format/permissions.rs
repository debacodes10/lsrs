use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;

pub fn format_permissions(meta: &Metadata) -> String {
    let mode = meta.permissions().mode();
    let file_type = if meta.is_dir() { 'd' } else { '-' };

    let perms = ["r", "w", "x"];
    let mut s = String::new();
    s.push(file_type);

    for shift in [6, 3, 0] {
        for (i, p) in perms.iter().enumerate() {
            let bit = 1 << (2 - i);
            if (mode >> shift) & bit != 0 {
                s.push_str(p);
            } else {
                s.push('-');
            }
        }
    }
    s
}
