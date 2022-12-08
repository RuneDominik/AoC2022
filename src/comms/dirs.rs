#![allow(dead_code)]

use std::{
    fs::File,
    io::{prelude::*, BufReader, Error, ErrorKind},
    path::Path,
};

trait Name {
    fn get_name(&self) -> Result<&str, Error>;
}

trait Cd {
    fn cd(&self, target: &str) -> Result<&Dir, Error>;
}

trait Ls {
    fn ls(&self) -> Result<Vec<&str>, Error>;
}

trait Size {
    fn size(&self) -> Result<u64, Error>;
}

trait Register {
    fn register_file(&mut self, size: u64);
    fn register_subdir(&mut self, name: &str);
}

trait Contains {
    fn contains(&self, target:&str) -> bool;
}

//#[derive(Debug)]
//struct Root {
//    subdirs: Vec<Dir>,
//    files: Vec<u64>
//}

#[derive(Debug)]
enum Parent<'a> {
    Dir(&'a Dir<'a>),
    String(&'a str),
}

impl Contains for Dir<'_> {
    fn contains(&self, target:&str) -> bool { 
            let names = self.ls().unwrap();
            return names.contains(&target);
            }
    }

#[derive(Debug)]
struct Dir<'a> {
    name: &'a str,
    //parent: Parent<'a>,
    parent: Option<&'a Dir<'a>>,
    subdirs: Vec<Dir<'a>>,
    files: Vec<u64>,
    plain: bool
}


impl Dir<'_> {
    fn new<'a>(name: &'a str, parent: Option<&'a Dir<'a>>) -> Dir<'a> {
        Dir { name: &name, parent: parent, subdirs:Vec::new(), files:Vec::new(), plain:true }
    }
}


impl Name for Dir<'_> {
    fn get_name(&self) -> Result<&str, Error> {
        return Ok(self.name);
    }
}

impl Register for Dir<'_> {
    fn register_file(&mut self, size: u64) {
        self.files.push(size);
    }
    fn register_subdir(&mut self, name: &str) {
        self.plain = false;
        self.subdirs.push(Dir::new(name, Some(self)));
    }
}

impl Size for Dir<'_> {
    fn size(&self) -> Result<u64, Error> {
        if self.plain {
            return Ok(self.files.iter().sum());
        } else {
            let filesum: u64 = self.files.iter().sum();
            let dirsum: u64 = self.subdirs.iter().map(|x| x.size().unwrap()).sum();

            return Ok(filesum + dirsum);
        }
    }
}


impl Cd for Dir<'_> {
    fn cd(&self, target: &str) -> Result<&Dir, Error> {
        let root_err = Error::new(ErrorKind::Other, "Cant move up from root");
        let not_found_err = Error::new(ErrorKind::Other, "Target not found");

        if target == ".." {
            if self.parent.is_none() {
                return Err(root_err);
            } else {
                return Ok(self.parent.unwrap());
            }
        } else {
            let names = self.ls();

            println!("{:?}", &names);
            //let ind = names.iter().position(|&x| x==target);
            let ind = Some(1);
            if ind.is_some() {
                return Ok(&self.subdirs[ind.unwrap()]);
            } else {
                return Err(not_found_err);
            }
        }
    }
}

impl Ls for Dir<'_> {
    fn ls(&self) -> Result<Vec<&str>, Error> {
        return Ok(self.subdirs.iter().map(|x| x.get_name().unwrap()).collect::<Vec<&str>>());
    }
}


fn parse_lines(lines: Vec<&str>) -> Result<u64, Error> {
    let mut lines = lines.iter();
    lines.next();
    let Root = Dir::new("tldir", None);
    let mut current_dir = &Root;

    loop {
        let mut line = lines.next();
        if line.is_none() {
            break;
        }

        let mut line = line.unwrap();

        if *line == "$ ls" {
            continue;
        } else if line.contains("dir") {
            let target = line.replace("dir ", "");
            if current_dir.contains(&target) {
               continue;
            } else {
               current_dir.register_subdir(&target);
            }
        } else if line.contains("cd") {
            let target = line.replace("cd ", "");
            current_dir = current_dir.cd(&target).unwrap();
        } else {
            let v: Vec<&str> = line.split(" ").collect();
            let file_size: u64 = v[0].parse().unwrap();
            current_dir.register_file(file_size);
        }
    }
        
    return Ok(Root.size().unwrap());
}


pub fn get_size_at_most(path: impl AsRef<Path>) -> Result<u64, Error> {
    let file = File::open(path)?;

    let br = BufReader::new(file);
    // let inp: Vec<&str> = br.lines().map(|x| &x.unwrap()).collect();
    let inp: Vec<String> = br.lines().collect::<Result<_, _>>().unwrap();
    let lines = Vec::from_iter(inp.iter().map(String::as_str));
    let root_size: u64 = parse_lines(lines).unwrap();

    Ok(root_size)
}
