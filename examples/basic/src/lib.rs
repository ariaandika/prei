#![allow(dead_code)]
use prei::Ts;

#[derive(Ts)]
pub struct App {
    pub id: i32,
    pub name: String,
    pub none: (),
}

#[derive(Ts)]
pub struct Deez {
    pub id: i32,
    pub name: String,
    pub app: App,
}

#[derive(Ts)]
pub struct Arr(String,usize,bool);

#[derive(Ts)]
pub struct Alias(Vec<String>);

