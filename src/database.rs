// Using a hash map and vectors, create a text interface to allow a user to add
// employee names to a department in a company. For example, “Add Sally to
// Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of
// all people in a department or all people in the company by department, sorted
// alphabetically (https://doc.rust-lang.org/book/second-edition/ch08-03-hash-maps.html).

use std::collections::HashMap;

#[derive(Debug)]
pub struct Database {
    people: HashMap<String, String>,
}

impl Database {

    pub fn new() -> Database {
        Database { people: HashMap::new() }
    }

    pub fn command(&mut self, s: &str) {
        let words: Vec<&str> = s.split_whitespace().collect();
        match words[0] {
            "Add" => { 
                self.add(words[1].to_owned(), words[3].to_owned());
            },
            "Print" => {
                match words[1] {
                    "Dept" => self.print_dept(words[2].to_owned()),
                    "All" => self.print_all(),
                    &_ => println!("Not a valid category: {:?}", words[1].to_owned()),
                }
            },
            &_ => println!("Not a valid command: {:?}", words[0]),
        }
    }

    fn add(&mut self, name: String, dept: String) {
        self.people.insert(name, dept);
    }

    fn print_dept(&self, dept: String) { 
        // retrieve a list of all people in a department, sorted alphabetically.
        let mut v: Vec<_> = self.people.iter().collect();
        v.retain(|x| *x.1 == dept);
        v.sort_by(|a, b| a.0.cmp(b.0));
        println!("\nEmployees in Department '{}':", dept);
        for p in v {
            println!("{:10}", p.0);
        } 
        println!("\n");
    }

    fn print_all(&self) {
        let mut v: Vec<_> = self.people.iter().collect();
        v.sort_by(|a, b| a.0.cmp(b.0));
        println!("\nEmployees:");
        for p in v {
            println!("{:10} {:<}", p.0, p.1);
        } 
        println!("\n");
    }
}

