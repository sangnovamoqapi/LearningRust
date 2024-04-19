fn main() {
    let s = String::from("Hello world");
    let index = first_word(&s);
    println!("{index}");
    {
        let s = String::from("hello world");
        let hello = &s[..5];
        let world = &s[6..];
        let hello_world = &s[..];
        println!("{hello}{world}{hello_world}")
    }
    let word = first_word_slice(&s);
    println!("{word}");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
