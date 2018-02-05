// Convert strings to pig latin. The first consonant of each word is moved to the
// end of the word and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead
// (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8
// encoding (https://doc.rust-lang.org/book/second-edition/ch08-03-hash-maps.html)!


pub fn to_latin(s: &str) -> String {
    let c = s.chars().next();       // get the first character
    match c.unwrap() {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}{}", s, "-hay"),
        _ => format!("{}-{}{}", &s[1..], c.unwrap(), "ay"),
    }
}

