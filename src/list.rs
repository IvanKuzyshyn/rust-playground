use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    value: T,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Display> Node<T> {
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

impl<T: Display> DoubleLinkedList<T> {
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

    pub fn add_last(&mut self, value: T) {
        let new_node = Node::new(value);

        match self.tail.take() {
            Some(old_tail) => {
                new_node.borrow_mut().prev = Some(Rc::clone(&old_tail));
                old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                self.tail = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }

        self.size += 1;
    }

    pub fn remove_first(&mut self) -> Option<T> {
        let first = self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
        });

        self.size -= 1;

        first
    }

    pub fn remove_last(&mut self) -> Option<T> {
        let last = self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None;
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
        });

        self.size -= 1;

        last
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.tail = None;
        self.size = 0;
    }

    pub fn get_size(&self) -> i16 {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

pub fn run() {
    println!("Testing list:");
    let mut list = DoubleLinkedList::new();

    list.add_first("Third");
    list.add_first("Second");
    list.add_first("First");
    list.add_last("Forth");
    list.add_last("Fifth");

    assert_eq!(list.get_size(), 5);

    list.remove_first();
    assert_eq!(list.get_size(), 4);

    list.remove_last();
    assert_eq!(list.get_size(), 3);

    list.clear();
    assert_eq!(list.get_size(), 0);
    assert_eq!(list.is_empty(), true);
}
