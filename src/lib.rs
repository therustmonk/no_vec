use std::{mem, ptr};

pub trait Stick<T> {
    type Target;

    fn stick(self, item: T) -> Self::Target;
}

pub trait Unstick<T> {
    type Target;

    fn unstick(self) -> (Self::Target, T);
}

macro_rules! impl_stick_unstick {
    ($from:expr, $to:expr) => {
        impl<T> Stick<T> for [T; $from] {
            type Target = [T; $to];

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

        impl<T> Unstick<T> for [T; $to] {
            type Target = [T; $from];

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
    use {Stick, Unstick};

    #[test]
    fn test_stick_and_unstick() {
        let arr = [123].stick(321);
        assert_eq!(arr, [123, 321]);

        let arr = arr.stick(999);
        assert_eq!(arr, [123, 321, 999]);

        let pair = arr.unstick();
        assert_eq!(pair, ([123, 321], 999));

        let pair = pair.0.unstick();
        assert_eq!(pair, ([123], 321));
    }
}
