fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("\nNone value detected!");
            None
        },
        Some(i) => {
            println!("Added one! New result: {:?}", i+1);
            Some(i + 1)
        },
    }
}

fn dice_rool_even(number: u8) -> bool {
    println!("result: {}", number % 2);
    match number % 2 {
        0 => true,
        1 => false,
        _ => unreachable!(),
    }
}

fn main() {
    let five = Some(5);
    println!("{:?}",five);
    let six = plus_one(five);
    let none = plus_one(None);

    println!(" ");

    for i in 1..=6 {
        let result: bool = dice_rool_even(i);
        println!("dice roll even? {}", result)
    }
    
}
