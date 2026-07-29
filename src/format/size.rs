pub fn format_size(size_in_bytes: u64) -> String {

    const SUFFIXES: [&str; 6] = ["B", "K", "M", "G", "T", "P"];
    let mut value = size_in_bytes as f64;
    let mut unit = 0;

    while value > 1024.0 && unit < SUFFIXES.len() - 1 {
        value /= 1024.0;
        unit+=1;
    }
    
    if unit == 0 {
        format!("{:.0}{}", value, SUFFIXES[unit])
    } else {
        format!("{:.1}{}", value, SUFFIXES[unit])
    }

}

