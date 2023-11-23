/*
--- Day 7: No Space Left On Device ---
You can hear birds chirping and raindrops hitting leaves as the expedition proceeds. Occasionally, you can even hear much louder sounds in the distance; how big do the animals get out here, anyway?

The device the Elves gave you has problems with more than just its communication system. You try to run a system update:

$ system-update --please --pretty-please-with-sugar-on-top
Error: No space left on device
Perhaps you can delete some files to make space for the update?

You browse around the filesystem to assess the situation and save the resulting terminal output (your puzzle input). For example:

$ cd /
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
7214296 k
The filesystem consists of a tree of files (plain data) and directories (which can contain other directories or files). The outermost directory is called /. You can navigate around the filesystem, moving into or out of directories and listing the contents of the directory you're currently in.

Within the terminal output, lines that begin with $ are commands you executed, very much like some modern computers:

cd means change directory. This changes which directory is the current directory, but the specific result depends on the argument:
cd x moves in one level: it looks in the current directory for the directory named x and makes it the current directory.
cd .. moves out one level: it finds the directory that contains the current directory, then makes that directory the current directory.
cd / switches the current directory to the outermost directory, /.
ls means list. It prints out all of the files and directories immediately contained by the current directory:
123 abc means that the current directory contains a file named abc with size 123.
dir xyz means that the current directory contains a directory named xyz.
Given the commands and output in the example above, you can determine that the filesystem looks visually like this:

- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)
Here, there are four directories: / (the outermost directory), a and d (which are in /), and e (which is in a). These directories also contain files of various sizes.

Since the disk is full, your first step should probably be to find directories that are good candidates for deletion. To do this, you need to determine the total size of each directory. The total size of a directory is the sum of the sizes of the files it contains, directly or indirectly. (Directories themselves do not count as having any intrinsic size.)

The total sizes of the directories above can be found as follows:

The total size of directory e is 584 because it contains a single file i of size 584 and no other directories.
The directory a has total size 94853 because it contains files f (size 29116), g (size 2557), and h.lst (size 62596), plus file i indirectly (a contains e which contains i).
Directory d has total size 24933642.
As the outermost directory, / contains every file. Its total size is 48381165, the sum of the size of every file.
To begin, find all of the directories with a total size of at most 100000, then calculate the sum of their total sizes. In the example above, these directories are a and e; the sum of their total sizes is 95437 (94853 + 584). (As in this example, this process can count files more than once!)

Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?

--- Part Two ---
Now, you're ready to choose a directory to delete.

The total disk space available to the filesystem is 70000000. To run the update, you need unused space of at least 30000000. You need to find a directory you can delete that will free up enough space to run the update.

In the example above, the total size of the outermost directory (and thus the total amount of used space) is 48381165; this means that the size of the unused space must currently be 21618835, which isn't quite the 30000000 required by the update. Therefore, the update still requires a directory with total size of at least 8381165 to be deleted before it can run.

To achieve this, you have the following options:

Delete directory e, which would increase unused space by 584.
Delete directory a, which would increase unused space by 94853.
Delete directory d, which would increase unused space by 24933642.
Delete directory /, which would increase unused space by 48381165.
Directories e and a are both too small; deleting them would not free up enough space. However, directories d and / are both big enough! Between these, choose the smallest: d, increasing unused space by 24933642.

Find the smallest directory that, if deleted, would free up enough space on the filesystem to run the update. What is the total size of that directory?
*/

use itertools::Itertools;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    rc::Rc,
};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
enum EntryKind {
    File,
    Dir,
}

#[derive(Debug)]
pub struct Entry {
    id: Uuid,
    parent: Option<Rc<RefCell<Entry>>>,
    name: String,
    kind: EntryKind,
    ext: Option<String>, // eot, jpg, txt
    size: usize,
    children: Vec<Rc<RefCell<Entry>>>,
}

#[derive(Debug)]
pub struct FileSystem {
    cwd: Option<Rc<RefCell<Entry>>>,
    entries: HashMap<Uuid, Rc<RefCell<Entry>>>,
}

impl Entry {
    pub fn new_dir(parent: Option<Rc<RefCell<Entry>>>, name: String) -> Self {
        Entry {
            id: Uuid::new_v4(),
            parent,
            name,
            kind: EntryKind::Dir,
            ext: None,
            size: 0,
            children: vec![],
        }
    }

    pub fn new_file(
        parent: Option<Rc<RefCell<Entry>>>,
        name: String,
        ext: Option<String>,
        size: usize,
    ) -> Self {
        Entry {
            id: Uuid::new_v4(),
            parent,
            name,
            kind: EntryKind::File,
            ext,
            size,
            children: vec![],
        }
    }

    fn is_file(&self) -> bool {
        self.kind == EntryKind::File
    }

