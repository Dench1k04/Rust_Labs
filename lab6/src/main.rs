use std::ptr;

struct Node {
    data: i32,
    next: *mut Node,
    prev: *mut Node,
}

struct DoublyLinkedList {
    head: *mut Node,
    tail: *mut Node,
}

impl DoublyLinkedList {
    fn new() -> Self {
        DoublyLinkedList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    fn push_back(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next: ptr::null_mut(),
            prev: self.tail,
        });
        let new_node_ptr = Box::into_raw(new_node);

        if self.tail.is_null() {
            self.head = new_node_ptr;
            self.tail = new_node_ptr;
        } else {
            unsafe {
                (*self.tail).next = new_node_ptr;
                self.tail = new_node_ptr;
            }
        }
    }

    fn remove(&mut self, data: i32) {
        let mut current = self.head;
        while !current.is_null() {
            unsafe {
                if (*current).data == data {
                    if current == self.head {
                        self.head = (*current).next;
                        if !self.head.is_null() {
                            (*self.head).prev = ptr::null_mut();
                        } else {
                            self.tail = ptr::null_mut();
                        }
                    } else if current == self.tail {
                        self.tail = (*current).prev;
                        (*self.tail).next = ptr::null_mut();
                    } else {
                        let prev = (*current).prev;
                        let next = (*current).next;
                        (*prev).next = next;
                        (*next).prev = prev;
                    }
                    let _ = Box::from_raw(current);
                    return;
                }
                current = (*current).next;
            }
        }
    }

    fn print(&self) {
        let mut current = self.head;
        print!("Список: ");
        while !current.is_null() {
            unsafe {
                print!("{} ", (*current).data);
                current = (*current).next;
            }
        }
        println!();
    }
}

impl Drop for DoublyLinkedList {
    fn drop(&mut self) {
        let mut current = self.head;
        while !current.is_null() {
            unsafe {
                let next = (*current).next;
                let _ = Box::from_raw(current);
                current = next;
            }
        }
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    println!("Додавання 1");
    list.push_back(1);
    list.print();
    println!("Додавання 2");
    list.push_back(2);
    list.print();
    println!("Додавання 3");
    list.push_back(3);
    list.print();
    println!("Додавання 4");
    list.push_back(4);
    list.print();
    println!("Видалення 2");
    list.remove(2);
    list.print();
    println!("Видалення 1");
    list.remove(1);
    list.print();
    println!("Повторення видалення 1");
    list.remove(1);
    list.print();
    println!("Додавання 5");
    list.push_back(5);
    list.print();
}