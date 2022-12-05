fn part1(lines: Vec<String>) -> String {
    let mut stacks: Vec<Vec<&str>> = vec![
        vec!["W", "M", "L", "F"],
        vec!["B", "Z", "V", "M", "F"],
        vec!["H", "V", "R", "S", "L", "Q"],
        vec!["F", "S", "V", "Q", "P", "M", "T", "J"],
        vec!["W", "S", "L"],
        vec!["F", "V", "P", "M", "R", "J", "W"],
        vec!["J", "Q", "C", "P", "N", "R", "F"],
        vec!["V", "H", "P", "S", "Z", "W", "R", "B"],
        vec!["B", "M", "J", "C", "G", "H", "Z", "W"],
    ];

    // let mut stacks: Vec<Vec<&str>> = vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]];

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let count = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap();
        let to = parts[5].parse::<usize>().unwrap();

        let stack1 = &mut stacks[from - 1];
        let mut temp_stack: Vec<&str> = stack1.drain((stack1.len() - count)..).collect();
        temp_stack.reverse();

        // print!("{:?}, {:?}, ", stack1, temp_stack);

        let stack2 = &mut stacks[to - 1];

        stack2.append(&mut temp_stack);

        // println!("{:?}, {}", stack2, count);
    }

    let mut items: Vec<&str> = vec![];

    for mut stack in stacks {
        items.push(&mut stack.pop().unwrap());
    }

    return format!("{}", items.join(""));
}

fn part2(lines: Vec<String>) -> String {
    let mut stacks: Vec<Vec<&str>> = vec![
        vec!["W", "M", "L", "F"],
        vec!["B", "Z", "V", "M", "F"],
        vec!["H", "V", "R", "S", "L", "Q"],
        vec!["F", "S", "V", "Q", "P", "M", "T", "J"],
        vec!["L", "S", "W"],
        vec!["F", "V", "P", "M", "R", "J", "W"],
        vec!["J", "Q", "C", "P", "N", "R", "F"],
        vec!["V", "H", "P", "S", "Z", "W", "R", "B"],
        vec!["B", "M", "J", "C", "G", "H", "Z", "W"],
    ];

    // let mut stacks: Vec<Vec<&str>> = vec![vec!["Z", "N"], vec!["M", "C", "D"], vec!["P"]];

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let count = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap();
        let to = parts[5].parse::<usize>().unwrap();

        let stack1 = &mut stacks[from - 1];
        let mut temp_stack: Vec<&str> = stack1.drain((stack1.len() - count)..).collect();
        // temp_stack.reverse();

        print!("{:?}, {:?}, ", stack1, temp_stack);

        let stack2 = &mut stacks[to - 1];

        stack2.append(&mut temp_stack);

        println!("{:?}, {}", stack2, count);
    }

    let mut items: Vec<&str> = vec![];

    for mut stack in stacks {
        items.push(&mut stack.pop().unwrap());
    }

    return format!("{}", items.join(""));
}

pub fn exec(lines: Vec<String>) -> String {
    format!("{}\n{}\n", part1(lines.to_owned()), part2(lines.to_owned()))
}
