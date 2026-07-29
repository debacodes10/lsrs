use std::env;
use std::fs::{self, DirEntry, Metadata};
use std::io;
use std::time::UNIX_EPOCH;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::ffi::CStr;

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

fn username_from_uid(uid: u32) -> String {
    unsafe {
        let pwd = libc::getpwuid(uid);
        if pwd.is_null() {
            return uid.to_string();
        }
        let name_ptr = (*pwd).pw_name;
        if name_ptr.is_null() {
            return uid.to_string();
        }
        CStr::from_ptr(name_ptr)
            .to_string_lossy()
            .into_owned()
    }
}

fn groupname_from_gid(gid: u32) -> String {
    unsafe {
        let grp = libc::getgrgid(gid);
        if grp.is_null(){
            return gid.to_string();
        }
        let grp_ptr = (*grp).gr_name;
        if grp_ptr.is_null(){
            return gid.to_string();
        }
        CStr::from_ptr(grp_ptr)
            .to_string_lossy()
            .into_owned()
    }
}

fn format_modified(meta: &Metadata) -> String {
    let modified = match meta.modified() {
        Ok(t) => t,
        Err(_) => return "-".to_string(),
    };

    let secs = modified 
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    civil_datetime_from_unix(secs)
}

/// Converts unix seconds -> "DD Mon YYYY HH:MM" using a manual civil-calendar
/// algorithm (Howard Hinnant's days_from_civil, inverted). No chrono needed.
fn civil_datetime_from_unix(unix_secs: i64) -> String {
    let days = unix_secs.div_euclid(86400);
    let secs_of_day = unix_secs.rem_euclid(86400);
    let hour = secs_of_day / 3600;
    let minute = (secs_of_day % 3600) / 60;

    let (year, month, day) = civil_from_days(days);

    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun",
        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    format!(
        "{:02} {} {:04} {:02}:{:02}",
        day,
        MONTHS[(month - 1) as usize],
        year,
        hour,
        minute
    )
}

/// Days since epoch (1970-01-01) -> (year, month, day). Civil calendar algorithm.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let year = if m <= 2 { y + 1 } else { y };
    (year, m, d)
}

fn print_long(entries: &[DirEntry]) -> io::Result<()> {
    println!("Permissions   Hard Links   Owner   Group    Modified   Name");
    for entry in entries{
        let meta = entry.metadata()?;
        let perms = format_permissions(&meta);
        let nlink = meta.nlink();
        let uid = username_from_uid(meta.uid());
        let gid = groupname_from_gid(meta.gid());
        let size = meta.size();
        let modified = format_modified(&meta);
        let name = entry.file_name().to_string_lossy().to_string();
        
        println!("{}   {}   {}   {}   {}   {}   {}", perms, nlink, uid, gid, size, modified, name);
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
