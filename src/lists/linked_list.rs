struct ListNode<'a, E> {
    next: & 'a mut Option<ListNode<E>>,
    value: E
}

struct LinkedList<'a, E> {
    head: & 'a mut Option<ListNode<E>>,
    size: u64
}

impl<'a, E> super::List<'a, E> for LinkedList<'a, E> {
    fn insert(&mut self, element: E, index: u64) -> bool {
        unimplemented!()
    }
    
    fn get(&self, index: u64) -> Option<E> {
        unimplemented!()
    }
    
    fn set(&mut self, index: u64, new_value: E) -> bool {
        unimplemented!()
    }
    
    fn remove(&mut self, index: u64) -> bool {
        unimplemented!()
    }
    
    fn size(&self) -> u64 {
        unimplemented!()
    }
}