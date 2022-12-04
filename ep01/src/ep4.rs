fn part1(lines: Vec<String>) -> String {
    let mut count = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(",").collect();

        let r1: Vec<&str> = parts[0].split("-").collect();
        let r11 = r1[0].parse::<i32>().unwrap();
        let r12 = r1[1].parse::<i32>().unwrap();

        let l1: Vec<&str> = parts[1].split("-").collect();
        let l11 = l1[0].parse::<i32>().unwrap();
        let l12 = l1[1].parse::<i32>().unwrap();

        let range1 = r11..=r12;
        let range2 = l11..=l12;

        if (range1.contains(&l11) && range1.contains(&l12))
            || (range2.contains(&r11) && range2.contains(&r12))
        {
            count = count + 1;
        }
    }

    return format!("{}", count);
}

fn part2(lines: Vec<String>) -> String {
    let mut count = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(",").collect();

        let r1: Vec<&str> = parts[0].split("-").collect();
        let r11 = r1[0].parse::<i32>().unwrap();
        let r12 = r1[1].parse::<i32>().unwrap();

        let l1: Vec<&str> = parts[1].split("-").collect();
        let l11 = l1[0].parse::<i32>().unwrap();
        let l12 = l1[1].parse::<i32>().unwrap();

        let range1 = r11..=r12;
        let range2 = l11..=l12;

        if (range1.contains(&l11) || range1.contains(&l12))
            || (range2.contains(&r11) || range2.contains(&r12))
        {
            count = count + 1;
        }
    }

    return format!("{}", count);
}

pub fn exec(lines: Vec<String>) -> String {
    format!("{}\n{}\n", part1(lines.to_owned()), part2(lines.to_owned()))
}
