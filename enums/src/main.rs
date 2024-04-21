fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        impl Message {
            fn call(&self) {
                match self {
                    Message::Quit => println!("Quit"),
                    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
                    Message::Write(s) => println!("Write: {}", s),
                    Message::ChangeColor(r, g, b) => {
                        println!("Change color to ({}, {}, {})", r, g, b)
                    }
                }
            }
        }

        let m = Message::Write(String::from("hello"));
        m.call();
    }

    {
        let some_number = Some(5);
        let some_char = Some('e');
    
        let absent_number: Option<i32> = None;
        let b = some_number.unwrap();
        let c = absent_number.unwrap();
        println!("{b}");
    }
}
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
