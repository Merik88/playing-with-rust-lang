use std::env;

enum Strategy {
    Horizontal,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let keyboard_word =  vec![String::from(" World!")];
    let new_words = append_keyboard_word_to_list_of_words(args, keyboard_word);
    for new_word in new_words.iter() {
        println!("{}", new_word);
    }
}

fn append_keyboard_word_to_word(word: String, keyboard_word: String) -> String {
    let mut word_with_keyboard_word = String::from(word);
    word_with_keyboard_word.push_str(keyboard_word.as_ref());
    word_with_keyboard_word
}

fn append_keyboard_word_to_list_of_words(words: Vec<String>, keyboard_words: Vec<String>) -> Vec<String> {
    let mut new_words: Vec<String> = vec![];
    for word in words.iter() {
        for keyboard_word in keyboard_words.iter() {
            new_words.push(append_keyboard_word_to_word(word.to_string(), keyboard_word.to_string()));
        }
    }
    new_words
}

fn generate_words_from_keyboard_layout(keyboard_layout: Vec<String>, _strategy: Strategy, word_length: usize) -> Vec<String> {
    let mut generated_words: Vec<String> = Vec::new();
    
    match _strategy {
        Strategy::Horizontal => {
            let first_row_chars_count = keyboard_layout[0].chars().count();
            let iterate_to = first_row_chars_count+1-word_length;

            for i in 0..iterate_to {
                generated_words.push(keyboard_layout[0].chars().skip(i).take(word_length).collect());
            }
        }
    }
    
    generated_words
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
        let keyboard_words = vec![String::from("qwe")];
        
        let expected_appended_word = vec![String::from("Markusqwe"), String::from("Mackanqwe"), String::from("Mackisqwe")];
        assert_eq!(expected_appended_word, append_keyboard_word_to_list_of_words(words, keyboard_words));
    }
    
    #[test]
    fn should_append_a_list_of_keyboard_words_to_a_list_of_words() {
        let words = vec![String::from("Markus"), String::from("Mackan"), String::from("Mackis")];
        let keyboard_words = vec![String::from("qwe"), String::from("123"), String::from("asd")];
        
        let expected_appended_word = vec![
            String::from("Markusqwe"), String::from("Markus123"), String::from("Markusasd"),
            String::from("Mackanqwe"), String::from("Mackan123"), String::from("Mackanasd"),
            String::from("Mackisqwe"), String::from("Mackis123"), String::from("Mackisasd")
        ];
        assert_eq!(expected_appended_word, append_keyboard_word_to_list_of_words(words, keyboard_words));
    }
    
    #[test]
    fn generate_one_word_of_3_char_horizontally_from_keyboard_layout() {
        
        let keyboard_layout = vec!["qwe".to_string()];
        let strategy = Strategy::Horizontal;
        let word_length = 3;
        
        let generated_words = generate_words_from_keyboard_layout(keyboard_layout, strategy, word_length);
        
        assert_eq!(word_length, generated_words[0].chars().count());
        assert_eq!(vec!["qwe".to_string()], generated_words);
    }
    
    #[test]
    fn generate_3_words_of_3_char_horizontally_from_keyboard_layout() {
        
        let keyboard_layout = vec!["qwert".to_string()];
        let strategy = Strategy::Horizontal;
        let word_length = 3;
        
        let generated_words = generate_words_from_keyboard_layout(keyboard_layout, strategy, word_length);
        
        let expected_words = vec!["qwe".to_string(), "wer".to_string(), "ert".to_string()];
        assert!(generated_words.iter().all(|item| item.chars().count() == word_length));
        assert_eq!(expected_words, generated_words);
    }
}