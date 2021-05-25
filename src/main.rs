fn main() {
    let mut x = 5;
    println!("Value of X: {}", x);
    x = 6;
    println!("Value of X: {}", x);

    const MAX_VALUE: u32 = 100_000;
    println!("Constant value: {}", MAX_VALUE);

    // Shadowing

    let x = 5;
    let x = x + 5;
    let x = x * 2;
    println!("X value is: {}", x);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("Spaces: {}", spaces);

    let mut other = "      ";
    other = other.len(); // This cause an error, because 'other' is a String mutable variable.
}
