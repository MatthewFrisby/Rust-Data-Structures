#![allow(unused)]
use std::io::{stdin, stdout, Write};

#[derive(Debug, Clone)]
struct Node {
    name: String,
    next: Option<Box<Node>>,
}

impl Node {
    fn push(&mut self, name: String) {
        match self.next {
            None => {
                self.next = Some(Box::new(Node {
                    name: name,
                    next: None,
                }))
            }
            Some(ref mut node) => node.push(name),
        }
    }

    fn pop(&mut self) -> Option<&mut Node> {
        match self.next {
            None => {
                print!("");
                None
            }
            Some(ref mut node) => match node.next {
                None => {
                    print!("");
                    None
                }
                Some(ref mut nextn) => Some(nextn),
            },
        }
    }
    fn print(&mut self, count: i32) {
        if (self.name != "head".to_string()) {
            print!("{}", self.name);
        } else {
            print!("");
        }
        match self.next {
            None => print!("{}", '\n'),
            Some(ref mut node) => {
                if (self.name != "head".to_string()) {
                    print!(" -> ");
                } else {
                    print!("");
                };
                node.print(count + 1)
            }
        }
    }

    fn count(&mut self, count: i32) {
        match self.next {
            None => println!("{}", count),
            Some(ref mut node) => {
                node.count(count + 1);
            }
        }
    }
}

fn main() {
    println!("Starting Queue");
    let mut head = Node {
        name: "head".to_string(),
        next: None,
    };

    loop {
        let mut input = String::new();
        println!("Next action please");
        stdin().read_line(&mut input);

        let cloned = input.clone();
        let items = cloned.split(' ');
        let mut cmds: Vec<&str> = items.collect();
        let command = cmds[0].trim_end();

        match command {
            command if command == "push" => {
                let name = cmds[1].trim_end().to_string();
                head.push(name);
            }
            command if command == "pop" => match head.clone().pop() {
                None => head.next = None,
                Some(ref mut node) => head.next = Some(Box::new(node.clone())),
            },
            command if command == "print" => {
                head.print(0);
            }
            command if command == "count" => {
                head.count(0);
            }
            command if command == "quit" => {
                println!("Good Bye");
                break;
            }
            _ => println!("Invalid command"),
        }
    }
    drop(head);
}
