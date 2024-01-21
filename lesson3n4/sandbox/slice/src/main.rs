fn main() {

    let mut s1 = String::from("Hello, world!");
    let s3 = space_index(&s1);

    println!("space index: {} | string till index: '{}'", s3, &s1[..s3]);

    let s4 = pre_string_until_space(&s1);

    println!("pre string until space: '{}'",s4);

    s1.clear();
}

fn space_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // on first space the iteration ends end return the length of the pre-string before
            return i; // i = index
        }
    }

    s.len()
}

fn pre_string_until_space(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}