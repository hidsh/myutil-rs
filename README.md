# myutil-rs
This repo is just my personal utility crate for rust.

## Usage
```shell
cargo new foo-app
cd $_
cargo add --git https://github.com/hidsh/myutil-rs
```

## dp! - (deadly-primitive) debug-print

`dp!` macro that might be useful instead of thousand `println("{}", xxx);`, but I recommend to use another modern debugger instead. 

```terminal
=> x: 10 "i32", @0x7ffc5bdbf9dc
```

That shows that:
- var name
- value
- type
- address

### Example
```rust
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
