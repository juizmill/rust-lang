

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

    // addition
    let sum = 5 + 10;

    println!("The Value of addition is: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of subtraction is: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The value of multiplication is: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of division is: {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);


    let cc = 'z';
    let zz = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("show char: {}, {}, {}", cc, zz, heart_eyed_cat);

}