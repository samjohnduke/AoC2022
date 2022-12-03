#[derive(Debug, Clone)]
struct Elf {
    calories: Vec<i32>,
}

impl Elf {
    fn total_cals(&self) -> i32 {
        return self.calories.iter().copied().sum::<i32>();
    }
}

fn gen_elf_vec(lines: Vec<String>) -> Vec<Elf> {
    let mut elves: Vec<Elf> = vec![];

    let mut count = 1;
    let mut current_elf = Elf { calories: vec![] };

    for line in lines {
        if line == "" {
            elves.push(current_elf.clone());
            count = count + 1;
            current_elf = Elf { calories: vec![] };
        } else {
            current_elf.calories.push(line.parse::<i32>().unwrap());
        }
    }

    elves.sort_by(|a, b| a.total_cals().cmp(&b.total_cals()));

    return elves;
}

fn part1(lines: Vec<String>) -> String {
    let mut elves = gen_elf_vec(lines);
    let elf = elves.pop();

    return format!("{}", elf.unwrap().total_cals());
}

fn part2(lines: Vec<String>) -> String {
    let mut elves = gen_elf_vec(lines);
    let elf1 = elves.pop().unwrap().total_cals();
    let elf2 = elves.pop().unwrap().total_cals();
    let elf3 = elves.pop().unwrap().total_cals();

    return format!("{}", elf1 + elf2 + elf3);
}

pub fn exec(lines: Vec<String>) -> String {
    format!("{}\n{}\n", part1(lines.to_owned()), part2(lines.to_owned()))
}
