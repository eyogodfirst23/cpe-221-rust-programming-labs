use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        let clean: String = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_lowercase().next().unwrap())
            .collect();
        if !clean.is_empty() {
            *freq.entry(clean).or_insert(0) += 1;
        }
    }
    freq
}

fn top_n(freq: &HashMap<String, usize>, n: usize) -> Vec<(&String, &usize)> {
    let mut entries: Vec<(&String, &usize)> = freq.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));
    entries.into_iter().take(n).collect()
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the fox was very quick";
    let freq = word_frequency(text);

    for (word, count) in top_n(&freq, 5) {
        println!("{:?}: {}", word, count);
    }
}
