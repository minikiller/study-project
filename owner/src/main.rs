fn main() {
    let str = "Hello, world!".to_owned();
    println!("Hello, world!");
    // let b = get_it();
    let b = get_pos(&str);
    println!("{}", b);
}

fn get_it<'a>() -> &'a str {
    let s: &'static str = "Hello";
    &s
}

fn get_pos(str: &str) -> &str {
    let byte = str.as_bytes();
    for (i, item) in byte.into_iter().enumerate() {
        if *item == b' ' {
            return &str[..i];
        }
    }
    &str[..]
}
