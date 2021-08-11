use serde::Deserialize;
use core::str::FromStr;
use std::ops::Deref;
use std::cmp::Ordering;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize)]
pub struct MyString(String);

impl FromStr for MyString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MyString(s.to_string()))
    }
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for MyString {
    fn from(s: String) -> Self {
        MyString(s)
    }
}

impl From<&String> for MyString {
    fn from(s: &String) -> Self {
        MyString(s.to_owned())
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

impl PartialOrd<String> for MyString {
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<&str> for MyString {
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        self.0.partial_cmp(&other.to_string())
    }
}

#[cfg(test)]
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
            let _ = MyString::from(str);
        }

        #[test]
        fn from_string_ref() {
            let string = String::from("aaa");
            let _ = MyString::from(&string);
        }

        #[test]
        fn from_str_ref() {
            let str = "aaa";
            let _ = MyString::from(&str);
        }
    }

    mod compare {
        use super::*;

        mod eq {
            use super::*;

            #[test]
            fn eq_self_from_str() {
                let first = MyString::from("aaa");
                let second = MyString::from("aaa");

                assert_eq!(first, second);
            }

            #[test]
            fn eq_self_from_ref_to_string() {
                let string = String::from("aaa");
                let first = MyString::from(&string);
                let second = MyString::from(&string);

                assert_eq!(first, second);
            }

            #[test]
            fn eq_self_from_str_and_string() {
                let str = "aaa";
                let string = String::from(str);
                let first = MyString::from(str);
                let second = MyString::from(string);

                assert_eq!(first, second);
            }

            #[test]
            fn eq_str() {
                let str = "aaa";
                let my_string = String::from("aaa");

                assert_eq!(my_string, str);
            }

            #[test]
            fn eq_string() {
                let string = String::from("aaa");
                let my_string = MyString::from("aaa");

                assert_eq!(my_string, string);
            }
        }

        mod ne {
            use super::*;

            #[test]
            fn ne_self_from_str() {
                let first = MyString::from("aaa");
                let second = MyString::from("AAA");

                assert_ne!(first, second);
            }

            #[test]
            fn ne_self_from_string() {
                let first_string = String::from("aaa");
                let first = MyString::from(&first_string);

                let second_string = String::from("AAA");
                let second = MyString::from(second_string);

                assert_ne!(first, second);
            }

            #[test]
            fn ne_self_from_str_and_string() {
                let str = "aaa";
                let string = String::from("BBB");
                let first = MyString::from(str);
                let second = MyString::from(string);

                assert_ne!(first, second);
            }
        }

        mod lt {
            use super::*;

            #[test]
            fn lt_str() {
                let my_string = MyString::from("aaa");
                let str = "bbb";

                assert!(my_string < str);
            }

            #[test]
            fn lt_string() {
                let my_string = MyString::from("aaa");
                let string = String::from("bbb");

                assert!(my_string < string);
            }

            #[test]
            fn lt_self() {
                let first = MyString::from("aaa");
                let second = MyString::from("bbb");

                assert!(first < second);
            }
        }

        mod le {
            use super::*;

            #[test]
            fn le_str() {
                let my_string = MyString::from("aaa");
                let str = "bbb";

                assert!(my_string <= str);
            }

            #[test]
            fn le_string() {
                let my_string = MyString::from("aaa");
                let string = String::from("bbb");

                assert!(my_string <= string);
            }

            #[test]
            fn le_self() {
                let first = MyString::from("aaa");
                let second = MyString::from("bbb");

                assert!(first <= second);
            }
        }

        mod gt {
            use super::*;

            #[test]
            fn le_str() {
                let my_string = MyString::from("bbb");
                let str = "aaa";

                assert!(my_string > str);
            }

            #[test]
            fn le_string() {
                let my_string = MyString::from("bbb");
                let string = String::from("aaa");

                assert!(my_string > string);
            }

            #[test]
            fn le_self() {
                let first = MyString::from("bbb");
                let second = MyString::from("aaa");

                assert!(first > second);
            }
        }

        mod ge {
            use super::*;

            #[test]
            fn ge_str() {
                let my_string = MyString::from("bbb");
                let str = "aaa";

                assert!(my_string >= str);
            }

            #[test]
            fn ge_string() {
                let my_string = MyString::from("bbb");
                let string = String::from("aaa");

                assert!(my_string >= string);
            }

            #[test]
            fn ge_self() {
                let first = MyString::from("bbb");
                let second = MyString::from("bbb");

                assert!(first >= second);
            }
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

        #[test]
        fn vector_of_newtypes_from_vector_of_strings() {
            let vec_of_strings = vec![String::from("aaa"), String::from("bbb")];
            let _ = Vec::from(vec_of_strings);
        }

        #[test]
        fn vector_of_newtypes_from_vector_of_refs_to_string() {
            let string_a = String::from("aaa");
            let string_b = String::from("bbb");

            let vec_of_refs = vec![&string_a, &string_b];
            let _: Vec<MyString> = vec_of_refs.iter().map(|&e| MyString::from(e)).collect();
        }

        #[test]
        fn vector_of_newtypes_from_vector_of_slices() {
            let vec_of_slices = vec!["aaa", "bbb"];
            let _: Vec<MyString> = vec_of_slices.iter().map(|&e| MyString::from(e)).collect();
        }

        #[test]
        fn vector_of_newtypes_from_vector_of_refs_to_slices() {
            let vec_of_refs = vec![&"aaa", &"bbb"];
            let _: Vec<MyString> = vec_of_refs.iter().map(|&e| MyString::from(e)).collect();
        }

        #[test]
        fn vector_of_newtypes_from_array_of_strs() {
            let array = ["aaa", "bbb", "ccc"];
            let _: Vec<MyString> = array.iter().map(|&e| MyString::from(e)).collect();
        }

        #[test]
        fn vector_of_newtypes_from_array_of_refs_to_strings() {
            let string_a = String::from("aaa");
            let string_b = String::from("bbb");

            let array = [&string_a, &string_b];
            let _: Vec<MyString> = array.iter().map(|&e| MyString::from(e)).collect();
        }

        #[test]
        fn vector_of_newtypes_from_array_of_refs_to_strs() {
            let str_a = "aaa";
            let str_b = "bbb";

            let array = [&str_a, &str_b];
            let _: Vec<MyString> = array.iter().map(|&e| MyString::from(e)).collect();
        }

        #[test]
        fn vector_of_newtypes_from_reference_to_vector_of_strings() {
            let string_a = String::from("aaa");
            let string_b = String::from("bbb");

            let ref_to_vec = &vec![string_a, string_b];
            let _: Vec<MyString> = ref_to_vec.iter().map(|e| MyString::from(e)).collect();
        }
    }
}
