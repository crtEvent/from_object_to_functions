pub struct FunStack<T> {
    data: *mut T,
    capacity: usize,
}

impl<T> FunStack<T> {
    pub fn new() -> FunStack<T> {
        Self { data: std::ptr::null_mut(), capacity: 0 }
    }

    pub fn push(&self, value: T) -> FunStack<T> {
        let new_capacity = self.capacity + 1;

        let new_data: *mut T = unsafe {
            let new_layout = std::alloc::Layout::array::<T>(new_capacity).unwrap();
            let pointer: *mut T = std::alloc::alloc(new_layout) as *mut T;
            if pointer.is_null() {
                std::alloc::handle_alloc_error(new_layout);
            }
            pointer
        };

        unsafe {
            if !self.data.is_null() {
                std::ptr::copy_nonoverlapping(self.data, new_data, self.capacity);
            }
            std::ptr::write(new_data.add(self.capacity), value);
        }

        FunStack { data: new_data, capacity: new_capacity }
    }

    pub fn pop(&self) -> (Option<T>, FunStack<T>) {
        if self.capacity == 0 {
            return (None, FunStack::new())
        }

        let new_capacity = self.capacity - 1;

        let new_data: *mut T = if new_capacity == 0 {
            std::ptr::null_mut()
        } else {
            unsafe {
                let new_layout = std::alloc::Layout::array::<T>(new_capacity).unwrap();
                let pointer: *mut T = std::alloc::alloc(new_layout) as *mut T;
                if pointer.is_null() {
                    std::alloc::handle_alloc_error(new_layout);
                }
                pointer
            }
        };

        if !new_data.is_null() {
            unsafe {
                std::ptr::copy_nonoverlapping(self.data, new_data, new_capacity);
            }
        }

        let value:T = unsafe {
            std::ptr::read(self.data.add(new_capacity))
        };

        (Some(value), FunStack { data: new_data, capacity: new_capacity })
    }

    pub fn len(&self) -> usize {
        self.capacity
    }
}

impl<T> Drop for FunStack<T> {
    fn drop(&mut self) {
        if !self.data.is_null() {
            let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
            unsafe { std::alloc::dealloc(self.data as *mut u8, layout); }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_into_the_stack() {
        let stack1: FunStack<char> = FunStack::new();
        let stack2 = stack1.push('A');
        assert_eq!(stack1.len(), 0);
        assert_eq!(stack2.len(), 1);
    }

    #[test]
    fn push_push_pop() {
        let (c, stack) = FunStack::new()
            .push('A')
            .push('B')
            .push('C')
            .pop();
        assert_eq!(stack.len(), 2);
        assert_eq!(c.unwrap(), 'C');
    }

    #[test]
    fn push_push_pop_pop_pop() {
        let stack = FunStack::new()
            .push('A')
            .push('B')
            .push('C');
        let (c, stack1) = stack.pop();
        let (b, stack2) = stack1.pop();
        let (a, stack3) = stack2.pop();
        let (none, stack4) = stack3.pop();

        assert_eq!(stack.len(), 3);
        assert_eq!(c.unwrap(), 'C');
        assert_eq!(stack1.len(), 2);
        assert_eq!(b.unwrap(), 'B');
        assert_eq!(stack2.len(), 1);
        assert_eq!(a.unwrap(), 'A');
        assert_eq!(stack3.len(), 0);
        assert_eq!(none, None);
        assert_eq!(stack4. len(), 0);
    }

    #[test]
    fn pop_from_empty_stack() {
        let (value, stack) = FunStack::<char>::new()
            .pop();

        assert_eq!(value, None);
        assert_eq!(stack.len(), 0);
    }

}
