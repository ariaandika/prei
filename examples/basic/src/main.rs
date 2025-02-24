#![allow(dead_code)]
use prei::Ts;

#[derive(Ts)]
struct OrderId(u64);

#[derive(Ts)]
struct User {
    id: u64,
    name: String,
}

#[derive(Ts)]
enum Event {
    Navigate(String),
    Message {
        user_id: u64,
        message: String,
    },
    Exit,
}

const OUTPUT: &str = "\
export type OrderId = number;
export type User = {
  id: number,
  name: string,
};
export type Event =
  | {
    tag: \"Navigate\",
    value: string
  } | {
    tag: \"Message\",
    value: {
      user_id: number,
      message: string,
    }
  } | {
    tag: \"Exit\",
    value: null
  };
";

fn main() {
    let result = prei::generate_type!(OrderId,User,Event);
    assert_eq!(result,OUTPUT);
    println!("{result}");
}


