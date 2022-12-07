use crate::{ParseError, SolveError};

#[derive(Debug, PartialEq)]
pub struct Dir {
    name: String,
    files: Vec<usize>,
    size: usize,
    children: Vec<Dir>,
}

impl Dir {
    fn new(name: String) -> Self {
        Dir {
            name,
            files: vec![],
            size: 0,
            children: vec![],
        }
    }

    fn add_dir(&mut self, dir: Dir) {
        let child_size = dir.size;
        self.children.push(dir);
        self.size += child_size;
    }

    fn add_file(&mut self, size: usize) {
        self.files.push(size);
        self.size += size;
    }

    fn get_dir_sizes(&self) -> Vec<usize> {
        self.children
            .iter()
            .fold(vec![self.size], |mut acc, child| {
                acc.extend(child.get_dir_sizes().into_iter());
                acc
            })
    }
}

pub struct Solver {}

enum Command {
    Cd(String),
    Up,
    Skip,
    AddFile(usize),
}

impl crate::Solver for Solver {
    type Input = Dir;
    type Output = usize;
    const DAY: u8 = 7;

    fn parse(input: String) -> Result<Self::Input, ParseError> {
        let mut commands = input.lines().map(|line| parse_line(line));

        if let Command::Cd(root) = commands.next().ok_or(ParseError::Invalid)?? {
            explore_dir(root, &mut commands)
        } else {
            Err(ParseError::Invalid)
        }
    }

    fn part_1(input: Self::Input) -> Result<Self::Output, SolveError> {
        Ok(input
            .get_dir_sizes()
            .into_iter()
            .filter(|size| *size < 100_000)
            .sum())
    }

    fn part_2(input: Self::Input) -> Result<Self::Output, SolveError> {
        const TOTAL_SPACE: usize = 70000000;
        const REQUIRED_SPACE: usize = 30000000;
        let used_space = input.size;
        let unused_space = TOTAL_SPACE - used_space;
        let needed_space = REQUIRED_SPACE - unused_space;

        input
            .get_dir_sizes()
            .into_iter()
            .filter(|size| *size > needed_space)
            .min()
            .ok_or(SolveError::InvalidInput)
    }
}

fn parse_line(line: &str) -> Result<Command, ParseError> {
    if line.starts_with("$ cd ..") {
        Ok(Command::Up)
    } else if line.starts_with("$ cd") {
        Ok(Command::Cd(line[5..].to_owned()))
    } else if line.starts_with("$ ls") {
        Ok(Command::Skip)
    } else if line.starts_with("dir") {
        Ok(Command::Skip)
    } else if let Ok(size) = line[..line.find(" ").ok_or(ParseError::Invalid)?].parse() {
        Ok(Command::AddFile(size))
    } else {
        Err(ParseError::Invalid)
    }
}

fn explore_dir(
    name: String,
    commands: &mut impl Iterator<Item = Result<Command, ParseError>>,
) -> Result<Dir, ParseError> {
    let mut dir = Dir::new(name);

    while let Some(command) = commands.by_ref().next() {
        match command? {
            Command::Cd(child) => dir.add_dir(explore_dir(child, commands)?),
            Command::AddFile(size) => dir.add_file(size),
            Command::Up => break,
            Command::Skip => (),
        }
    }

    Ok(dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Solver;

    fn get_input() -> Dir {
        let mut root = Dir::new(String::from("/"));
        root.add_file(14848514);
        root.add_file(8504156);

        let mut a = Dir::new(String::from("a"));
        a.add_file(29116);
        a.add_file(2557);
        a.add_file(62596);

        let mut d = Dir::new(String::from("d"));
        d.add_file(4060174);
        d.add_file(8033020);
        d.add_file(5626152);
        d.add_file(7214296);

        let mut e = Dir::new(String::from("e"));
        e.add_file(584);

        a.add_dir(e);
        root.add_dir(a);
        root.add_dir(d);

        root
    }

    #[test]
    fn parsing() {
        let input = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(
            super::Solver::parse(String::from(input)).unwrap(),
            get_input()
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(super::Solver::part_1(get_input()).unwrap(), 95437);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::Solver::part_2(get_input()).unwrap(), 24933642);
    }
}
