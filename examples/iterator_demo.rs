use hardly_trie::trie::Trie;

fn main() {
    let mut trie: Trie<str, String, 16> = Trie::new();

    // Insert some words
    trie.insert("apple", "apple".to_string());
    trie.insert("banana", "banana".to_string());
    trie.insert("cherry", "cherry".to_string());
    trie.insert("date", "date".to_string());

    println!("Trie contains {} items", trie.len());

    // Forward iteration
    println!("\nForward iteration:");
    for (path, value) in trie.iter() {
        println!("Path: {:?}, Value: {}", path, value);
    }

    // Backward iteration
    println!("\nBackward iteration:");
    for (path, value) in trie.iter().rev() {
        println!("Path: {:?}, Value: {}", path, value);
    }

    // Bidirectional iteration
    println!("\nBidirectional iteration (alternating front/back):");
    let mut iter = trie.iter();
    let mut count = 0;
    while let Some((path, value)) = if count % 2 == 0 {
        iter.next()
    } else {
        iter.next_back()
    } {
        println!("Path: {:?}, Value: {}", path, value);
        count += 1;
    }

    // Size hint demonstration
    let iter = trie.iter();
    let (lower, upper) = iter.size_hint();
    println!("\nSize hint: lower={}, upper={:?}", lower, upper);
}
