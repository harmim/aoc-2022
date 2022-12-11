use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::usize;

use crate::{DaySolution, FromInput};

struct Dir {
    files: RefCell<HashMap<String, usize>>,
    dirs: RefCell<HashMap<String, Rc<Dir>>>,
    back: RefCell<Weak<Dir>>,
}

pub struct Day07(Rc<Dir>);

impl FromInput for Day07 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let root = Rc::new(Dir {
            files: RefCell::new(HashMap::new()),
            dirs: RefCell::new(HashMap::new()),
            back: RefCell::new(Weak::new()),
        });
        let mut dir = root.clone();

        for line in lines {
            if line.starts_with("$ ls") {
                continue;
            }

            if line.starts_with("$ cd") {
                match &line[5..] {
                    "/" => dir = root.clone(),
                    ".." => dir = dir.clone().back.borrow().upgrade().unwrap(),
                    name => dir = dir.clone().dirs.borrow().get(name).unwrap().clone(),
                }

                continue;
            }

            match line.split_once(' ').unwrap() {
                ("dir", name) => {
                    dir.dirs.borrow_mut().insert(
                        name.to_string(),
                        Rc::new(Dir {
                            files: RefCell::new(HashMap::new()),
                            dirs: RefCell::new(HashMap::new()),
                            back: RefCell::new(Rc::downgrade(&dir)),
                        }),
                    );
                }
                (size, name) => {
                    dir.files
                        .borrow_mut()
                        .insert(name.to_string(), size.parse().unwrap());
                }
            }
        }

        Self(root)
    }
}

impl DaySolution for Day07 {
    fn part_one(&self) -> String {
        dir_size(self.0.clone(), 100_000).1.to_string()
    }

    fn part_two(&self) -> String {
        space_to_delete(
            self.0.clone(),
            dir_size(self.0.clone(), 0).0 - (70_000_000 - 30_000_000),
        )
        .1
        .to_string()
    }
}

fn dir_size(dir: Rc<Dir>, limit: usize) -> (usize, usize) {
    let f: usize = dir.files.borrow().iter().map(|(_, size)| size).sum();
    let d = dir
        .dirs
        .borrow()
        .iter()
        .map(|(_, d)| dir_size(d.clone(), limit))
        .reduce(|(a1, b1), (a2, b2)| (a1 + a2, b1 + b2))
        .unwrap_or((0, 0));
    let size = f + d.0;

    (size, if size <= limit { size + d.1 } else { d.1 })
}

fn space_to_delete(dir: Rc<Dir>, needed_to_free: usize) -> (usize, usize) {
    let f: usize = dir.files.borrow().iter().map(|(_, size)| size).sum();
    let d = dir
        .dirs
        .borrow()
        .iter()
        .map(|(_, d)| space_to_delete(d.clone(), needed_to_free))
        .reduce(|(a1, b1), (a2, b2)| (a1 + a2, cmp::min(b1, b2)))
        .unwrap_or((0, usize::MAX));
    let size = f + d.0;

    (
        size,
        if size > needed_to_free && size < d.1 {
            size
        } else {
            d.1
        },
    )
}
