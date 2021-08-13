use num_traits::Num;
use std::ops::Deref;

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

impl<T: Num + Copy> PartialEq<T> for MyNum<T> {
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
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
            assert!(my_num_vec == i32_vec)
        }
    }
}

