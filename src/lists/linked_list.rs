struct ListNode<'a, E: 'a> {
    next: & 'a mut Option<ListNode<'a, E>>,
    value: E
}

struct LinkedList<'a, E: 'a> {
    head: & 'a mut Option<ListNode<'a, E>>,
    size: & 'a mut u64
}

impl<'a, E> super::List<'a, E> for LinkedList<'a, E> {

    fn insert(&mut self, element: E, index: u64) -> bool {

        let result = true;

        let mut iterator = self.head;

        loop {
            match iterator {
                None => {
                    let new_node = ListNode {
                        next: & mut None,
                        value: element
                    };
                    self.head = &mut Some(new_node);
                },

                Some(next_node) => {
                    iterator = next_node.next;
                }
            }
        }

        true
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