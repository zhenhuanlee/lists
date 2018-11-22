use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    // Self 是impl后面的类型的 alias
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node { elem: elem, next: mem::replace(&mut self.head, Link::Empty) };
        self.head = Link::More(Box::new(new_node));
    }
}

