#![allow(unused)]
use std::io::{stdin, stdout, Write};

type Stackelem = Option<Box<Valnode>>;
#[derive(Debug, Clone)]
struct Stack {
    head: Stackelem,
}
#[derive(Debug, Clone)]
struct Valnode {
    elem: String,
    next: Stackelem,
}

impl Stack {
    fn push(&mut self, name: String) {
        match self.head.clone() {
            None => {
                self.head = Some(Box::new(Valnode {
                    elem: name,
                    next: None,
                }))
            }
            node => {
                self.head = Some(Box::new(Valnode {
                    elem: name,
                    next: node,
                }));
            }
        }
    }

    fn pop(&mut self) -> Option<String> {
        match self.head.clone() {
            None => None,
            Some(ref mut node) => {
                match node.clone().next {
                    None => self.head = None,
                    Some(ref mut next) => self.head = node.clone().next,
                }
                Some(node.elem.clone())
            }
        }
    }

    fn print(&mut self) {
        let mut temp = self.clone();
        loop {
            let node = temp.pop();
            match node {
                None => break,
                Some(ref string) => println!("{}", string),
            }
        }
    }
}

fn main() {
    println!("Starting Queue");
    let mut my_stack = Stack { head: None };
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
                my_stack.push(name);
            }
            command if command == "pop" => {
                let empty = my_stack.pop();
                match empty {
                    None => println!("Nothing left to remove"),
                    Some(ref string) => println!("Removed: {}", string),
                }
            }
            command if command == "print" => {
                my_stack.print();
            }
            command if command == "quit" => {
                println!("Good Bye");
                break;
            }
            _ => println!("Invalid command"),
        }
    }
    drop(my_stack);
}
