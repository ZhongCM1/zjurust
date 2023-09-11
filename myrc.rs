use std::cell::RefCell;

struct Data<T> {
    data: T,
    count: usize,
}

impl<T> Data<T> {
    fn new(x: T) -> Data<T> {
        Data {
            data: x,
            count: 1,
        }
    }
}

pub struct MyRc< T> {
    ptr: RefCell<Data<T>>,
}

impl<T> MyRc<T> {
    pub fn new(data: T) -> Self {
        let data1 = Data::new(data);
        MyRc {
            ptr: RefCell::new(data1),
        }
    }

    pub fn clone(&self) -> Self {
        self.ptr.borrow_mut().count += 1;
        MyRc { ptr: self.ptr, }
    }

    pub fn strong_count(&self) -> usize {
        self.ptr.borrow().count
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        unsafe {
            self.ptr.borrow_mut().count -= 1;
            if self.ptr.borrow_mut().count == 0 {
                drop(*self.ptr.borrow_mut());
            }
        }
    }
}

impl<T> std::ops::Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &(*(self.ptr.as_ptr())).data }
    }
}
