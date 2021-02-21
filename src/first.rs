use std::mem;

#[derive(Debug)]
pub struct LList<T> {
    elem: T,
    next: Option<Box<LList<T>>>,
}

impl<T> LList<T> {
    pub fn new(elem: T) -> Self {
        Self {
            elem,
            next: None,
        }
    }

    pub fn to_list(vec: Vec<T>) -> Option<Box<LList<T>>>
    where
        T: Copy
    {
        let mut cur = None;
        // & actually derefs
        for &i in vec.iter().rev() {
            let mut next_node = Self::new(i);
            next_node.next = cur;

            cur = Some(Box::new(next_node));
        }

        cur
    }
}

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub struct Node {
    val: i32,
    next: Link,
}

// this version maintains a pointer to head and each `next` will maintain a pointer to the previous
// head
impl List {
    pub fn new() -> Self {
        Self {
            head: Link::Empty
        }
    }

    pub fn push(&mut self, val: i32) {
        let new_node = Node {
            val,
            next: mem::replace(&mut self.head, Link::Empty),
        };

        self.head = Link::More(Box::new(new_node))
    }
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.val)

            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn basics_ll() {
        let list = LList::to_list(vec![1,2,3]).unwrap();

        assert_eq!((*list).elem, 1);
        let next = (*list).next;
        assert_eq!((*next.unwrap()).elem, 2);
    }

    #[test]
    fn basics_ll_string() {
        let list = LList::to_list(vec!["1", "2", "3"]).unwrap();

        assert_eq!((*list).elem, "1");
        let next = (*list).next;
        assert_eq!((*next.unwrap()).elem, "2");
    }

    #[test]
    fn mem_replace() {
        let node = Node {
            val: 31,
            next: Link::Empty,
        };
        let mut l = List {
            head: Link::More(Box::new(node)),
        };

        let x = mem::replace(&mut l.head, Link::Empty);

        if let Link::More(boxed) = x {
            assert_eq!((*boxed).val, 31);
        }
    }
}
