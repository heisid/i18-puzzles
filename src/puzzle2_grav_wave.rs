use std::fs;
use chrono::{DateTime, Utc};


fn get_duplicates(input: &str) -> Option<String> {
    let mut duplicates: Vec<(u8, DateTime<Utc>)> = Vec::new();
    let mut result: Option<String> = None;
    'line_loop: for line in input.lines() {
        let parsed: Result<DateTime<Utc>, _> = line.trim().parse();
        if let Ok(parsed) = parsed {
            let mut found = false;
            for (count, dup) in &mut duplicates {
                if parsed == *dup {
                    *count += 1;
                    found = true;
                    if *count >= 4 {
                        result = Some(dup.to_rfc3339());
                        break 'line_loop;
                    }
                }
            }
            if !found {
                duplicates.push((1, parsed));
            }
        } else {
            continue;
        }
    }
    result
}

pub fn get_time_gravitational_wave_detected() {
    let file_path = "input2.txt";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    if let Some(duplicate) = get_duplicates(&contents) {
        println!("Result Puzzle 2: {}", duplicate);
    } else {
        println!("Result Puzzle 2: No duplicates found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_duplicates() {
        let input = r#"
        2019-06-05T08:15:00-04:00
        2019-06-05T14:15:00+02:00
        2019-06-05T17:45:00+05:30
        2019-06-05T05:15:00-07:00
        2011-02-01T09:15:00-03:00
        2011-02-01T09:15:00-05:00"#;

        let result = get_duplicates(input);
        if let Some(result) = result {
            assert_eq!(result, "2019-06-05T12:15:00+00:00");
        }
    }
}