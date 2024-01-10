fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("3h are {} seconds", THREE_HOURS_IN_SECONDS);

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let guess: u8 = "42".parse().expect("Not a number!");

    println!("result of guess: {}", guess);

    second_function(15);
}

fn second_function(x: u32) {
    println!("Called another function - passing param {}", x);
}