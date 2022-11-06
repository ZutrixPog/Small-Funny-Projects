use autocomplete::Autocomplete::Autocompleter;

fn main() {
    let mut auto = Autocompleter::new_with_trie();

    let words = vec!["run".to_string(), "runner".to_string(), "runners".to_string(), "running".to_string(), "ran".to_string()];
    auto.load_words(words);
    auto.set_limit(2);

    println!("{:?}", auto.auto_complete("ru".to_string()));
}