    fn is_dir(&self) -> bool {
        self.kind == EntryKind::Dir
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.kind {
            EntryKind::Dir => {
                write!(
                    f,
                    "/{} ({}), [{}]",
                    self.name,
                    file_size_entry(self),
                    self.id
                )
            }
            EntryKind::File => {
                if let Some(ext) = &self.ext {
                    write!(f, "{}.{} ({}) [{}]", self.name, ext, self.size, self.id)
                } else {
                    write!(f, "{} ({}) [{}]", self.name, self.size, self.id)
                }
            }
        }
    }
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            cwd: None,
            entries: HashMap::new(),
        }
    }

    fn get_cwd(&self) -> Rc<RefCell<Entry>> {
        self.cwd.as_ref().unwrap().clone()
    }

    fn get_root(&self) -> Option<Rc<RefCell<Entry>>> {
        self.entries
            .values()
            .into_iter()
            .find(|&entry| entry.borrow().name == "/")
            .cloned()
    }

    fn cd(&mut self, entry: &Rc<RefCell<Entry>>) {
        let e = entry.borrow();

        if e.is_dir() {
            self.cwd = Some(entry.clone());
        } else {
            panic!("Cannot `cd` into a file");
        }
    }

    fn add(&mut self, entry: Rc<RefCell<Entry>>) {
        let e = entry.borrow();

        match e.kind {
            // add dir to parent dirs
            EntryKind::Dir => {
                if let Some(ref parent) = e.parent {
                    parent.borrow_mut().children.push(entry.clone());
                }
            }
            // add file to parent files
            EntryKind::File => {
                if let Some(ref parent) = e.parent {
                    parent.borrow_mut().children.push(entry.clone());
                }
            }
        }

        self.entries.insert(entry.borrow().id, entry.clone());
    }
}

pub fn file_size_entry(entry: &Entry) -> usize {
    match entry.kind {
        EntryKind::File => entry.size,
        EntryKind::Dir => entry.children.iter().map(|child| file_size(child)).sum(),
    }
}

pub fn file_size(entry: &Rc<RefCell<Entry>>) -> usize {
    let e = entry.borrow();
    match e.kind {
        EntryKind::File => e.size,
        EntryKind::Dir => e.children.iter().map(|child| file_size(child)).sum(),
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> FileSystem {
    let mut fs = FileSystem::new();

    for line in input.lines() {
        let mut words = line.split_whitespace().peekable();

        /*
         * parse command from line
         *      $ cd <dir>, $ cd ..
         *      $ ls
         */
        if let Some(&"$") = words.peek() {
            let _ = words.next();

            match words.next() {
                Some("cd") => {
                    match words.next() {
                        // go up
                        Some("..") => {
                            fs.cd(fs.get_cwd().borrow().parent.as_ref().unwrap());
                        }

                        Some("/") => {
                            let root_dir =
                                Rc::new(RefCell::new(Entry::new_dir(None, "/".to_string())));

                            fs.add(root_dir.clone());
                            fs.cd(&root_dir);
                        }

                        // cd into dir (and create it if not pre-existing)
                        Some(name) => {
                            let cwd = fs.get_cwd();
                            let current_dir = cwd.borrow();

                            match current_dir.kind {
                                EntryKind::Dir => {
                                    for dir in current_dir.children.iter() {
                                        if dir.borrow().is_dir() && dir.borrow().name == name {
                                            fs.cd(dir);
                                            break;
                                        }
                                    }
                                }
                                _ => panic!("Unexpectedly got a File for `cwd`"),
                            }
                        }

                        None => panic!("Expected directory name to `cd` into"),
                    }
                }

                // list cwd contents
                Some("ls") => {
                    // do nothing
                }

                // unrecognized cmd
                Some(cmd) => panic!("{}", format!("Unrecognized command, '{}'!", cmd)),

                // unexpected end of input
                _ => panic!("Invalid input line"),
            }

            continue;
        }

        /*
         * cwd contains a directory with the specified name
         *      dir <dir>
         */
        if let Some(&"dir") = words.peek() {
            let _ = words.next();

            if let Some(name) = words.next() {
                let cwd = fs.get_cwd();
                let dir_entry = Rc::new(RefCell::new(Entry::new_dir(Some(cwd), name.to_string())));

                fs.add(dir_entry);
            }

            continue;
        }

        /*
         * cwd has a file with the specified name and size
         *      <size> <name>
         */
        if let Some(_) = words.peek() {
            let size = words
                .next()
                .expect("Invalid file")
                .parse::<usize>()
                .expect("Invalid file size");

            let full_name = words.next().expect("Invalid file name");

            let (filename, kind) = if let Some(_) = full_name.find(".") {
                let (filename, ext) = full_name.split_once(".").expect("Invalid file name");
                (filename.to_string(), Some(ext.to_string()))
            } else {
                (full_name.to_string(), None)
            };
            let cwd = fs.get_cwd();
            let file_entry = Rc::new(RefCell::new(Entry::new_file(
                Some(cwd),
                filename,
                kind,
                size,
            )));

            fs.add(file_entry);
        }
    }

    fs
}

#[aoc(day7, part1)]
pub fn part1(fs: &FileSystem) -> usize {
    fs.entries
        .values()
        .filter_map(|entry| {
            if entry.borrow().is_file() {
                return None;
            }

            let size = file_size(entry);

            if size > 100000 {
                return None;
            }

            Some(size)
        })
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(fs: &FileSystem) -> usize {
    let hdd_size = 70000000;
    let space_needed = 30000000;
    let used_size = file_size(&fs.get_root().unwrap());
    let free = hdd_size - used_size;
    let min_size_to_free = space_needed - free;

    fs.entries
        .values()
        .filter_map(|entry| {
            if entry.borrow().is_file() {
                return None;
            }

            let size = file_size(entry);

            if size < min_size_to_free {
                return None;
            }

            Some(size)
        })
        .sorted()
        .nth(0)
        .expect("Could not find a suitable directory to delete")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        let input = "$ cd /
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
        let fs = input_generator(input);

        assert_eq!(fs.entries.len(), 14);

        let root = fs.get_root().expect("Could not find root dir");
        assert_eq!(file_size(&root), 48381165);
    }

    #[test]
    fn sample1() {
        let input = "$ cd /
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
        let fs = input_generator(input);
        assert_eq!(part1(&fs), 95437);
    }

    #[test]
    fn sample2() {
        let input = "$ cd /
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
        assert_eq!(part2(&input_generator(input)), 24933642);
    }
}
