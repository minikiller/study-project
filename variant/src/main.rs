fn main() {
    let tup = (2, "str", "str".to_owned());
    let (a, b, c) = tup.clone();
    println!("{a}{b}{c}");
    println!("{ }{ }{ }", tup.0, tup.1, tup.2);
    let name: (String, u32, Vec<&str>);
    let vd= add_it(18);

}

struct Hello<'a> {
    name: (String, u32, Vec<&'a str>),
}

fn add_it(x: i32) -> i32 {
    x + 5
}
