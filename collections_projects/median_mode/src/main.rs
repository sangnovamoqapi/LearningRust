fn main() {
    let mut v = vec![5,3,4,2,1];
    v.sort();
    let median = v.get(v.len()/2);
    match median {
        Some(v) => println!("the median is {v}"),
        None => println!("Some sorcery happened!"),
    }
    let mode = v.iter().max();
    match mode {
        Some(i) => println!("The mode of the vector is {i}"),
        None => println!("Some sorcery happened in mode!"),
    }
}
