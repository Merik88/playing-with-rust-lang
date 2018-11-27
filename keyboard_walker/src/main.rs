use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let keyboard_word =  String::from(" World!");
    println!("{:?}", append_keyboard_word_to_list_of_words(args, keyboard_word));
}

fn append_keyboard_word_to_word(word: String, keyboard_word: String) -> String {
    let mut word_with_keyboard_word = String::from(word);
    word_with_keyboard_word.push_str(keyboard_word.as_ref());
    word_with_keyboard_word
}

fn append_keyboard_word_to_list_of_words(words: Vec<String>, keyboard_word: String) -> Vec<String> {
    let mut new_words: Vec<String> = vec![];
    for word in words.iter() {
        new_words.push(append_keyboard_word_to_word(word.to_string(), keyboard_word.to_string()));
    }
    new_words
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
    
    #[test]
    fn should_append_keyboard_word_to_a_list_of_words() {
        let words = vec![String::from("Markus"), String::from("Mackan"), String::from("Mackis")];
        let keyboard_word = String::from("qwe");
        
        let expected_appended_word = vec![String::from("Markusqwe"), String::from("Mackanqwe"), String::from("Mackisqwe")];
        assert_eq!(expected_appended_word, append_keyboard_word_to_list_of_words(words, keyboard_word));
    }
}