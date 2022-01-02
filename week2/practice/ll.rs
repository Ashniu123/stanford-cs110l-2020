use std::fmt;

struct LinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

struct Node {
    value: u32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: u32, next: Option<Box<Node>>) -> Node {
        Node { value, next }
    }
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }

    pub fn push(&mut self, value: u32) {
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<u32> {
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }

    // pub fn display(&self) {
    //     let mut current = &self.head;
    //     let mut result = String::new();
    //     while let Some(node) = current {
    //         result = format!("{} {}", result, &node.value);
    //         current = &node.next;
    //     }
    //     println!("{}", result);
    // }
}

impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;
        let mut result = String::new();
        while let Some(node) = current {
            result = format!("{} {}", result, &node.value);
            current = &node.next;
        }
        write!(f, "{}", result)
    }
}

// https://stackoverflow.com/questions/38147453/do-we-need-to-manually-create-a-destructor-for-a-linked-list
impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

fn main() {
    let mut ll = LinkedList::new();
    assert!(ll.is_empty());
    assert_eq!(ll.pop(), None);
    ll.push(11);
    ll.push(21);
    ll.push(22);
    ll.push(31);
    println!("{}", ll);
    assert_eq!(ll.get_size(), 4);
    assert!(!ll.is_empty());
    assert_eq!(ll.pop(), Some(31));
    println!("{}", ll);
}
