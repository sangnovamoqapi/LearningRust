fn main() {
    let mut s = String::from("foo");
    s += "bar";
    s.push_str("zo");
    s.push('o');
    println!("{s}");

    {
        // addition of two strings
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    }

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    {
        for c in "नमस्ते".chars() {
            print!("{c} ");
        }
        println!("");
        for c in "नमस्ते".bytes() {
            print!("{c} ");
        }
    }
}
