/// # Loops in Rust

fn main() {

    /// ## Using `loop`
    let mut x = 10;
    loop {
        x += x - 3;
        println!("x is {}", x);
        if x % 5 == 0 { break; }
    }

    /// Using `for`
    for i in 0..5 {
        println!("\"i\" is now {}", i);
    }

    for (i, val) in (1..11).enumerate() {
        println!("{}: {}", i, val);
    }

    let v = vec!["one", "two", "three", "four"];
    for num in &v {
        println!("In v we have {}", num);
    }
}
