use std::mem;

#[derive(Debug)]
pub struct List {
    pub head: Link,
}

#[derive(Debug)]
pub enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
pub struct Node {
    elem: i32,
    next: Link,
}

impl List {
    // Self 是impl后面的类型的 alias
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let foo = mem::replace(&mut self.head, Link::Empty);
        let new_node = Node { elem: elem, next: foo };
        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                // pull the whole node out of the box
                let new_node = *node;
                self.head = new_node.next;
                Some(new_node.elem)
            },
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(Some(3), list.pop());
        assert_eq!(Some(2), list.pop());
    }
}





