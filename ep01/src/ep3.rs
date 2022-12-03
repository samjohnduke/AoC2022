use std::vec;

fn part1(lines: Vec<String>) -> String {
    let mut total: i32 = 0;
    for line in lines {
        let items = line.as_bytes();
        let comp_size = items.len();
        let (left, right) = items.split_at(comp_size / 2);
        let mut dup: u8 = 0;
        let mut score: u8 = 0;
        for item in right {
            if left.contains(item) {
                dup = *item;
            }
        }
        if dup >= 97 {
            score = dup - 97 + 1;
        } else {
            score = dup - 65 + 26 + 1;
        }
        total = total + score as i32;
    }
    return total.to_string();
}

fn part2(lines: Vec<String>) -> String {
    let mut total: i32 = 0;
    let mut groups: Vec<Vec<String>> = vec![];

    let mut acc = 0;
    let mut vec_acc: Vec<String> = vec![];
    for line in lines {
        vec_acc.push(line);
        acc = acc + 1;

        if acc == 3 {
            groups.push(vec_acc);
            vec_acc = vec![];
            acc = 0
        }
    }

    for group in groups {
        let one = group[0].as_bytes();
        let two = group[1].as_bytes();
        let three = group[2].as_bytes();
        let mut dup: u8 = 0;
        let mut score: u8 = 0;

        for item in three {
            if one.contains(item) && two.contains(item) {
                dup = *item;
            }
        }

        if dup >= 97 {
            score = dup - 97 + 1;
        } else {
            score = dup - 65 + 26 + 1;
        }
        total = total + score as i32;
    }

    return total.to_string();
}

pub fn exec(lines: Vec<String>) -> String {
    format!("{}\n{}\n", part1(lines.to_owned()), part2(lines.to_owned()))
}
