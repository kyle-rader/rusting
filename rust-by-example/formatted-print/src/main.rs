use std::fmt;


#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// A structure holding 2 numbers `Debug` will be derived so the results can be contrasted with `Display`
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2 {
    x: f64,
    y: f64,
}

// Similarly, implement for Point2
impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{{ x: {}, y: {} }}", self.x, self.y)
    }
}

// My Complex structure
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{real} + {imag}i", real=self.real, imag=self.imag)
    }
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Complex {{ real: {real}, imag: {imag} }}", real=self.real, imag=self.imag)
    }
}

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

    // e, pi, & golden ratio
    let special_nums = vec![2.71828, 3.14159, 1.61803];

    // some iteration over a vector.
    for special_num in &special_nums {
        println!("This is a special number: {val:.3}", val=special_num);
    }

    println!("\nDebugging stuff:");

    println!("Now {:?} is a thing!", Structure(3));

    println!("And Deep: {:?}", Deep(Structure(3)));

    println!("A MinMax: debug: {obj:?}  display: {obj}", obj=MinMax(23, 56));
    println!("A Point : debug: {obj:?}  display: {obj}", obj=Point2 { x: 3.3, y: 56. });
    println!("My Complex struct: {0}  Debugged: {0:?}", Complex { real: 4.6, imag: 8.9 });
}
