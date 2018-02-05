#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::iter::FromIterator;
use std::io::{stdin,stdout,Write};


// calculate mean
fn mean(data: &Vec<i32>) -> f32 {
    let total: i32 = data.iter().sum();
    total as f32/ data.len() as f32
}

// median middle value of a sorted vector, or the mean of the two center values if the length of the vector is an even number
fn median(data: &mut Vec<i32>) -> i32 {
    data.sort();
    let mid = data.len() / 2;
    if data.len() % 2 == 0 {
        // mean of two center values
        (data[mid-1] + data[mid]) / 2
    } else {
        data[mid]
    }
}

// mode: the most frequent number
fn mode(data: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for n in data {
        *map.entry(*n).or_insert(0) += 1;
    }
    let mut count_v: Vec<_> = map.iter().collect();
    count_v.sort_by(|a, b| a.1.cmp(b.1).reverse());
    *count_v[0].0
}

// convert a string to pig latin
fn to_latin(s: &str) -> String {
    // let bs = s.as_bytes();
    let c = s.chars().next();
    match c.unwrap() {
        'a' | 'e' | 'i' | 'o' | 'u' => { return format!("{}{}", s, "hay"); }
        _ => { return format!("{}{}{}", &s[1..], c.unwrap(), "ay"); }
    }
}


fn second_word(s: &str) -> &str {
    let bs = s.as_bytes();
    let mut start = 0;
    for (i, &item) in bs.iter().enumerate() {
        if item == b' ' {
            if start == 0 {
                start = i+1;
            } else {
                return &s[start..i];
            }
        }
    }
    &s
}

fn first_word(s: &str) -> &str {    // Now you can pass literals as well as strings
    let happen = s.as_bytes();
    for (i, &item) in happen.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s  // slice containing the entire string
}

struct Database {
    people: HashMap<String, String>,
    // depts: HashMap<_, _>,
}

impl Database {

    fn new() -> Database {
        Database { people: HashMap::new() }
    }

    fn command(&mut self, s: &str) {
        let words: Vec<&str> = Vec::from_iter(s.split_whitespace());
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
        let mut v = Vec::from_iter(self.people.iter());
        v.retain(|x| *x.1 == dept);
        v.sort_by(|a, b| a.0.cmp(b.0));
        println!("\nEmployees in Department '{}':", dept);
        for p in v {
            println!("{:10}", p.0);
        } 
    }

    fn print_all(&self) {
        let mut v = Vec::from_iter(self.people.iter());
        v.sort_by(|a, b| a.0.cmp(b.0));
        println!("\nEmployees:");
        for p in v {
            println!("{:10} {:<}", p.0, p.1);
        } 
    }
}

fn main() {
    let mut d = Database::new();
    d.command("Add Sally to Engineering");
    d.command("Add Amir to Sales");
    d.command("Add Henkie to Engineering");
    d.command("Add Jan to Baarden");
    d.command("Add Piet to Baarden");
    d.command("Add Joris to Baarden");
    let s = "Print All".to_string();
    let s2 = "Print Dept Engineering".to_string();
    d.command(&s);
    d.command(&s2);
    d.command("add x to y");

//     let v1 = vec![3, -7, 5, 13, -2];
//     println!("{}", mean(&v1)); // 2.4

//     let mut v2 = vec![3, 13, 7, 5, 21, 23, 23, 40, 23, 14, 12, 56, 23, 29];
//     println!("{}", median(&mut v2)); // 22

//     let mut v3 = vec![3, 13, 7, 5, 21, 23, 39, 23, 40, 23, 14, 12, 56, 23, 29];
//     println!("{}", median(&mut v3)); // 23
    
//     let v4 = vec![19, 8, 29, 35, 19, 28, 15];
//     println!("{}", mode(&v4)); // 19
    
//     let s1 = "first".to_string();
//     println!("{}", to_latin(&s1)); // irst-fay

//     let s2 = "apple".to_string();
//     println!("{}", to_latin(&s2)); // apple-hay

//     println!("{}", to_latin("latin"));
//     println!("{}", to_latin("banana"));
//     println!("{}", to_latin("trash"));
//     println!("{}", to_latin("happy"));
}
