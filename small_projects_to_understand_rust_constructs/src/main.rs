use rand::Rng;
fn main() {
    for f in (-40..140).step_by(20) {
        let celcius = convert_f_c(f as f32);
        println!("{f} -> {celcius}");
    }
    let n = rand::thread_rng().gen_range(0..=20);
    let fibo = nth_fibo(n);
    println!("The {n}th fibonacci number is {fibo}")

}

fn convert_f_c(f: f32) -> f32 {
    ((5.0/9.0)*(f-32.0)).into()
}

fn nth_fibo(n: u64) -> u64 {
    let mut f = 0;
    let mut s = 1;
    if n == 0 {
        return f;
    }
    if n == 1 {
        return s;
    }
    for _ in 2..=n {
        let temp = f;
        f = s;
        s += temp;
    }
    s

}