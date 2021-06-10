const VOWELS: [&str; 8] = ["a", "o", "u", "e", "i", "y", "ą", "ę"];

pub fn convert(text: &&str) -> String {
    let mut result = String::new();

    for word in text.split(' ') {
        if starts_with_vowel(&word) {
            result.push_str(&word);
            result.push_str("hay");
        } else {
            result.push_str(&word[1..]);
            result.push_str(&word[0..1]);
            result.push_str("ay");
        }
        result.push(' ');
    }

    result
}

fn starts_with_vowel(word: &str) -> bool {
    VOWELS.contains(&&word[0..1])
}