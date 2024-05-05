use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = String::from("Hello");
    let y = String::from("World!");
    let ann = "I am going to announce the biggest word of the two Hello or World with !";
    println!("And It is {}",longest_with_an_announcement(&x, &y, ann));
}
