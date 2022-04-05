use chrono::{prelude::*, format::{DelayedFormat, StrftimeItems}};
fn main() {
    //
    let fmt = "%m%d";
    // let fmt = "%Y-%m-%d %H:%M:%S";
    //
    let now: DateTime<Local> = Local::now();
    let dft: DelayedFormat<StrftimeItems> = now.format(fmt);
    let str_date: String = dft.to_string(); // 2021-01-04 20:02:09
    println!("now: {}", str_date);
}
