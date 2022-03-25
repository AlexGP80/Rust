use doubly_linked_list::DoublyLinkedList;

fn main() {
    let mut doubly_linked_list = DoublyLinkedList::new_empty();
    for i in 1..1000 {
        doubly_linked_list.append(i.to_string());
    }
    // println!("{:?}", doubly_linked_list);
}
