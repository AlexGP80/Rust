use std::cell::RefCell;
use std::rc::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug, PartialEq)]
struct Node {
    value: String,
    next: SingleLink,
}

impl Node {
    // A nice and short way of creating a new Node
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

#[derive(Debug)]
struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(old_node) => old_node.borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone()),
        };
        self.length += 1;
        self.tail = Some(new_node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let transaction_log = TransactionLog::new_empty();
        assert_eq!(transaction_log.head, None);
        assert_eq!(transaction_log.tail, None);
        assert_eq!(transaction_log.length, 0);
    }

    #[test]
    fn test_append_to_empty() {
        let mut transaction_log = TransactionLog::new_empty();
        transaction_log.append("hola".to_string());
        println!("{:?}", transaction_log);
        assert_ne!(transaction_log.head, None);
        assert_ne!(transaction_log.tail, None);
        assert_eq!(transaction_log.head, transaction_log.tail);
    }
}
