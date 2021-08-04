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
