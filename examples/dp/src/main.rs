use myutil::dp;

fn main() {
    let x = 10;
    let r = &x;
    
    dp!(x); 
    dp!(r); 
    dp!(&r);
}

/*
=> x: 10 "i32", @0x7ffc5bdbf9dc
=> r: 10 "&i32", @0x7ffc5bdbf9e0
=> &r: 10 "&&i32", @0x7ffc5bdbfb60
*/
