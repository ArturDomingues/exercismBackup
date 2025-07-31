use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    let sort_chars=|s:&str|{
        let mut chars: Vec<char>=s.to_lowercase().chars().collect();
        chars.sort_unstable();
        chars
    };
    let sorted_word=sort_chars(word);
    possible_anagrams
        .iter()
        .copied()
        .filter(|candidate|{
            if candidate.to_lowercase()==word.to_lowercase(){
                return false;
            }
            sort_chars(candidate)==sorted_word
        })
        .collect()    
}
