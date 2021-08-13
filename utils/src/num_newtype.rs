use num_traits::Num;
use std::ops::{Deref, Add};

#[derive(Clone, Copy, Debug)]
pub struct MyNum<T: Num + Copy>(T);

impl<T: Num + Copy> From<T> for MyNum<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T: Num + Copy> From<&T> for MyNum<T> {
    fn from(t: &T) -> Self {
        Self(*t)
    }
}

impl<T: Num + Copy> Deref for MyNum<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Num + Copy> PartialEq<MyNum<T>> for MyNum<T> {
    fn eq(&self, other: &MyNum<T>) -> bool {
        self.0 == other.0
    }
}

impl<T: Num + Copy> PartialEq<T> for MyNum<T> {
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

impl<T: Num + Copy> Add<MyNum<T>> for MyNum<T> {
    type Output = MyNum<T>;

    fn add(self, rhs: MyNum<T>) -> Self::Output {
        MyNum::from(self.0 + *rhs)
    }
}

impl<T: Num + Copy> Add<&MyNum<T>> for MyNum<T> {
    type Output = MyNum<T>;

    fn add(self, rhs: &MyNum<T>) -> Self::Output {
        MyNum::from(self.0 + **rhs)
    }
}

impl<T: Num + Copy> Add<T> for MyNum<T> {
    type Output = MyNum<T>;

    fn add(self, rhs: T) -> Self::Output {
        MyNum::from(self.0 + rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_and_eq {
        use super::*;
        use test_case::test_case;

        #[test_case(3i32)]
        #[test_case(3i8)]
        #[test_case(3i128)]
        #[test_case(3u32)]
        #[test_case(3u8)]
        #[test_case(3.0f32)]
        fn from<T: Num + Copy>(val: T) {
            let my_num = MyNum::from(val);

            assert!(my_num == val);
        }
    }

    mod vectors {
        use super::*;

        #[test]
        fn create_vec_from_vec_of_i32() {
            let i32_vec: Vec<i32> = vec![1, 2, 3];

            let my_num_vec: Vec<MyNum<i32>> = i32_vec.iter().map(From::from).collect();
            assert_eq!(my_num_vec, i32_vec)
        }
    }

    mod arithmetics {
        use super::*;

        mod add {
            use super::*;

            #[test]
            fn add_my_num_and_my_num() {
                let one = MyNum::from(1);
                let two = MyNum::from(2);
                let three = MyNum::from(3);

                assert_eq!(one + two, three);
            }

            #[test]
            fn add_my_num_and_i32() {
                let one = MyNum::from(1);
                let two = 2;
                let three = MyNum::from(3);

                assert_eq!(one + two, three);
            }
        }
    }
}

