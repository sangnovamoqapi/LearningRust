fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    let x = five();
    println!("The value of x from five() is {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
// Return type is denoted by prefixing with '->'
// Last expression returns
fn five() -> i32 {
    5
}
