use std::fs::File;
use std::io::prelude::*;
// use crate::utils::file_to_string;

const UNKNOWN: i64 = 0;
const FILE: i64 = 1;
const VITURL: i64 = 2;

const SELECT: i64 = 1;
const DELETE: i64 = 2;

const STRING: i64 = 1;
const REG: i64 = 2;

pub fn filter_line(
    command: i64,
    pattern_type: i64,
    pattern: &str,
    file_source: i64,
    source_str: &str,
) -> std::io::Result<String> {
    if command == UNKNOWN || pattern_type == UNKNOWN || file_source == UNKNOWN {
        let err_info = format!(
            "command:{}, patternType:{}, file_source:{}",
            command, pattern_type, file_source
        );
        return Ok(err_info);
    }

    let tmp;
    let mut source_str = source_str;
    if file_source == FILE {
        let mut buffer = Vec::new();
        let mut file = File::open(source_str)?;
        file.read_to_end(&mut buffer)?;
        tmp = String::from_utf8_lossy(&buffer).to_string();
        source_str = &tmp;
    }

    if pattern_type == STRING {
        if command == SELECT {
            select_string(source_str, pattern)
        } else {
            return delete_string(source_str, pattern);
        }
    } else {
        Ok(String::from("reg not impl yet"))
    }
}

fn select_string<'a, T>(string: T, pattern: &str) -> std::io::Result<String>
where
    T: Into<&'a str>,
{
    let mut result = String::new();
    string.into().split('\n').for_each(|line| {
        if line.contains(pattern) {
            result.push_str(line);
            result.push_str("\n");
        }
    });
    Ok(result)
}

fn delete_string<'a, T>(string: T, pattern: &str) -> std::io::Result<String>
where
    T: Into<&'a str>,
{
    let mut result = String::new();
    string.into().split('\n').for_each(|line| {
        if !line.contains(pattern) {
            result.push_str(line);
            result.push_str("\n");
        } else {
            println!("shit {}", line);
        }
    });
    Ok(result)
}

mod tests {
    use crate::utils::*;
    #[test]
    fn filter_01() {
        let file = "/Users/lizhen/Desktop/LOG/lark.mmap2.12.xlog.log";
        if let Ok(filtered) = crate::log_filter::filter_line(1, 1, "/var/mobile/Containers/Data/Application/F367B2C5-2E", 1, file) {
            println!("\"\n{}\n\"", filtered);
        }
    }
}
