mod model;
use model::User;
fn main() {
    let user = User::new("hello").unwrap_or_default();
    println!("Hello, world 1! {user:?}");
    // let user1 = User {
    //     name: "test".to_string(),
    //     ..user
    // };
    let mut user2 = User::new("hello").unwrap_or_default();
    user2.name = "123";
    user2.sex = true;
    println!("Hello, world 2! {user2:?}");
}
