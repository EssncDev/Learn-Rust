struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {

    let email = String::from("examplemail@gmx.com");
    let username = String::from("rgh");
    let mut user1 = build_user(email, username);
    println!("Mail before chainge: {}", user1.email);

    user1.email = String::from("anothermail@gmail.com");
    println!("Mail after change: {}", user1.email);

    

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }

}
