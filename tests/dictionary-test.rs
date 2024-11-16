#![cfg(test)]

use hardly_trie::trie::Trie;
use std::collections::HashMap;
use radix_trie::{Trie as RxTrie, TrieCommon};

const CONTENTS: &str = include_str!("./data/wordlist/wordlist-20210729.txt");

use std::time::Instant;

trait Collection {
    fn add(&mut self, key: &str) -> Option<String>;

    fn find(&self, key: &str) -> Option<&String>;
    fn size(&self) -> usize;

    fn remove(&mut self, key: &str) -> Option<String>;
}

impl Collection for Trie<str, String, 16> {
    fn add(&mut self, key: &str) -> Option<String> {
        self.insert(key, key.into())
    }

    fn find(&self, key: &str) -> Option<&String> {
        self.get(key)
    }

    fn size(&self) -> usize {
        self.len()
    }

    fn remove(&mut self, key: &str) -> Option<String> {
        self.delete(key)
    }
}

#[test]
fn insert_all_trie() {
    let mut trie = Trie::new();
    let now = Instant::now();
    insert_all(&mut trie);
    let elapsed_insert = now.elapsed();
    let now = Instant::now();
    find_all(&trie);
    let elapsed_find = now.elapsed();
    let now = Instant::now();
    remove_all(&mut trie);
    let elapsed_remove = now.elapsed();
    println!("hardly_trie::Trie insert: {:?} - size: {}", elapsed_insert, trie.len());
    println!("hardly_trie::Trie find: {elapsed_find:?}");
    println!("hardly_trie::Trie remove_all: {elapsed_remove:?}");
    println!("hardly_trie::Trie total: {:?}\n", elapsed_insert + elapsed_find + elapsed_remove);
}

impl Collection for HashMap<String, String> {
    fn add(&mut self, key: &str) -> Option<String> {
        self.insert(key.into(), key.into())
    }

    fn find(&self, key: &str) -> Option<&String> {
        self.get(key)
    }

    fn size(&self) -> usize {
        self.len()
    }

    fn remove(&mut self, key: &str) -> Option<String> {
        self.remove(key)
    }
}

#[test]
fn insert_all_hashmap() {
    let mut hashmap = HashMap::new();
    let now = Instant::now();
    insert_all(&mut hashmap);
    let elapsed_insert = now.elapsed();
    let now = Instant::now();
    find_all(&hashmap);
    let elapsed_find = now.elapsed();
    let now = Instant::now();
    remove_all(&mut hashmap);
    let elapsed_remove = now.elapsed();
    println!("std::HashMap insert: {:?} - size: {}", elapsed_insert, hashmap.len());
    println!("std::HashMap find: {elapsed_find:?}");
    println!("std::HashMap remove_all: {elapsed_remove:?}");
    println!("std::HashMap total: {:?}\n", elapsed_insert + elapsed_find + elapsed_remove);
}

impl Collection for RxTrie<String, String> {
    fn add(&mut self, key: &str) -> Option<String> {
        self.insert(key.into(), key.into())
    }

    fn find(&self, key: &str) -> Option<&String> {
        self.get(key)
    }

    fn size(&self) -> usize {
        self.len()
    }

    fn remove(&mut self, key: &str) -> Option<String> {
        self.remove(key)
    }
}

#[test]
fn insert_all_radixtrie() {
    let mut trie = RxTrie::new();
    let now = Instant::now();
    insert_all(&mut trie);
    let elapsed_insert = now.elapsed();
    let now = Instant::now();
    find_all(&trie);
    let elapsed_find = now.elapsed();
    let now = Instant::now();
    remove_all(&mut trie);
    let elapsed_remove = now.elapsed();
    println!("radix_trie::Trie insert: {:?} - size: {}", elapsed_remove, trie.len());
    println!("radix_trie::Trie find: {elapsed_remove:?}");
    println!("radix_trie::Trie remove_all: {elapsed_remove:?}");
    println!("radix_trie::Trie total: {:?}\n", elapsed_insert + elapsed_find + elapsed_remove);
}

fn insert_all<C: Collection>(c: &mut C) {
    for line in CONTENTS.lines() {
        let line = line.strip_prefix('"').unwrap_or(line);
        let line = line.strip_suffix('"').unwrap_or(line);
        if let Some(val) = c.add(line) {
            panic!("What? {line} <> {val}");
        }
    }
}

fn find_all<C: Collection>(c: &C) {
    let mut found: usize = 0;
    let mut not_found: usize = 0;
    for line in CONTENTS.lines() {
        let line = line.strip_prefix('"').unwrap_or(line);
        let line = line.strip_suffix('"').unwrap_or(line);
        if c.find(line).is_some() {
            found += 1;
        } else {
            not_found += 1;
        }
    }
    assert_eq!(not_found, 0);
    assert_eq!(found, c.size());
}

fn remove_all<C: Collection>(c: &mut C) {
    let orig_size = c.size();
    let mut found: usize = 0;
    let mut not_found: usize = 0;
    for line in CONTENTS.lines() {
        let line = line.strip_prefix('"').unwrap_or(line);
        let line = line.strip_suffix('"').unwrap_or(line);
        if c.remove(line).is_some() {
            found += 1;
        } else {
            not_found += 1;
        }
    }
    assert_eq!(not_found, 0);
    assert_eq!(found, orig_size);
}


