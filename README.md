# Prei

generate typescript type from a struct

## Example

this example can be tested in `examples/basic/src/main.rs`

```bash
cargo run -p basic
```

```rs
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

fn main() {
    let result = prei::generate_type!(OrderId,User,Event);
    println!("{result}");
}
```

output:

```ts
export type OrderId = number;
export type User = {
  id: number,
  name: string,
};
export type Event =
  | {
    tag: "Navigate",
    value: string
  } | {
    tag: "Message",
    value: {
      user_id: number,
      message: string,
    }
  } | {
    tag: "Exit",
    value: null
  };

```

