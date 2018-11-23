fn main() {
    println!("{}", append_keyboard_word_to_word(String::from("Hello, "), String::from("World!")));
}

fn append_keyboard_word_to_word(word: String, keyboard_word: String) -> String {
    let mut word_with_keyboard_word = String::from(word);
    word_with_keyboard_word.push_str(keyboard_word.as_ref());
    word_with_keyboard_word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_append_keyboard_word_to_word() {
        let word = String::from("Markus");
        let keyboard_word = String::from("qwe");
        
        let expected_appended_word = String::from("Markusqwe");
        assert_eq!(expected_appended_word, append_keyboard_word_to_word(word, keyboard_word));
    }
}