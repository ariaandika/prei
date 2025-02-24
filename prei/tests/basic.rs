#![allow(dead_code)]
use std::{collections::HashMap, sync::Arc};

use prei::{Ts, TsType};

#[derive(Ts)]
struct Basic<'a> {
    ref_str: &'a str,
    refm: &'a mut String,
    nullable: Option<i32>,
    ptr: Arc<Vec<u8>>,
    tup: (f64,bool,HashMap<String,Basic<'a>>)
}

const BASIC: &str = "\
export type Basic = {
  ref_str: string,
  refm: string,
  nullable: number | null,
  ptr: number[],
  tup: [number,boolean,Record<string,Basic>,],
};
";

#[derive(Ts)]
enum Message {
    Call,
    Alias(u8),
    Tup(u8,char,bool),
    Ref(Box<Basic<'static>>),
}

const MESSAGE: &str = "\
export type Message =
  | {
    tag: \"Call\",
    value: null
  } | {
    tag: \"Alias\",
    value: number
  } | {
    tag: \"Tup\",
    value: [number,string,boolean,]
  } | {
    tag: \"Ref\",
    value: Basic
  };
";

#[test]
fn basic() {
    assert_eq!(Basic::gen_type(), BASIC);
    assert_eq!(Message::gen_type(), MESSAGE);
}

