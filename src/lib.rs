/// This crate contains methods for modifying arrays and converting
/// them to vectors and back.

use std::ptr;
use std::mem::MaybeUninit;

/// This trait helps to join array with a new element.
pub trait Stick<T> {
    type Target;

    /// Appends an item to an array:
    /// ```rust
    /// # extern crate no_vec;
    /// # use no_vec::Stick;
    /// let arr: [u16; 2] = [123u16].stick(456);
    /// ```
    fn stick(self, item: T) -> Self::Target;
}

/// This trait helps to remove an item element from sized array.
pub trait Unstick<T> {
    type Target;

    /// Removes an item from an array:
    /// ```rust
    /// # extern crate no_vec;
    /// # use no_vec::Unstick;
    /// let (arr, item): ([u16; 1], u16) = [123u16, 456].unstick();
    /// ```
    fn unstick(self) -> (Self::Target, T);
}

/// Helps to covert `Vec<T>` to `[T]`.
pub trait Concrete<T>: Sized {
    /// Converts from a vector to an array:
    /// ```rust
    /// # extern crate no_vec;
    /// # use no_vec::Concrete;
    /// let arr: Result<[u16; 2], Vec<u16>> = vec![123u16, 456].concrete();
    /// ```
    fn concrete(self) -> Result<T, Self>;
}

/// Helps to covert `[T]` into `Vec<T>`.
pub trait Melt<T> {

    /// Converts from an array to a vector:
    /// ```rust
    /// # extern crate no_vec;
    /// # use no_vec::Melt;
    /// let vec: Vec<u16> = [123u16, 456].melt();
    /// ```
    fn melt(self) -> Vec<T>;
}

macro_rules! impl_stick_unstick {
    ($from:expr, $to:expr) => {
        impl<T> Stick<T> for [T; $from] {
            type Target = [T; $to];

            fn stick(self, item: T) -> Self::Target {
                unsafe {
                    let mut result: Self::Target = MaybeUninit::uninit().assume_init();
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
                    let mut result: Self::Target = MaybeUninit::uninit().assume_init();
                    ptr::copy(
                        self.as_ptr(),
                        result.as_mut_ptr(),
                        $from,
                    );
                    let mut item: T = MaybeUninit::uninit().assume_init();
                    ptr::swap(&mut item, &mut self[$from]);
                    (result, item)
                }
            }
        }

        impl<T> Concrete<[T; $from]> for Vec<T> {
            fn concrete(self) -> Result<[T; $from], Self> {
                if self.len() == $from {
                    unsafe {
                        let mut result: [T; $from] = MaybeUninit::uninit().assume_init();
                        ptr::copy(self.as_ptr(), result.as_mut_ptr(), $from);
                        drop(self);
                        Ok(result)
                    }
                } else {
                    Err(self)
                }
            }
        }

        impl<T> Melt<T> for [T; $from] {
            fn melt(self) -> Vec<T> {
                let boxed: Box<[T]> = Box::new(self);
                boxed.into_vec()
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
    use {Concrete, Melt, Stick, Unstick};

    #[test]
    fn test_stick_and_unstick() {
        let arr = [123u16].stick(321);
        assert_eq!(arr, [123, 321]);

        let arr = arr.stick(999);
        assert_eq!(arr, [123, 321, 999]);

        let mut vec: Vec<u16> = arr.melt();
        vec.push(111);
        let arr: [u16; 4] = vec.concrete().unwrap();
        assert_eq!(arr, [123, 321, 999, 111]);

        let (arr, item) = arr.unstick();
        assert_eq!(arr, [123, 321, 999]);
        assert_eq!(item, 111);

        let (arr, item) = arr.unstick();
        assert_eq!(arr, [123, 321]);
        assert_eq!(item, 999);

        let (arr, item) = arr.unstick();
        assert_eq!(arr, [123]);
        assert_eq!(item, 321);
    }
}
