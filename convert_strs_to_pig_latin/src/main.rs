// A program that translates english sentences to Pig-latin.

// The first consonant(s) of each word is moved to the end of the word and ay is added, so first
// becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes
// apple-hay).

fn is_vowel(c: char) -> bool {
    // TODO: Can be improved with pattern matching with `matches!()` macro, for example.
    ['a', 'e', 'i', 'o', 'u'].contains(&c.to_ascii_lowercase())
}

fn translate_to_pig_latin(word: &str) -> String {
    let (v_suffix, c_suffix) = ("-hay", "-ay");
    let mut first_vowel_at: Option<usize> = None;
    let mut has_punctuation = false;
    let last_index: usize;

    if let Some((last_idx, ch)) = word.char_indices().last() {
        has_punctuation = !ch.is_alphanumeric();
        last_index = last_idx;
    } else {
        last_index = word.len() - 1;
    }

    for (byte_index, ch) in word.char_indices() {
        // If it's a vowel
        if is_vowel(ch) {
            first_vowel_at = Some(byte_index);
            break;
        }
    }

    let translated_word: String = match first_vowel_at {
        Some(first_vowel_index) => {
            let start = if has_punctuation {
                &word[first_vowel_index..last_index]
            } else {
                &word[first_vowel_index..]
            };

            let end = if has_punctuation {
                &word[last_index..]
            } else {
                ""
            };
            if first_vowel_index == 0 {
                format!("{start}{v_suffix}{end}")
            } else {
                let mid = word[0..first_vowel_index].to_lowercase();
                format!("{start}{mid}{c_suffix}{end}")
            }
        }
        None => format!("{word}{c_suffix}"),
    };

    translated_word
}

fn main() {
    let s = String::from("This is a sample string. We will convert this string to Pig-Latin.");
    let mut pig_latin_words = String::new();

    println!("Input string: \"{s}\"");
    // println!("{words:?}");
    for word in s.split_whitespace() {
        pig_latin_words.push_str(&translate_to_pig_latin(word));
        pig_latin_words.push(' ');
    }
    let pig_latin_words = pig_latin_words.trim_end();
    println!("Translated: \"{pig_latin_words}\"");
}
