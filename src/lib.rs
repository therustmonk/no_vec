use std::{mem, ptr};

pub trait Appender {
    type Item;
    type Target;

    fn append(self, item: Self::Item) -> Self::Target;
}

impl<T> Appender for [T; 1] {
    type Item = T;
    type Target = [T; 2];

    fn append(self, item: Self::Item) -> Self::Target {
        unsafe {
            let mut result: Self::Target = mem::uninitialized();
            ptr::copy(
                self.as_ptr() as *const u8,
                result.as_mut_ptr() as *mut u8,
                mem::size_of::<Self>()
            );
            result[self.len()] = item;
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use Appender;

    #[test]
    fn it_works() {
        let arr: [u8; 2] = [123].append(321);
        assert_eq!(arr, [123, 321]);
    }
}
