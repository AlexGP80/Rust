use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub value: String,
    previous: Link,
    next: Link,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            previous: None,
            next: None,
        }))
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.previous.take();
        self.next.take();
        println!("Dropping {}", &self.value);
        drop(&self.value);
    }
}

#[derive(Debug)]
pub struct DoublyLinkedList {
    head: Link,
    tail: Link,
    pub length: u64,
}


impl DoublyLinkedList {
    pub fn new_empty() -> DoublyLinkedList {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new_node = Node::new(value);
        if let Some(node) = &self.tail {
            new_node.borrow_mut().previous = Some(node.clone());
            node.borrow_mut().next = Some(new_node.clone());
        } else {
            self.head = Some(new_node.clone());
        }
        self.tail = Some(new_node.clone());
        self.length += 1;
    }
}


impl Drop for DoublyLinkedList {
    fn drop(&mut self) {
        self.tail = None;
        self.length = 0;
        while let Some(old_head) = self.head.take() {
            self.head = old_head.borrow_mut().next.take();
            if let Some(_) = &self.head {
                self.head.as_ref().unwrap().borrow_mut().previous.take();
            }
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let doubly_linked_list = DoublyLinkedList::new_empty();
        assert_eq!(doubly_linked_list.head, None);
        assert_eq!(doubly_linked_list.tail, None);
        assert_eq!(doubly_linked_list.length, 0);
    }

    #[test]
    fn test_append_to_empty() {
        let mut doubly_linked_list = DoublyLinkedList::new_empty();
        doubly_linked_list.append("hola".to_string());
        assert_ne!(doubly_linked_list.head, None);
        assert_ne!(doubly_linked_list.tail, None);
        assert_eq!(doubly_linked_list.head, doubly_linked_list.tail);
    }

    #[test]
    fn test_append() {
        let mut doubly_linked_list = DoublyLinkedList::new_empty();
        doubly_linked_list.append("1".to_string());
        doubly_linked_list.append("2".to_string());
        doubly_linked_list.append("3".to_string());
        doubly_linked_list.append("4".to_string());
        doubly_linked_list.append("5".to_string());

        assert_ne!(doubly_linked_list.head, None);
        assert_ne!(doubly_linked_list.tail, None);
        assert_eq!(doubly_linked_list.length, 5);
        assert_eq!(
            doubly_linked_list.head.take().unwrap().borrow().value,
            "1".to_string()
        );
        assert_eq!(
            doubly_linked_list.tail.take().unwrap().borrow().value,
            "5".to_string()
        );
    }

    // #[test]
    // fn test_pop() {
    //     let mut doubly_linked_list = BetterTransactionLog::new_empty();
    //     doubly_linked_list.append("1".to_string());
    //     doubly_linked_list.append("2".to_string());
    //     doubly_linked_list.append("3".to_string());
    //     doubly_linked_list.append("4".to_string());
    //     doubly_linked_list.append("5".to_string());
    //
    //     assert_eq!(doubly_linked_list.pop(), Some("1".to_string()));
    //     assert_eq!(doubly_linked_list.pop(), Some("2".to_string()));
    //     assert_eq!(doubly_linked_list.pop(), Some("3".to_string()));
    //     assert_eq!(doubly_linked_list.pop(), Some("4".to_string()));
    //     assert_eq!(doubly_linked_list.pop(), Some("5".to_string()));
    //     assert_eq!(doubly_linked_list.length, 0);
    //     assert_eq!(doubly_linked_list.pop(), None);
    //     assert_eq!(doubly_linked_list.pop(), None);
    //     assert_eq!(doubly_linked_list.pop(), None);
    //     assert_eq!(doubly_linked_list.head, None);
    //     assert_eq!(doubly_linked_list.tail, None);
    //     assert_eq!(doubly_linked_list.head, doubly_linked_list.tail);
    //     doubly_linked_list.append("5".to_string());
    //     doubly_linked_list.append("4".to_string());
    //     doubly_linked_list.append("3".to_string());
    //     doubly_linked_list.append("2".to_string());
    //     doubly_linked_list.append("1".to_string());
    //     assert_eq!(doubly_linked_list.pop(), Some("5".to_string()));
    //     assert_eq!(doubly_linked_list.pop(), Some("4".to_string()));
    //     assert_eq!(doubly_linked_list.pop(), Some("3".to_string()));
    //     assert_eq!(doubly_linked_list.pop(), Some("2".to_string()));
    //     assert_eq!(doubly_linked_list.pop(), Some("1".to_string()));
    //     assert_eq!(doubly_linked_list.length, 0);
    //     assert_eq!(doubly_linked_list.pop(), None);
    //     assert_eq!(doubly_linked_list.pop(), None);
    //     assert_eq!(doubly_linked_list.pop(), None);
    //     assert_eq!(doubly_linked_list.head, None);
    //     assert_eq!(doubly_linked_list.tail, None);
    //     assert_eq!(doubly_linked_list.head, doubly_linked_list.tail);
    // }
    //
    // #[test]
    // fn test_overflow_when_drop_many_items() {
    //     let mut doubly_linked_list = BetterTransactionLog::new_empty();
    //     for i in 1..1048576 {
    //         doubly_linked_list.append(i.to_string());
    //     }
    //     // println!("{:?}", doubly_linked_list.head);
    // }
}
