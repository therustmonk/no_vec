use std::{mem, ptr};

pub trait Stick<T> {
    type Target;

    fn stick(self, item: T) -> Self::Target;
}

pub trait Unstick<T> {
    type Target;

    fn unstick(self) -> (Self::Target, T);
}

pub trait Concrete<T>: Sized {
    fn concrete(self) -> Result<T, Self>;
}

macro_rules! impl_stick_unstick {
    ($from:expr, $to:expr) => {
        impl<T> Stick<T> for [T; $from] {
            type Target = [T; $to];

            fn stick(self, item: T) -> Self::Target {
                unsafe {
                    let mut result: Self::Target = mem::uninitialized();
                    ptr::copy(
                        self.as_ptr(),
                        result.as_mut_ptr(),
                        $from,
                    );
                    result[$from] = item;
                    result
                }
            }
        }

        impl<T> Unstick<T> for [T; $to] {
            type Target = [T; $from];

            fn unstick(mut self) -> (Self::Target, T) {
                unsafe {
                    let mut result: Self::Target = mem::uninitialized();
                    ptr::copy(
                        self.as_ptr(),
                        result.as_mut_ptr(),
                        $from,
                    );
                    let mut item: T = mem::uninitialized();
                    ptr::swap(&mut item, &mut self[$from]);
                    (result, item)
                }
            }
        }

        impl<T> Concrete<[T; $from]> for Vec<T> {
            fn concrete(self) -> Result<[T; $from], Self> {
                if self.len() == $from {
                    unsafe {
                        let mut result: [T; $from] = mem::uninitialized();
                        ptr::copy(self.as_ptr(), result.as_mut_ptr(), $from);
                        drop(self);
                        Ok(result)
                    }
                } else {
                    Err(self)
                }
            }
        }
    };
}

macro_rules! impl_stick_unstick_all {
    () => { };
    ($last:expr ,) => { };
    ($from:expr , $to:expr , $($qnt:expr,)*) => {
        impl_stick_unstick!($from, $to);
        impl_stick_unstick_all!($to, $( $qnt, )*);
    };
}

impl_stick_unstick_all!(
    1, 2, 3, 4, 5, 6, 7, 8, 9,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
    30, 31, 32,
);

#[cfg(test)]
mod tests {
    use {Concrete, Stick, Unstick};

    #[test]
    fn test_stick_and_unstick() {
        let arr = [123u16].stick(321);
        assert_eq!(arr, [123, 321]);

        let arr = arr.stick(999);
        assert_eq!(arr, [123, 321, 999]);

        let vec = Box::new(arr).to_vec();
        let arr: [u16; 3] = vec.concrete().unwrap();

        let (arr, item) = arr.unstick();
        assert_eq!(arr, [123, 321]);
        assert_eq!(item, 999);

        let (arr, item) = arr.unstick();
        assert_eq!(arr, [123]);
        assert_eq!(item, 321);
    }
}
