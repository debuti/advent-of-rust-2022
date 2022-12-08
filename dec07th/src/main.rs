use std::cell::RefCell;
use std::rc::Rc;

static SOURCES: [(&'static str, &'static [u8]); 2] = [
    ("input", include_bytes!("input.txt")),
    ("test", include_bytes!("test.txt")),
];

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum Cmd<'a> {
    cd(&'a str),
    ls(Vec<Content<'a>>),
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum Content<'a> {
    dir(&'a str),
    file((usize, &'a str)),
}

#[derive(Debug)]
enum Node {
    Dir(String, RefCell<Vec<Rc<Node>>>),
    File(String, usize),
}

fn main() {
    let data = String::from_utf8_lossy(
        SOURCES
            .iter()
            .filter(|x| x.0 == std::env::args().nth(1).unwrap())
            .nth(0)
            .unwrap()
            .1,
    );

    let cmds = data
        .split("$ ")
        .skip(1)
        .map(|c| {
            if c.chars().nth(0).unwrap() == 'c' {
                Cmd::cd(&c[3..c.len() - 1])
            } else {
                Cmd::ls(
                    c[3..c.len() - 1]
                        .split("\n")
                        .map(|l| {
                            if &l[0..3] == "dir" {
                                Content::dir(&l[4..])
                            } else {
                                Content::file({
                                    let (s, n) = l.split_once(" ").unwrap();
                                    (s.parse().unwrap(), n)
                                })
                            }
                        })
                        .collect(),
                )
            }
        })
        .collect::<Vec<Cmd>>();

    let tree = Rc::new(Node::Dir("/".to_string(), RefCell::new(vec![])));
    let mut crumbs = vec![Rc::clone(&tree)];
    // We know we are at root, so skip it
    for cmd in cmds.into_iter().skip(1) {
        match cmd {
            Cmd::ls(v) => {
                for item in v {
                    match item {
                        Content::file((size, name)) => {
                            if let Node::Dir(_, vector) = &**crumbs.last().unwrap() {
                                vector
                                    .borrow_mut()
                                    .push(Rc::new(Node::File(String::from(name), size)))
                            }
                        }
                        Content::dir(name) => {
                            if let Node::Dir(_, vector) = &**crumbs.last().unwrap() {
                                vector.borrow_mut().push(Rc::new(Node::Dir(
                                    String::from(name),
                                    RefCell::new(vec![]),
                                )))
                            }
                        }
                    }
                }
            }
            Cmd::cd(n) => {
                if n == ".." {
                    crumbs.pop();
                } else {
                    let mut found = None;
                    if let Node::Dir(_, vector) = &**crumbs.last().unwrap() {
                        for item in &*vector.borrow() {
                            match &**item {
                                Node::Dir(name, _) => {
                                    if n == name {
                                        found = Some(Rc::clone(&item));
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                        if let Some(value) = found {
                            crumbs.push(value);
                        } else {
                            panic!("cd to a non existing folder: {}", n);
                        }
                    }
                }
            }
        }
    }

    let mut sizes = vec![];
    let total = du_s(&tree, &mut sizes);
    println!(
        "1: {}\n2: {}",
        sizes.iter().filter(|&x| *x < 100000).sum::<usize>(),
        sizes.iter().filter(|&x| *x >=30000000-(70000000-total)).min().unwrap()
    );
}

fn du_s(ptr: &Rc<Node>, acc: &mut Vec<usize>) -> usize {
    match ptr.as_ref() {
        Node::Dir(_, vector) => {
            let mut foldersize = 0;
            for item in vector.borrow().iter() {
                foldersize += du_s(&item, acc);
            }
            acc.push(foldersize);
            foldersize
        }
        Node::File(_, size) => *size,
    }
}
