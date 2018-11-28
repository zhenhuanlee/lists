mod lib;

use std::mem;

use self::lib::first;

fn main() {
    // first();
    test_ref();
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

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn test_ref() {
    let c = 'Q';

    let ref m = c;
    let n = &c;
    assert_eq!(m, n);

    let point = Point {x: 0, y: 0};
    let _copy_of_x = {
        let Point {x: ref ref_to_x, y: _} = point;
        *ref_to_x
    };
    assert_eq!(_copy_of_x, 0);

    let mut mut_point = point;
    mut_point.x = 1;
    println!("{:?} - {:?}", point, mut_point);

    let tt = Some(3);
    if let Some(ref x) = tt {
        assert_eq!(*x, 3);
    };
    
}
