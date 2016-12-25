fn main() {

    let x: i32 = 5;
    {
        let x: i8 = 3;
        println!("x inner is {}", x);
    }

    println!("x outer is {}", x);
}
