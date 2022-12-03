use std::str::FromStr;

pub fn exec(lines: Vec<String>) -> String {
    format!("{}\n{}", part1(lines.to_owned()), part2(lines.to_owned()))
}

#[derive(Debug)]
enum ElfHand {
    A,
    B,
    C,
}

impl FromStr for ElfHand {
    type Err = ();

    fn from_str(input: &str) -> Result<ElfHand, Self::Err> {
        match input {
            "A" => Ok(ElfHand::A),
            "B" => Ok(ElfHand::B),
            "C" => Ok(ElfHand::C),
            _ => Err(()),
        }
    }
}

impl ElfHand {
    fn score(&self, you: &YouHand) -> i32 {
        match self {
            Self::A => match you {
                YouHand::X => 3,
                YouHand::Y => 6,
                YouHand::Z => 0,
            },
            Self::B => match you {
                YouHand::X => 0,
                YouHand::Y => 3,
                YouHand::Z => 6,
            },
            Self::C => match you {
                YouHand::X => 6,
                YouHand::Y => 0,
                YouHand::Z => 3,
            },
        }
    }

    fn choice(&self, you: &YouHand) -> YouHand {
        match self {
            Self::A => match you {
                YouHand::X => YouHand::Z,
                YouHand::Y => YouHand::X,
                YouHand::Z => YouHand::Y,
            },
            Self::B => match you {
                YouHand::X => YouHand::X,
                YouHand::Y => YouHand::Y,
                YouHand::Z => YouHand::Z,
            },
            Self::C => match you {
                YouHand::X => YouHand::Y,
                YouHand::Y => YouHand::Z,
                YouHand::Z => YouHand::X,
            },
        }
    }
}

#[derive(Debug)]
enum YouHand {
    X,
    Y,
    Z,
}

impl FromStr for YouHand {
    type Err = ();

    fn from_str(input: &str) -> Result<YouHand, Self::Err> {
        match input {
            "X" => Ok(YouHand::X),
            "Y" => Ok(YouHand::Y),
            "Z" => Ok(YouHand::Z),
            _ => Err(()),
        }
    }
}

impl YouHand {
    fn score(&self) -> i32 {
        match self {
            Self::X => 1,
            Self::Y => 2,
            Self::Z => 3,
        }
    }
}

struct Hand {
    elf: ElfHand,
    you: YouHand,
}

impl Hand {
    fn score(&self) -> i32 {
        let result = self.elf.score(&self.you);
        let val = self.you.score();
        return result + val;
    }
}

fn part2(lines: Vec<String>) -> String {
    let mut score = 0;
    for line in lines {
        let g: Vec<&str> = line.split(" ").collect();

        let elf = ElfHand::from_str(g[0]).unwrap();
        let you = YouHand::from_str(g[1]).unwrap();
        let choice = elf.choice(&you);

        let hand = Hand {
            elf: elf,
            you: choice,
        };

        score = score + hand.score();
    }
    return score.to_string();
}

fn part1(lines: Vec<String>) -> String {
    let mut score = 0;
    for line in lines {
        let g: Vec<&str> = line.split(" ").collect();

        let elf = ElfHand::from_str(g[0]).unwrap();
        let you = YouHand::from_str(g[1]).unwrap();

        let hand = Hand { elf: elf, you: you };

        score = score + hand.score();
    }
    return score.to_string();
}
