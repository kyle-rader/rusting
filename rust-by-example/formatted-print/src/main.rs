


fn main() {
    println!("Hello, formatted print!");

    println!("Arg 1:{} and arg 2:{}", "first", "second");

    println!("Args can be named: {first} and {second}", first=1, second="two");

    println!("We can fmt binary! {:b}", 5);

    println!("We can align text with spaces or zeros: {number:>width$}, {number:>0width$}",
             number=3,
             width=6);

    let pi: f32 = 3.14159;

    let pi_3 = format!("{:.3}", pi);
    let pi_5 = format!("{:.5}", pi);

    println!("Pi to 3 places: {} and to 5 places: {}", pi_3, pi_5);
}
