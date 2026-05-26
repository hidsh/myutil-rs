# myutil-rs

## Usage
```
cargo new foo-app
cd $_
cargo add --git https://github.com/hidsh/myutil-rs
```

## dp! - (deadly-primitive) debug-print

`dp!` macro that may be used instead of `println("{}", xxx);`, but I recommend to use modern debugger instead. 

```terminal
=> x: 10 "i32", @0x7ffc5bdbf9dc
```

That shows that:
- var name
- value
- type
- address


### example
```rust:src/main.rs
use myutil::dp;

fn main() {
    let x = 10;
    let r = &x;

    dp!(x);
    dp!(r);
    dp!(&r);
}
```

```terminal
=> x: 10 "i32", @0x7ffc5bdbf9dc
=> r: 10 "&i32", @0x7ffc5bdbf9e0
=> &r: 10 "&&i32", @0x7ffc5bdbfb60
```
