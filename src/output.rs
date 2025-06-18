use colored::Colorize;

use crate::check_result::{CheckResult, Kind, Record};

pub fn print_check_result(domain: &str, result: &CheckResult, verbose: bool) {
    println!("Domain: {}", domain.cyan());
    print!("Status: ");

    if result.success {
        println!("{}", "success".green());
    } else {
        println!("{}", "fail".red());
    }

    if result.success && !verbose {
        return;
    }

    result.records.iter().for_each(|record| {
        println!();
        print_record(record);
    });
}

fn print_record(record: &Record) {
    println!("{} {}", record.kind.as_str().bold(), record.name.cyan());
    print!("Status:");

    if record.passed {
        println!(" {}", "passed".green());
    } else {
        println!(" {}", "failed".red());
    }

    if record.incomplete {
        println!("{}", "incomplete".yellow());
    }

    if record.passed && record.legacy {
        println!("{}", "deprecated: use CNAME instead".yellow());
    }

    print_diff(record);
}

fn print_diff(record: &Record) {
    if record.legacy && record.kind == Kind::CNAME {
        return;
    }

    println!("Values:");
    let mut actual: Vec<String> = record.actual.iter().map(|s| s.to_string()).collect();
    record.expected.iter().for_each(|exp_val| {
        let find_res = actual.iter().position(|a| is_matching_value(exp_val, a));
        match find_res {
            Some(idx) => {
                let val = actual[idx].clone();
                println!("  {}: ok", val.green());
                actual.remove(idx);
            }
            None if record.kind == Kind::AAAA => {
                println!("  {}: not found", exp_val.magenta());
            }
            None => {
                println!("  {}: not found", exp_val.red());
            }
        }
    });

    for a in actual {
        println!("  {}: unexpected", a.red());
    }
}

fn is_matching_value(expected: &str, actual: &str) -> bool {
    if actual.starts_with("*") {
        return false;
    }

    if expected.contains('*') {
        let expected_parts: Vec<&str> = expected.split('*').collect();
        return actual.ends_with(expected_parts[1]);
    }

    expected == actual
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_matching_value() {
        assert!(is_matching_value("*.example.com", "sub.example.com"));
        assert!(!is_matching_value("*.example.com", "example.com"));
        assert!(is_matching_value("exact.com", "exact.com"));
        assert!(!is_matching_value("exact.com", "different.com"));
        assert!(!is_matching_value("*.example.com", "*.example.com"));
        assert!(is_matching_value("*.example.com", "foo.bar.example.com"));
    }
}
