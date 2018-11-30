use std::rc::Rc;

#[derive(Debug)]
 pub struct List<T> {
     head: Link<T>,
 }

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node )}
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|n| &**n);
            &node.elem
        })
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn append(&self, elem: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            }))
        }
        // self.head = Some(Rc::new(Node{
        //     elem: elem,
        //     next: self.head.clone(),
        // }));
        // self
    }

    pub fn tail(&self) -> Self {
        List { head: self.head.as_ref().and_then(|node| { node.next.clone() }) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| { &node.elem })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().append(1).append(2).append(3);
        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}