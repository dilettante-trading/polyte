use std::time::Duration;

/// Parse comma-separated values into a Vec of trimmed strings.
/// Used as a clap value_parser for arguments that accept multiple IDs.
pub fn parse_comma_separated(s: &str) -> Result<Vec<String>, std::convert::Infallible> {
    if s.is_empty() {
        return Ok(Vec::new());
    }
    let strings = s.split(',').map(|s| s.trim().to_string()).collect();
    Ok(strings)
}

pub fn parse_duration(s: &str) -> Result<Duration, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("empty duration".to_string());
    }

    let (num, unit) = if let Some(n) = s.strip_suffix("ms") {
        (n, "ms")
    } else if let Some(n) = s.strip_suffix('s') {
        (n, "s")
    } else if let Some(n) = s.strip_suffix('m') {
        (n, "m")
    } else if let Some(n) = s.strip_suffix('h') {
        (n, "h")
    } else {
        // Default to seconds if no unit
        (s, "s")
    };

    let num: u64 = num
        .parse()
        .map_err(|_| format!("invalid number: {}", num))?;

    match unit {
        "ms" => Ok(Duration::from_millis(num)),
        "s" => Ok(Duration::from_secs(num)),
        "m" => Ok(Duration::from_secs(num * 60)),
        "h" => Ok(Duration::from_secs(num * 3600)),
        _ => Err(format!("unknown unit: {}", unit)),
    }
}
