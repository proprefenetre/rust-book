#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::collections::HashMap;

#[derive(Debug)]
struct CentralTendency {
    mean: f32,
    median: f32,
    mode: Vec<i32>,
}

impl CentralTendency {
    fn new() -> CentralTendency {
        CentralTendency { mean: 0f32, median: 0f32, mode: vec![0],}
    }

    fn calculate(&mut self, data: Vec<i32>) {
        self.mean = self.calc_mean(&data);
        self.mode = self.calc_mode(&data);
        self.median = self.calc_median(data);
    }

    // calculate mean
    fn calc_mean(&self, data: &[i32]) -> f32 {
        data.iter().fold(0i32, |s, &n| s + n as i32) as f32 / data.len() as f32
    }

    // median middle value of a sorted vector, or the mean of the two center 
    // values if the length of the vector is an even number
    fn calc_median(&self, mut data: Vec<i32>) -> f32 {
        data.sort();
        let mid = data.len() / 2;
        if data.len() % 2 == 0 {
            (data[mid-1] as f32 + data[mid] as f32) / 2 as f32
        } else {
            data[mid] as f32
        }
    }
    
    // mode: the number(s) that occurs most frequently
    fn calc_mode(&self, data: &[i32]) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for n in data {
            *map.entry(*n).or_insert(0) += 1;
        }

        let max = map.values()
                     .max()
                     .unwrap()
                     .clone();

        map.into_iter()
            .filter(|v| v.1 == max)
            .map(|v| v.0)
            .collect::<Vec<i32>>()
        
    }
}


// convert a string to pig latin
fn to_latin(s: &str) -> String {
    // let bs = s.as_bytes();
    let c = s.chars()
             .next();
    match c.unwrap() {
        'a' | 'e' | 'i' | 'o' | 'u' => { return format!("{}{}", s, "hay"); }
        _ => { return format!("{}{}{}", &s[1..], c.unwrap(), "ay"); }
    }
}

// retrieve the first word of a string (slice)
fn first_word(s: &str) -> &str {    // Now you can pass literals as well as strings
    let happen = s.as_bytes();
    for (i, &item) in happen.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s  // slice containing the entire string
}

// retrieve the second word of a string (slice)
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

#[derive(Debug)]
struct Database {
    people: HashMap<String, String>,
}

impl Database {

    fn new() -> Database {
        Database { people: HashMap::new() }
    }

    fn command(&mut self, s: &str) {
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

fn main() {
    let mut d = Database::new();
    d.command("Add Sally to Engineering");
    d.command("Add Amir to Sales");
    d.command("Add Henkie to Engineering");
    d.command("Add Jan to Baarden");
    d.command("Add Piet to Baarden");
    d.command("Add Joris to Baarden");

    let s1 = "Print All".to_string();
    let s2 = "Print Dept Engineering".to_string();
    let s3 = "Print Dept Baarden";

    d.command(&s1);
    d.command(&s2);
    d.command(s3);

    let mut ct = CentralTendency::new();
    let v1 = vec![3, -7, 5, 13, -2];
    ct.calculate(v1);
    println!("{:?}", ct);

    // assert_eq!(mean(&v1), 2.4);

    let v2 = vec![3, 13, 7, 5, 21, 23, 23, 40, 23, 14, 12, 56, 23, 29];
    ct.calculate(v2);
    println!("{:?}", ct);
    // assert_eq!(median(&mut v2), 22);

    let v3 = vec![3, 13, 7, 5, 21, 23, 39, 23, 40, 23, 14, 12, 56, 23, 29];
    ct.calculate(v3);
    println!("{:?}", ct);
    // assert_eq!(median(&mut v3), 23);
    
    let v4 = vec![19, 8, 29, 35, 19, 28, 15];
    ct.calculate(v4);
    println!("{:?}", ct);
    

    ct.calculate(vec![3, 5, 8]);
    println!("{:?}", ct);


    let s1 = "first".to_string();
    assert_eq!(to_latin(&s1), "irstfay");

    let s2 = "apple".to_string();
    assert_eq!(to_latin(&s2), "applehay");

    assert_eq!(to_latin("latin"), "atinlay");
    assert_eq!(to_latin("banana"), "ananabay");

}
