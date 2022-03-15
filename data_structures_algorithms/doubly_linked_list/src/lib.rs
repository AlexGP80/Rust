use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone, PartialEq)]
struct Node {
    value: String,
    prev: Link,
    next: Link,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }))
    }
}

#[derive(Debug, Clone)]
pub struct BetterTransactionLog {
    head: Link,
    tail: Link,
    pub length: u64,
}

impl BetterTransactionLog {
    pub fn new_empty() -> BetterTransactionLog {
        BetterTransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new_node = Node::new(value);
        match self.tail.take() {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            Some(node) => {
                node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(node.clone());
                self.tail = Some(new_node);
            }
        }
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<String> {
        match self.head.take() {
            None => None,
            Some(node) => {
                let mut node = node.borrow_mut();
                self.head = node.next.take();
                if self.head == None {
                    self.tail.take();
                }
                if let Some(next_node) = &self.head {
                    next_node.borrow_mut().prev = None;
                }
                self.length -= 1;
                Some(node.value.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let transaction_log = BetterTransactionLog::new_empty();
        assert_eq!(transaction_log.head, None);
        assert_eq!(transaction_log.tail, None);
        assert_eq!(transaction_log.length, 0);
    }

    #[test]
    fn test_append_to_empty() {
        let mut transaction_log = BetterTransactionLog::new_empty();
        transaction_log.append("hola".to_string());
        assert_ne!(transaction_log.head, None);
        assert_ne!(transaction_log.tail, None);
        assert_eq!(transaction_log.head, transaction_log.tail);
    }

    #[test]
    fn test_append() {
        let mut transaction_log = BetterTransactionLog::new_empty();
        transaction_log.append("1".to_string());
        transaction_log.append("2".to_string());
        transaction_log.append("3".to_string());
        transaction_log.append("4".to_string());
        transaction_log.append("5".to_string());

        assert_ne!(transaction_log.head, None);
        assert_ne!(transaction_log.tail, None);
        assert_eq!(transaction_log.length, 5);
        assert_eq!(
            transaction_log.head.take().unwrap().borrow().value,
            "1".to_string()
        );
        assert_eq!(
            transaction_log.tail.take().unwrap().borrow().value,
            "5".to_string()
        );
    }

    #[test]
    fn test_pop() {
        let mut transaction_log = BetterTransactionLog::new_empty();
        transaction_log.append("1".to_string());
        transaction_log.append("2".to_string());
        transaction_log.append("3".to_string());
        transaction_log.append("4".to_string());
        transaction_log.append("5".to_string());

        assert_eq!(transaction_log.pop(), Some("1".to_string()));
        assert_eq!(transaction_log.pop(), Some("2".to_string()));
        assert_eq!(transaction_log.pop(), Some("3".to_string()));
        assert_eq!(transaction_log.pop(), Some("4".to_string()));
        assert_eq!(transaction_log.pop(), Some("5".to_string()));
        assert_eq!(transaction_log.length, 0);
        assert_eq!(transaction_log.pop(), None);
        assert_eq!(transaction_log.pop(), None);
        assert_eq!(transaction_log.pop(), None);
        assert_eq!(transaction_log.head, None);
        assert_eq!(transaction_log.tail, None);
        assert_eq!(transaction_log.head, transaction_log.tail);
        transaction_log.append("5".to_string());
        transaction_log.append("4".to_string());
        transaction_log.append("3".to_string());
        transaction_log.append("2".to_string());
        transaction_log.append("1".to_string());
        assert_eq!(transaction_log.pop(), Some("5".to_string()));
        assert_eq!(transaction_log.pop(), Some("4".to_string()));
        assert_eq!(transaction_log.pop(), Some("3".to_string()));
        assert_eq!(transaction_log.pop(), Some("2".to_string()));
        assert_eq!(transaction_log.pop(), Some("1".to_string()));
        assert_eq!(transaction_log.length, 0);
        assert_eq!(transaction_log.pop(), None);
        assert_eq!(transaction_log.pop(), None);
        assert_eq!(transaction_log.pop(), None);
        assert_eq!(transaction_log.head, None);
        assert_eq!(transaction_log.tail, None);
        assert_eq!(transaction_log.head, transaction_log.tail);
    }

    #[test]
    fn test_overflow_when_drop_many_items() {
        let mut transaction_log = BetterTransactionLog::new_empty();
        for i in 1..1048576 {
            transaction_log.append(i.to_string());
        }
        // println!("{:?}", transaction_log.head);
    }
}
