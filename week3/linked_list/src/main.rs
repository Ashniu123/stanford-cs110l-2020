use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    let mut match_list: LinkedList<u32> = LinkedList::new();
    let mut unmatch_list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
        match_list.push_front(i);
        if i % 2 == 0 {
            unmatch_list.push_front(i);
        }
    }
    let clone_list = list.clone();
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display
    println!("Cloned list: {}", clone_list); // impl Clone
    println!("Match list: {}", clone_list == match_list); // impl PartialEq
    println!("Unmatch list: {}", clone_list == unmatch_list); // impl PartialEq

    // impl IntoIterator for LinkedList<T>
    let mut other_list = LinkedList::new();
    for mut val in list {
        val += 1;
        other_list.push_front(val);
    }

    // impl Iterator for &LinkedList<T>
    for val in &other_list {
        println!("{}", val);
    }
}
