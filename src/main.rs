

fn main() {
    println!("Hello, World");

    let mut x: i32 = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    let a: i32 = 5;
    let a: i32 = a + 1;
    let a: i32 = a * 2;

    println!("The value of a is: {}", a);

    let spaces: &str = " ";
    let spaces: usize = spaces.len();

    println!("The value of spaces is: {}", spaces);

    let b = 2.8;
    let c: f32 = 3.10;

    println!("The value of b is: {} and c is: {}", b, c);
}