mod centraltendencies;
mod database;

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

fn main() {
    let mut d = database::Database::new();
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

    let mut ct = centraltendencies::CentralTendencies::new();
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
