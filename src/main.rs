mod lib;

use std::mem;

use self::lib::first;

fn main() {
    // first();
    let mut a = String::from("xx");
    second(&mut a);
    println!("{}", a);
}

fn second(a: &mut String) {
    a.push_str("yy");
    mem::replace(a, String::from("zzz"));
    println!("{}", a);
}

fn first() {
    let mut a = first::List::new();
    a.push(2);
    a.push(3);
    a.push(4);
    println!("{:?}", a);
    println!("{:?}", a.pop().unwrap());
    println!("{:?}", a);
}
