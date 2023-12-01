use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
    process::Child,
    rc::{self, Rc},
};

#[derive(Debug)]
enum NodeType {
    File,
    Directory,
}

#[derive(Debug)]
struct Node {
    name: String,
    node_type: NodeType,
    size: Option<i32>,
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
struct FileSystem {
    pwd: Rc<RefCell<Node>>,
    root: Rc<RefCell<Node>>,
}

impl FileSystem {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(Node {
            name: "/".to_string(),
            node_type: NodeType::Directory,
            size: None,
            parent: None,
            children: Vec::new(),
        }));

        FileSystem {
            root: Rc::clone(&root),
            pwd: Rc::clone(&root),
        }
    }

    pub fn chdir(&mut self, dir_name: &str) {
        match dir_name {
            "/" => {
                self.pwd = Rc::clone(&self.root);
            }
            ".." => {
                let parent = self.pwd.borrow().parent.clone();
                self.pwd = parent.unwrap();
            }
            _ => {
                let target = match self
                    .pwd
                    .borrow()
                    .children
                    .iter()
                    .find(|child| child.borrow().name.eq(dir_name))
                {
                    Some(child) => child.clone(),
                    None => panic!("Directory not found"),
                };

                self.pwd = target;
            }
        }
    }

    pub fn add_file(&self, name: &str, size: u32) {
        let file = Node {
            name: name.to_string(),
            node_type: NodeType::File,
            size: Some(size as i32),
            parent: Some(Rc::clone(&self.pwd)),
            children: Vec::new(),
        };

        self.pwd
            .borrow_mut()
            .children
            .push(Rc::new(RefCell::new(file)));
    }
    pub fn add_dir(&self, name: &str) {
        let dir = Node {
            name: name.to_string(),
            node_type: NodeType::Directory,
            size: None,
            parent: Some(Rc::clone(&self.pwd)),
            children: Vec::new(),
        };

        self.pwd
            .borrow_mut()
            .children
            .push(Rc::new(RefCell::new(dir)));
    }
}

fn process_command(fs: &mut FileSystem, command: Vec<&str>) {
    match command.get(1) {
        Some(&"cd") => {
            let dir_name = command.get(2).unwrap();
            fs.chdir(dir_name);
        }
        Some(&"ls") => {
            println!("list the files in side {}", fs.pwd.borrow().name);
        }
        None | _ => {
            panic!("Invalid command")
        }
    }
}

fn process_ls_output(fs: &FileSystem, output: Vec<&str>) {
    println!("{:?}", output);
    match output.get(0) {
        Some(&"dir") => {
            let dir_name = output.get(1).unwrap();
            fs.add_dir(dir_name);
        }
        Some(c) => {
            println!("{:?}", c);
            let file_size = output.get(0).unwrap().parse::<u32>().unwrap();
            let file_name = output.get(1).unwrap();
            fs.add_file(file_name, file_size);
        }
        None | _ => {
            panic!("Invalid command")
        }
    }
}

fn main() {
    let path = Path::new("input.txt");
    let lines = read_lines(path).unwrap();

    let line_vec: Vec<String> = lines.map(|l| l.unwrap()).collect();

    let mut i = 0;

    let mut fs = FileSystem::new();

    while i < line_vec.len() {
        let line = line_vec.get(i).unwrap();

        let mut split = line.split_whitespace().peekable();

        if split.peek().unwrap().eq_ignore_ascii_case("$") {
            process_command(&mut fs, split.collect())
        } else {
            process_ls_output(&fs, split.collect())
        }

        i += 1;
    }

    println!("{:?}", fs.root);
}

fn read_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
