fn main() {

    println!("print_number(3)");
    print_number(3);

    println!("countdown(5)");
    countdown(5);

    println!("add_numbers(3, 8)");
    println!("{}", add_numbers(3, 8));
}

fn print_number(num: i32) {
    println!("{}", num);
}

fn countdown(num: i32) {
    println!("{}", num);
    if num > 0 {
        countdown(num-1);
    } else {
        println!("Blast Off!");
    }
}

fn add_numbers(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
