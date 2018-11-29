use std::mem;

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

// tuple structs
pub struct IntoIter<T>(List<T>);

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

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop()
    }

}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
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

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1).push(2).push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1).push(2).push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1).push(2).push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }

}