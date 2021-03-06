pub mod linked_list;

pub trait List<'a, E> {
    fn insert(&mut self, element: E, index: u64) -> bool;
    fn get(&self, index: u64) -> Option<E>;
    fn set(&mut self, index: u64, new_value: E) -> bool;
    fn remove(&mut self, index: u64) -> bool;
    fn size(&self) -> u64;
}

pub trait Stack<'a, E> {
    fn push(&mut self, element: E) -> bool;
    fn pop(&mut self) -> Option<E>;
    fn peek(&self) -> Option<E>;
}

pub trait Queue<'a, E> {
    fn put(&mut self, element: E) -> bool;
    fn take(&mut self) -> Option<E>;
    fn check(&self) -> Option<E>;
}