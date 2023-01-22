mod enums;
mod json;

use enums::Cooltime;
use enums::RoughTime;
use enums::TimeUnit;
use json::*;
use std::collections::HashMap;

fn main() {
    let bla = RoughTime::InTheFuture(3, 3);
    println!("Hello, world! {}", bla);

    let bla = Cooltime::LastTime(TimeUnit::Hours, 3);
    println!("Hello, world! {}", bla);

    let wow = Json::Array(Vec::from([
        Json::Boolean(true),
        Json::String(String::from("cool")),
        Json::Object(Box::new(HashMap::from([
            (String::from("a"), Json::String(String::from("wow"))),
            (String::from("b"), Json::Number(33.3)),
            (String::from("oops"), Json::Null),
        ]))),
    ]));
    println!("JSON: {}", wow);
}
