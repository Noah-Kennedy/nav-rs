pub struct ListNode<T> {
    value: T,
    next: Option<Box<ListNode<T>>>
}

impl<T> ListNode<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: None,
        }
    }

    pub fn set_next(&mut self, next: ListNode<T>) {
        self.next = Some(Box::new(next));
    }

    pub fn get_next(&self) -> &Option<Box<ListNode<T>>> {
        &self.next
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
}
