use std::{mem, ptr};

pub trait Stick<T> {
    type Target;

    fn stick(self, item: T) -> Self::Target;
}

impl<T> Stick<T> for [T; 1] {
    type Target = [T; 2];

    fn stick(self, item: T) -> Self::Target {
        unsafe {
            let mut result: Self::Target = mem::uninitialized();
            let len_and_last = self.len();
            ptr::copy(
                self.as_ptr(),
                result.as_mut_ptr(),
                len_and_last,
            );
            result[len_and_last] = item;
            result
        }
    }
}

pub trait Unstick<T> {
    type Target;

    fn unstick(self) -> (Self::Target, T);
}

impl<T> Unstick<T> for [T; 2] {
    type Target = [T; 1];

    fn unstick(mut self) -> (Self::Target, T) {
        unsafe {
            let mut result: Self::Target = mem::uninitialized();
            let len_and_last = result.len();
            ptr::copy(
                self.as_ptr(),
                result.as_mut_ptr(),
                len_and_last,
            );
            let mut item: T = mem::uninitialized();
            ptr::swap(&mut item, &mut self[len_and_last]);
            (result, item)
        }
    }
}

#[cfg(test)]
mod tests {
    use {Stick, Unstick};

    #[test]
    fn it_works() {
        let arr: [u8; 2] = [123].stick(321);
        assert_eq!(arr, [123, 321]);
        let pair = arr.unstick();
        assert_eq!(pair, ([123], 321));
    }
}
