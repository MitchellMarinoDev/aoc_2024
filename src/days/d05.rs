// https://adventofcode.com/2022/day/5

pub fn solve(input: String) -> (String, String) {
    // Parse
    let mut split = input.split("\n\n");
    let rules = split.next().unwrap();
    let updates = split.next().unwrap();
    assert_eq!(split.next(), None);

    let rules: Vec<_> = rules
        .lines()
        .map(|l| {
            let mut split = l.split("|").map(|v| v.parse::<u32>().unwrap());
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect();

    let updates: Vec<Vec<u32>> = updates
        .lines()
        .map(|l| l.split(',').map(|v| v.parse::<u32>().unwrap()).collect())
        .collect();

    // sort the updates into correct and incorrect updates
    let mut correct = vec![];
    let mut incorrect = vec![];

    for update in updates.into_iter() {
        if check_rules(&update, &rules) {
            correct.push(update);
        } else {
            incorrect.push(update);
        }
    }

    // Part 1
    let mut sum1 = 0;
    for update in correct.into_iter() {
        sum1 += update[update.len() / 2];
    }

    // Part 2
    let mut sum2 = 0;
    for mut update in incorrect.into_iter() {
        reorder_update(&mut update, &rules);
        sum2 += update[update.len() / 2];
    }

    (sum1.to_string(), sum2.to_string())
}

fn check_rules(update: &[u32], rules: &[(u32, u32)]) -> bool {
    for (x, y) in rules.iter().copied() {
        let xi = update.iter().position(|v| *v == x);
        let yi = update.iter().position(|v| *v == y);
        let (Some(xi), Some(yi)) = (xi, yi) else {
            continue;
        };
        if xi > yi {
            return false;
        }
    }
    true
}

/// Reorders `update` to follow all the rules.
fn reorder_update(update: &mut Vec<u32>, rules: &[(u32, u32)]) {
    for i in 0..update.len() {
        // Anytime an element is incorrect, things get shifted around on the right.
        // So, Everything to the left of i should be correct.
        // So, we will keep doing these shifts that change the elements >= `i`
        //   until the value at index `i` upholds all the rules.
        while !fix_rules_at_element(update, rules, i) {}
    }
}

/// This checks all the rules pertaining to element at index `i`.
///
/// If any rule is volated, elements at indexs >= `i` are moved around to fix it
/// and `false` is returned. If all rules are upheld, nothing is moved around and
/// `true` is returned.
fn fix_rules_at_element(update: &mut Vec<u32>, rules: &[(u32, u32)], i: usize) -> bool {
    let element = update[i];

    for (x, y) in rules.iter().copied() {
        if x == element {
            // ensure element `y` is after this element.
            let Some(yi) = update.iter().position(|v| *v == y) else {
                // y does not exist
                continue;
            };

            if yi < i {
                // move y to be after this element
                let y_element = update.remove(yi);
                update.insert(i - 1, y_element);
                return false;
            }
        } else if y == element {
            // ensure the element `x` is before this element.

            let Some(xi) = update.iter().position(|v| *v == x) else {
                // x does not exist
                continue;
            };

            if xi > i {
                // move this value to be after `x`
                let element = update.remove(i);
                update.insert(xi, element);
                return false;
            }
        }
    }

    // all rules passed
    true
}
