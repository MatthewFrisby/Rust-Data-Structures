#![allow(unused)]
use std::io::{stdin, stdout, Write};

#[derive(Debug, Clone)]
struct Node {
    name: String,
    mother: Option<Box<Node>>,
    father: Option<Box<Node>>,
}

impl Node {
    fn add_father(&mut self, dad: String) {
        match self.father {
            None => {
                self.father = Some(Box::new(Node {
                    name: dad,
                    mother: None,
                    father: None,
                }))
            }
            Some(ref mut dadn) => println!("Relationship already exists"),
        }
    }

    fn add_mother(&mut self, mom: String) {
        match self.mother {
            None => {
                self.mother = Some(Box::new(Node {
                    name: mom,
                    mother: None,
                    father: None,
                }))
            }
            Some(ref mut momn) => println!("Relationship already exists"),
        }
    }

    fn find(&mut self, to_find: String) -> Option<&mut Node> {
        if (self.name == to_find) {
            return Some(self);
        } else {
            match self.mother {
                Some(ref mut mom) => match mom.find(to_find.clone()) {
                    None => {}
                    found => return found,
                },
                None => {}
            }
            match self.father {
                Some(ref mut dad) => match dad.find(to_find.clone()) {
                    None => {}
                    found => return found,
                },
                None => {}
            }
            return None;
        }
    }

    fn print(&mut self, count: i32) {
        let mut i = 0;
        loop {
            if (i == count) {
                break;
            } else {
                print!("{}", '\t');

                i += 1;
            }
        }
        println!("{}", self.name);

        
        match self.mother {
            None => print!(""),
            Some(ref mut mom) => mom.print(count + 1),
        }
        match self.father {
            None => print!(""),
            Some(ref mut dad) => dad.print(count + 1),
        }
    }

    fn delete(&mut self, to_find: String) -> Option<&mut Node> {
        let mut remom = false;
        let mut remof = false;

        if (self.name == to_find) {
            return Some(self);
        } else {
            match self.mother {
                Some(ref mut mom) => match mom.delete(to_find.clone()) {
                    None => {}
                    found => remom = true,
                },
                None => {}
            }
            match self.father {
                Some(ref mut dad) => match dad.delete(to_find.clone()) {
                    None => {}
                    found => remof = true,
                },
                None => {}
            }
            if (remom) {
                self.mother = None;
            } else if (remof) {
                self.father = None;
            } else {
                ()
            }
            return None;
        }
    }
}

fn main() {
    let mut s = String::new();
    println!("Please enter your name");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Invalid command");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }

    let mut user = Node {
        name: s.to_string(),
        mother: None,
        father: None,
    };

    loop {
        let mut temp = user.clone();
        let mut input = String::new();
        println!("Next action please");
        stdin().read_line(&mut input);

        let cloned = input.clone();
        let items = cloned.split(' ');
        let mut cmds: Vec<&str> = items.collect();
        let command = cmds[0].trim_end();
        
        match command {
            command if command == "add" => {
                let name = cmds[1].to_string();
                let rela = cmds[2];
                let child = cmds[3].trim_end().to_string();

                match user.clone().find(name.clone()) {
                    Some(ref mut node) => println!("Name already exists"),
                    None => match rela {
                        rela if rela == "father" => match user.find(child) {
                            None => println!("Name not found"),
                            Some(ref mut node) => node.add_father(name),
                        },
                        rela if rela == "mother" => match user.find(child) {
                            None => println!("Name not found"),
                            Some(ref mut node) => node.add_mother(name),
                        },
                        _ => println!("Invalid relationship"),
                    },
                }
            }
            command if command == "delete" => {
                let mut boo = false;
                let name = cmds[1].trim_end().to_string();
                if (user.name == name.clone()) {
                    println!("Deletion failed");
                } else {
                    match user.find(name.clone()) {
                        None => println!("Name not found"),
                        Some(ref mut node) => {
                            boo = true;
                        }
                    }
                }
                if (boo) {
                    user.delete(name.clone());
                    println!("Delete completed");
                }
            }
            command if command == "print" => {
                user.print(0);
            }
            command if command == "quit" => {
                println!("Good Bye");
                break;
            }
            _ => println!("Invalid command"),
        }
    }
    drop(user);
}
