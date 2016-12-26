fn main() {

    let x: i32 = 5;
    {
        let x: i8 = 3;
        println!("x inner is {}", x);
    }

    println!("x outer is {}", x);

    let x = "I'm a string!";

    println!("{}", x);

    println!("\nBorrowing:");
    borrow();
}

fn borrow() {
    let mut x = 10;
    println!("x is {}", x);

    {
        let y = &mut x;
        println!("y is {}", y);

        *y += *y * 2;

        println!("now y is {}", y);
    }

    println!("now x is {}", x);
}
