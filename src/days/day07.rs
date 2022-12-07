use std::fmt;
use crate::days::Day;
use crate::util::number::parse_usize;

pub const DAY7: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let root = parse_terminal_history(input).unwrap();
    let dirs_under_100k = get_directories_under_100k(&root);
    let size_sum = dirs_under_100k.iter().map(|d| d.get_total_size()).sum::<usize>();

    println!("Sum of sizes of dirs < 100k: {}", size_sum);
}
fn puzzle2(input: &String) {
    let root = parse_terminal_history(input).unwrap();

    let disk_size = 70_000_000;
    let free_space_needed = 30_000_000;

    let used_space = root.get_total_size();
    let needed_space = used_space - (disk_size - free_space_needed);

    let all_dirs = root.all_dirs();
    let mut options = all_dirs.iter().filter(|d| d.get_total_size() >= needed_space).collect::<Vec<_>>();
    options.sort_by(|l, r| l.get_total_size().cmp(&r.get_total_size()));

    println!("Smallest dir to remove = {}, size = {}", options[0].name, options[0].get_total_size());
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Directory {
    name: String,
    sub_dirs: Vec<Directory>,
    files: Vec<File>
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory { name, sub_dirs: vec![], files: vec![] }
    }

    fn get_mut(&mut self, path: &[&str]) -> Option<&mut Directory> {
        if path.is_empty() {
            Some(self)
        } else {
            match self.sub_dirs.iter_mut().filter(|d| d.name == path[0]).next() {
                None => None,
                Some(dir) => dir.get_mut(&path[1..])
            }
        }
    }

    fn get_total_size(&self) -> usize {
        self.sub_dirs.iter().map(|d| d.get_total_size()).sum::<usize>() + self.files.iter().map(|f| f.size).sum::<usize>()
    }

    fn all_dirs(&self) -> Vec<&Directory> {
        let all_subdirs = self.sub_dirs.iter().flat_map(|d| d.all_dirs()).collect::<Vec<_>>();
        vec![self].into_iter().chain(all_subdirs.into_iter()).collect()
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn write(dir: &Directory, indent: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(" ".repeat(indent * 2).as_str())?;
            f.write_str("- ")?;
            f.write_str(dir.name.as_str())?;
            f.write_str(" (dir)\n")?;

            for sub_dir in &dir.sub_dirs {
                write(sub_dir, indent + 1, f)?;
            }
            for file in &dir.files {
                f.write_str(" ".repeat((indent + 1) * 2).as_str())?;
                f.write_str("- ")?;
                f.write_str(file.name.as_str())?;
                f.write_str(" (file, size=")?;
                f.write_str(file.size.to_string().as_str())?;
                f.write_str(")\n")?;
            }

            Ok(())
        }

        write(self, 0, f)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct File {
    name: String,
    size: usize
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParserState {
    Ready,
    List
}

fn parse_terminal_history(input: &str) -> Result<Directory, String> {
    let mut root_dir = Directory::new("/".to_string());
    let mut current_path: Vec<&str> = vec![];
    let mut state = ParserState::Ready;

    for line in input.lines() {
        if state == ParserState::List {
            let current = root_dir.get_mut(&current_path).ok_or(format!("Missing directory '{}'", current_path.join("/")))?;

            if line.starts_with("$") {
                state = ParserState::Ready;
            } else if line.starts_with("dir ") {
                let dirname = &line[4..];
                current.sub_dirs.push(Directory::new(dirname.to_string()));
            } else if let [fs, name] = line.split(" ").collect::<Vec<_>>()[..] {
                current.files.push(File { name: name.to_string(), size: parse_usize(fs)? });
            } else {
                return Err(format!("Invalid list line: '{}'", line));
            }
        }

        if state == ParserState::Ready {
            if line == "$ ls" {
                state = ParserState::List;
            } else if line.starts_with("$ cd ") {
                let folder = &line[5..];
                if folder == ".." {
                    if let None = current_path.pop() {
                        return Err(format!("Tried 'cd ..' from root dir"))
                    }
                } else if folder == "/" {
                    current_path.clear();
                } else {
                    current_path.push(folder);
                }
            } else {
                return Err(format!("Expected to read command, but got: '{}'", line));
            }
        }
    }

    Ok(root_dir)
}

fn get_directories_under_100k(root: &Directory) -> Vec<&Directory> {
    let mut result = vec![];

    if root.get_total_size() < 100_000 {
        result.push(root)
    }

    root.sub_dirs.iter().flat_map(|d| get_directories_under_100k(d)).for_each(|d| result.push(d));

    result
}

#[cfg(test)]
mod tests {
    use crate::days::day07::{get_directories_under_100k, parse_terminal_history};

    const TEST_INPUT: &str = "\
        $ cd /\n\
        $ ls\n\
        dir a\n\
        14848514 b.txt\n\
        8504156 c.dat\n\
        dir d\n\
        $ cd a\n\
        $ ls\n\
        dir e\n\
        29116 f\n\
        2557 g\n\
        62596 h.lst\n\
        $ cd e\n\
        $ ls\n\
        584 i\n\
        $ cd ..\n\
        $ cd ..\n\
        $ cd d\n\
        $ ls\n\
        4060174 j\n\
        8033020 d.log\n\
        5626152 d.ext\n\
        7214296 k\n\
        ";

    #[test]
    fn test_parse_history() {
        let result = parse_terminal_history(TEST_INPUT);
        if result.is_err() {
            println!("Failed: {}", result.as_ref().err().unwrap());
        }

        assert!(result.is_ok());

        let root = result.unwrap();
        assert_eq!(vec!["a", "d"], root.sub_dirs.iter().map(|d| d.name.to_string()).collect::<Vec<_>>());

        assert_eq!(vec![
            "- / (dir)",
            "  - a (dir)",
            "    - e (dir)",
            "      - i (file, size=584)",
            "    - f (file, size=29116)",
            "    - g (file, size=2557)",
            "    - h.lst (file, size=62596)",
            "  - d (dir)",
            "    - j (file, size=4060174)",
            "    - d.log (file, size=8033020)",
            "    - d.ext (file, size=5626152)",
            "    - k (file, size=7214296)",
            "  - b.txt (file, size=14848514)",
            "  - c.dat (file, size=8504156)",
        ].join("\n"), format!("{}", root).trim());
    }

    #[test]
    fn test_get_directories_under_100k() {
        let root = parse_terminal_history(TEST_INPUT).unwrap();
        let result = get_directories_under_100k(&root);

        assert_eq!(2, result.len());
        assert_eq!("a", result[0].name);
        assert_eq!(94853, result[0].get_total_size());
        assert_eq!("e", result[1].name);
        assert_eq!(584, result[1].get_total_size());
    }
}