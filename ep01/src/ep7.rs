use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::format;

enum Cmd {
    LS,
    CD,
}

struct Act {
    cmd: Cmd,
    path: Option<String>,
}

#[derive(Debug)]
struct File {
    size: i32,
    name: String,
}

#[derive(Debug)]
struct Directory {
    name: String,
}
#[derive(Debug)]
enum Node {
    FILE(File),
    DIR(Directory),
}

fn part1(lines: Vec<String>) -> String {
    let mut count = 0;

    let mut tree: HashMap<String, Node> = HashMap::new();
    let mut pwd: String = "".to_owned();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                if parts[2] == "/" {
                    pwd = "".to_owned();
                } else if parts[2] == ".." {
                    let mut t1 = pwd.split("/").collect::<Vec<&str>>();
                    t1.pop();
                    pwd = t1.join("/")
                } else {
                    pwd = format!("{}/{}", pwd, parts[2])
                }
            }
        } else if parts[0] == "dir" {
            let d = Directory {
                name: parts[1].to_owned(),
            };

            tree.insert(format!("{}/{}/", pwd, d.name), Node::DIR(d));
        } else {
            let f = File {
                name: parts[1].to_owned(),
                size: parts[0].parse::<i32>().unwrap(),
            };

            tree.insert(format!("{}/{}", pwd, f.name), Node::FILE(f));
        }
    }

    print!("{:?}", tree);

    return format!("{}", count);
}

fn part2(lines: Vec<String>) -> String {
    let mut count = 0;

    return format!("{}", count);
}

pub fn exec(lines: Vec<String>) -> String {
    format!("{}\n{}\n", part1(lines.to_owned()), part2(lines.to_owned()))
}
