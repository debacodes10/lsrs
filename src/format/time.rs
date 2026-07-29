use std::fs::Metadata;
use std::time::UNIX_EPOCH;

pub fn format_modified(meta: &Metadata) -> String {
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
