fn main() {
    let s = String::from("first");
    let fChar = &s[..1];
    let laterChars = &s[1..];
    let ans = format!("{laterChars}-{fChar}ay");
    println!("{ans}");
}
