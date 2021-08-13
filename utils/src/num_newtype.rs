use num_traits::Num;

pub struct MyNum<T: Num>(T);

impl <T: Num> From<T> for MyNum<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

