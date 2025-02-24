#![allow(dead_code)]
use prei::Ts;

#[derive(Ts)]
struct Primitives {
    num: i32,
    str: String,
    nil: (),
}

#[derive(Ts)]
struct Reference {
    app: Primitives,
}

/// [string,number,boolean]
#[derive(Ts)]
struct Tuple(String,usize,bool);

/// string[]
#[derive(Ts)]
struct Wraped(Vec<String>);

const OUTPUT: &str = "\
export type Primitives = {
  num: number,
  str: string,
  nil: null,
};
export type Reference = {
  app: Primitives,
};
export type Tuple = [string,number,boolean,];
export type Wraped = string[];
";

fn main() {
    let result = prei::generate!(Primitives,Reference,Tuple,Wraped);
    assert_eq!(&result,OUTPUT);
    println!("{result}");
}

