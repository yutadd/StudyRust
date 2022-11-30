#![allow(unused)]
fn main() {
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
println!("{:?}",five);
let six = plus_one(five);
println!("{:?} should be 6",six);
assert_eq!(six.unwrap_or(6),6);
assert_eq!(six.map(|v| format!("value is {}", v)), Some("value is 1".to_string()));
let none = plus_one(None);
}