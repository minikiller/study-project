#![allow(dead_code)]
#![warn(unused_variables)]
// #[cfg(feature = "dead_code")]

pub struct Address(i32, i32, i32);

#[derive(Debug)]
pub struct User<'a> {
    id: i32,
    pub name: &'a str,
    pub sex: bool,
}

impl User<'_> {
    pub fn new(par: &str) -> Option<Self> {
        match par {
            "hello" => None,
            _ => Some(Self {
                id: 100,
                name: "sunlf",
                sex: true,
            }),
        }
    }
}

impl Default for User<'_> {
    fn default() -> Self {
        Self {
            id: 123,
            name: "Default::default()",
            sex: false,
        }
    }
}
