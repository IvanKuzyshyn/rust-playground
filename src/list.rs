use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    value: T,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }))
    }
}

#[derive(Debug)]
struct DoubleLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: i16,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn add_first(&mut self, value: T) {
        let new_node = Node::new(value);

        match self.head.take() {
            Some(old_head) => {
                new_node.borrow_mut().next = Some(Rc::clone(&old_head));
                old_head.borrow_mut().prev = Some(Rc::clone(&new_node));
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }

        self.size += 1;
    }

    pub fn add_last(&mut self, value: T) {}

    pub fn remove_first(&mut self, value: T) {}

    pub fn remove_last(&mut self, value: T) {}

    pub fn clear(&mut self, value: T) {}

    pub fn get_size(&self) -> i16 {
        self.size
    }
}

pub fn run() {
    println!("Testing list:");
    let mut list = DoubleLinkedList::new();

    list.add_first("Third");
    list.add_first("Second");
    list.add_first("First");

    assert_eq!(list.get_size(), 3);
    println!("List size {}", list.get_size());
}
