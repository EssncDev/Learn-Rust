use rand::Rng;
use std::io;


fn get_input() -> String {
    let mut guess = String::new();
    let base = String::from("Hello World!");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    match guess.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid input. Using default value 0.");
            base
        }
    }
}

fn get_random_numb() -> u8 {
    let numb = rand::thread_rng().gen_range(128..255) as u8; 

    numb
}

fn get_random_numb_char(min:u8, max: u8) -> char {
    let random_numb = rand::thread_rng().gen_range(min..=max) as u8;

    random_numb as char
}

fn add_special_char_to_string(mut str_1: String) -> String {
    let rng_char = get_random_numb_char(0, 64);
    str_1.push(rng_char);

    str_1
}

fn add_rng_to_string(mut str_1: String) -> String {
    let rng = get_random_numb();
    str_1.push_str(&rng.to_string());

    str_1.replace(" ", "_")
}


fn main() {
    let guess = get_input();
    let mut new_string = add_special_char_to_string(guess);
    new_string = add_rng_to_string(new_string);
    println!("{}", new_string);

}
