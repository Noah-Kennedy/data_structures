struct ListNode<E> {
    value: E,
    next: &mut Option<ListNode<E>>
}

struct LinkedList<E> {
    head: &mut Option<ListNode<E>>,
    size: u64
}