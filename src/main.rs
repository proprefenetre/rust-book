mod centraltendencies;
mod latin;
mod database;

use latin::to_latin;

fn main() {

    // 1. Central tendencies
    let mut ct = centraltendencies::CentralTendencies::new();
    let v1 = vec![3, -7, 5, 13, -2];
    ct.calculate(v1);
    assert_eq!(ct.mean, 2.4);
    assert_eq!(ct.median, 3f32);
    assert_eq!(ct.mode.sort(), vec![3, -7, 5, 13, -2].sort());       // HashMaps are not ordered so the resulting vector isn't either.

    let v2 = vec![3, 13, 7, 5, 21, 23, 23, 40, 23, 14, 12, 56, 23, 29];
    ct.calculate(v2);
    assert_eq!(ct.mean, 20.857143);
    assert_eq!(ct.median, 22f32);
    assert_eq!(ct.mode, vec![23]);

    let v3 = vec![3, 13, 7, 5, 21, 23, 39, 23, 40, 23, 14, 12, 56, 23, 29];
    ct.calculate(v3);
    assert_eq!(ct.mean, 22.066668);
    assert_eq!(ct.median, 23f32);
    assert_eq!(ct.mode, vec![23]);
    
    ct.calculate(vec![19, 8, 29, 35, 19, 28, 15]);
    assert_eq!(ct.mean, 21.857143);
    assert_eq!(ct.median, 19f32);
    assert_eq!(ct.mode, vec![19]);

    ct.calculate(vec![3, 5, 8]);
    assert_eq!(ct.mean, 5.3333335);
    assert_eq!(ct.median, 5f32);
    assert_eq!(ct.mode.sort(), vec![3, 5, 8].sort());

    // 2. Pig latin
    let s1 = "first".to_string();
    assert_eq!(to_latin(&s1), "irst-fay");

    let s2 = "apple".to_string();
    assert_eq!(to_latin(&s2), "apple-hay");

    assert_eq!(to_latin("latin"), "atin-lay");

    assert_eq!(to_latin("banana"), "anana-bay");

    // 3. Database
    let mut d = database::Database::new();
    d.command("Add Sally to Engineering");
    d.command("Add Amir to Sales");
    d.command("Add Jan to Baarden");
    d.command("Add Piet to Baarden");
    d.command("Add Joris to Baarden");
    d.command("Add Korneel to Scheel kijken"); // oops

    let s1 = "Print All".to_string();
    let s2 = "Print Dept Engineering".to_string();
    let s3 = "Print Dept Baarden";

    d.command(&s1);
    d.command(&s2);
    d.command(s3);
    d.command("Print Dept Scheel kijken"); // lucky
}
