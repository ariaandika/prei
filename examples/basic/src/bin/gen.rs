use basic::{Alias, App, Arr, Deez};
use prei::Interface;


fn main() {
    let mut result = App::generate();
    result += &Deez::generate();
    result += &Arr::generate();
    result += &Alias::generate();
    println!("{result}");
}

