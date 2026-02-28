use std::time::Duration;

use color_eyre::eyre::{Result, bail};
use polyoxide_data::types::ActivityType;

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

pub fn parse_activity_types(input: &str) -> Result<Vec<ActivityType>> {
    let mut valid = Vec::new();
    let mut invalid = Vec::new();

    for s in input.split(',') {
        let trimmed = s.trim();
        match trimmed.to_uppercase().as_str() {
            "TRADE" => valid.push(ActivityType::Trade),
            "SPLIT" => valid.push(ActivityType::Split),
            "MERGE" => valid.push(ActivityType::Merge),
            "REDEEM" => valid.push(ActivityType::Redeem),
            "REWARD" => valid.push(ActivityType::Reward),
            "CONVERSION" => valid.push(ActivityType::Conversion),
            _ => invalid.push(trimmed.to_string()),
        }
    }

    if !invalid.is_empty() {
        bail!(
            "Invalid activity type(s): {}. Valid types: trade, split, merge, redeem, reward, conversion",
            invalid.join(", ")
        );
    }

    Ok(valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_activity_types_valid_single() {
        let result = parse_activity_types("trade").unwrap();
        assert_eq!(result, vec![ActivityType::Trade]);
    }

    #[test]
    fn parse_activity_types_valid_multiple() {
        let result = parse_activity_types("trade,split,merge").unwrap();
        assert_eq!(
            result,
            vec![ActivityType::Trade, ActivityType::Split, ActivityType::Merge]
        );
    }

    #[test]
    fn parse_activity_types_case_insensitive() {
        let result = parse_activity_types("Trade,SPLIT,rEdEeM").unwrap();
        assert_eq!(
            result,
            vec![ActivityType::Trade, ActivityType::Split, ActivityType::Redeem]
        );
    }

    #[test]
    fn parse_activity_types_trims_whitespace() {
        let result = parse_activity_types(" trade , split ").unwrap();
        assert_eq!(result, vec![ActivityType::Trade, ActivityType::Split]);
    }

    #[test]
    fn parse_activity_types_rejects_invalid() {
        let err = parse_activity_types("trade,invalid,split,typo").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("invalid"), "error should list 'invalid': {msg}");
        assert!(msg.contains("typo"), "error should list 'typo': {msg}");
    }

    #[test]
    fn parse_activity_types_all_invalid() {
        let err = parse_activity_types("foo,bar").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("foo"), "error should list 'foo': {msg}");
        assert!(msg.contains("bar"), "error should list 'bar': {msg}");
    }

    #[test]
    fn parse_activity_types_all_variants() {
        let result =
            parse_activity_types("trade,split,merge,redeem,reward,conversion").unwrap();
        assert_eq!(result.len(), 6);
    }
}
