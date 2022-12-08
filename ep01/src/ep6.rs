use std::collections::VecDeque;

fn part1(lines: Vec<String>) -> String {
    let l = lines[0].as_str();

    let mut test: VecDeque<char> = VecDeque::new();
    let mut count = 0;

    for i in l.chars() {
        if test.len() == 4 {
            let mut n: Vec<char> = test.clone().drain(..).collect();
            n.sort_unstable();
            n.dedup();
            if n.len() == 4 {
                break;
            }
            test.pop_front();
        }

        test.push_back(i);

        count = count + 1;
    }
    return format!("{}", count);
}

fn part2(lines: Vec<String>) -> String {
    let l = lines[0].as_str();

    let mut test: VecDeque<char> = VecDeque::new();
    let mut count = 0;

    for i in l.chars() {
        if test.len() == 14 {
            println!("{:?}, {}", test, i);
            let mut n: Vec<char> = test.clone().drain(..).collect();
            println!("{:?}", n);
            n.sort_unstable();
            n.dedup();
            if n.len() == 14 {
                break;
            }
            test.pop_front();
        }

        test.push_back(i);

        count = count + 1;
    }
    return format!("{}", count);
}

pub fn exec(lines: Vec<String>) -> String {
    format!("{}\n{}\n", part1(lines.to_owned()), part2(lines.to_owned()))
}
