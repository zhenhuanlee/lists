mod lib;

use self::lib::first;

fn main() {
    let mut a = first::List::new();
    a.push(2);
    a.push(3);
    a.push(4);
    println!("{:?}", a);
    println!("{:?}", a.pop().unwrap());
    println!("{:?}", a);
}
