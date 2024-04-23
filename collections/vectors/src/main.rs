fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1,2,3];

    let mut v = Vec::new();

    v.push(5);
    v.push(4);
    v.push(3);
    v.push(2);
    v.push(1);

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element!"),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }    
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{i}");
    }  

    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
    
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    
    }

}
