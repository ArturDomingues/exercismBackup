use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    let word_lc = word.to_lowercase();
    let sort_chars = |s: &str| {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        chars
    };
    let sorted_word = sort_chars(&word_lc);

    let mut seen = HashSet::<&str>::new();
    possible_anagrams
        .iter()
        .copied()
        .filter(|s| seen.insert(s)) 
        .filter(|candidate| {
            let candidate_lc = candidate.to_lowercase();
            if candidate_lc == word_lc {
                return false;
            }
            sort_chars(&candidate_lc) == sorted_word
        })
        .collect()
}
