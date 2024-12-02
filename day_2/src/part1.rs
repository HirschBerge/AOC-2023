use std::process::exit;

use crate::{nom_into_nums, SafetyLevel, UpOrDown};

pub fn check_all_safe_p1(all_reports: String) -> i32 {
    let (_, reports) = match nom_into_nums(all_reports.as_str()) {
        Ok(numbas) => numbas,
        Err(e) => {
            eprintln!("Invalid data!! {}", e);
            exit(1);
        }
    };
    let mut safe_reports = 0;
    reports.into_iter().for_each(|report| {
        let status = check_safe(report);
        if status == SafetyLevel::Safe {
            safe_reports += 1;
        }
    });
    safe_reports
}

fn check_safe(report: Vec<i32>) -> SafetyLevel {
    let mut safety = SafetyLevel::Safe;
    let mut status = UpOrDown::Basis;
    for number in report.windows(2) {
        let (a, b) = (number[0], number[1]);
        #[allow(unused_assignments)]
        let mut this_change = UpOrDown::Basis;
        if a > b {
            this_change = UpOrDown::Lowering;
        } else {
            this_change = UpOrDown::Raising;
        }
        if this_change != status && status != UpOrDown::Basis {
            safety.set_unsafe();
        } else {
            status = this_change;
        }
        let difference = (a - b).abs();
        if !(1..=3).contains(&difference) {
            safety.set_unsafe();
            return safety;
        }
    }
    safety
}

#[cfg(test)]
mod tests {
    use part1::check_all_safe_p1;

    use crate::*;

    #[test]
    fn test_safeness_p2() {
        let data = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string();
        let safeness = check_all_safe_p1(data);
        assert_eq!(safeness, 2);
    }
}
