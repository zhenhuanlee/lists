use std::mem;

pub struct List<T> {
    pub head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) -> &mut Self {
        let new_node = Node {
            elem: elem,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node));
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    // pub fn peek(&mut self) -> Option<T> {
    //     self.head.take().map(|node| {
    //         node.elem
    //     })
    // }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn mut_peek(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

mod test {
    use super::List;

    #[test]
    fn ttt() {
        let mut list = List::new();

        list.push('a');
        list.push('b');
        list.push('c');

        assert_eq!(list.pop(), Some('c'));
        assert_eq!(list.pop(), Some('b'));
        assert_eq!(list.pop(), Some('a'));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.mut_peek(), None);
        list.push(1).push(2).push(3);
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.mut_peek(), Some(&mut 3));
    }

}