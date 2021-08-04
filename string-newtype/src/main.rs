use serde::Deserialize;
use core::str::FromStr;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize)]
struct MyString(String);

impl FromStr for MyString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MyString(s.to_string()))
    }
}

impl From<String> for MyString {
    fn from(s: String) -> Self {
        MyString(s)
    }
}

impl From<&String> for MyString {
    fn from(s: &String) -> Self {
        MyString(s.to_string())
    }
}

impl From<&str> for MyString {
    fn from(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl From<&&str> for MyString {
    fn from(s: &&str) -> Self {
        s.parse().unwrap()
    }
}

impl Into<String> for MyString {
    fn into(self) -> String {
        self.0
    }
}

impl PartialEq<String> for MyString {
    fn eq(&self, other: &String) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<&str> for MyString {
    fn eq(&self, other: &&str) -> bool {
        self.0.eq(other)
    }
}


fn main() {
    println!("Hello, world!");
}

mod tests {
    use super::*;

    mod from {
        use super::*;

        #[test]
        fn from_string() {
            let string = String::from("aaa");
            let _ = MyString::from(string);
        }

        #[test]
        fn from_str() {
            let str = "aaa";
            let my_string = MyString::from(str);

            assert_eq!(my_string, str);
        }

        #[test]
        fn from_string_ref() {
            let string = String::from("aaa");
            let my_string = MyString::from(&string);

            assert_eq!(my_string, string);
        }

        #[test]
        fn from_str_ref() {
            let str = "aaa";
            let my_string = MyString::from(&str);

            assert_eq!(my_string, str);
        }
    }

    mod vectors {
        use super::*;

        #[test]
        fn sort_vector() {
            let mut vector = vec![MyString::from("bbb"), MyString::from("ccc"), MyString::from("aaa")];
            vector.sort();

            let expected = vec![MyString::from("aaa"), MyString::from("bbb"), MyString::from("ccc")];
            assert_eq!(vector, expected);
        }

        #[test]
        fn sort_vector_of_refs() {
            let my_string_a = String::from("aaa");
            let my_string_b = String::from("bbb");
            let my_string_c = String::from("ccc");

            let mut vector = vec![&my_string_c, &my_string_a, &my_string_b];
            vector.sort();

            let expected = vec![&my_string_a, &my_string_b, &my_string_c];
            assert_eq!(vector, expected);
        }
    }
}
