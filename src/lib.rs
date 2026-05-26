#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// ------------------------------------------------
pub mod dbg {

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// ------------------------------------------------

#[macro_export]
macro_rules! dp {
    ($val:expr) => {
        println!(
            "=> {}: {:?} {:?}, @{:p}",
            stringify!($val),   // variable name string
            &$val,              // variable value
            std::any::type_name_of_val(&$val),  // type
            $crate::dbg::as_ptr(&$val)               // addr
        );
    };
}

pub fn as_ptr<T: ?Sized>(r: &T) -> *const T {
    r as *const T
}

/*
use std::ptr::addr_of;

fn main() {
    let x = 10;
    let r = &x;
    
    dp!(x); 
    dp!(r); 
    dp!(&r);

    println!("x のアドレス: {:p}", addr_of!(x));
    println!("r の指す先  : {:p}", addr_of!(*r));

}
*/

/*
=> x: 10 "i32", @0x7ffc5bdbf9dc
=> r: 10 "&i32", @0x7ffc5bdbf9e0
=> &r: 10 "&&i32", @0x7ffc5bdbfb60
x のアドレス: 0x7ffc5bdbf9dc
r の指す先  : 0x7ffc5bdbf9dc
*/

// ------------------------------------------------

}
