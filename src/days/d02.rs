// https://adventofcode.com/2022/day/2

pub fn solve(input: String) -> (String, String) {
    // Parse
    let reports = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Part 1
    let count1 = reports.iter().filter(|&r| check_report(r)).count();

    // Part 2
    let count2 = reports.iter().filter(|&r| check_report_dampened(r)).count();

    (count1.to_string(), count2.to_string())
}

/// Checks a report for part 1.
fn check_report(report: &Vec<u32>) -> bool {
    let should_be_decreasing = report[1] < report[0];
    let report_iter = report.iter().zip(report.iter().skip(1));
    for (&level, &next_level) in report_iter {
        let is_decreasing = next_level < level;
        let is_acceptable_difference = (1..=3).contains(&next_level.abs_diff(level));

        if (is_decreasing ^ should_be_decreasing) || !is_acceptable_difference {
            return false;
        }
    }
    true
}

/// Checks a report for part 2 that uses the "problem dampener" to remove
/// a single level from the report.
fn check_report_dampened(report: &Vec<u32>) -> bool {
    // If the numbers decrease twice, the entire report is either decreasing, or is unsafe.
    let decreasing_count = (report[1] < report[0]) as u32
        + (report[2] < report[1]) as u32
        + (report[3] < report[2]) as u32;
    let should_be_decreasing = decreasing_count >= 2;

    let report_iter = report.iter().zip(report.iter().skip(1)).enumerate();
    for (index, (&level, &next_level)) in report_iter {
        let is_decreasing = next_level < level;
        let is_acceptable_difference = (1..=3).contains(&next_level.abs_diff(level));
        if (is_decreasing ^ should_be_decreasing) || !is_acceptable_difference {
            // we need to drop this level or the next level
            let mut report1 = report.clone();
            report1.remove(index);
            let mut report2 = report.clone();
            report2.remove(index + 1);

            return check_report(&report1) || check_report(&report2);
        }
    }

    true
}
