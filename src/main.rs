use std::io;

fn main() {
    println!("How many fib numbers do you want me to print?");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Cannot read input");
    let n = n.trim().parse().expect("Cannot convert input");
    println!();
    let mut x = 0;
    let mut y = 1;
    let mut z : i64;
    for _i in 0..n {
        z = x + y;
        println!("{}", z);
        x = y;
        y = z;
    }
}