mod AnotherFile;
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
AnotherFile::init();
}