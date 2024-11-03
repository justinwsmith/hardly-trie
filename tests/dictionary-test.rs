#![cfg(test)]

use hardly_trie::Trie;
use std::collections::HashMap;
use radix_trie::{Trie as RxTrie, TrieCommon};


use std::time::Instant;

trait Collection {
    fn add(&mut self, key: &str) -> Option<String>;
    fn find(&self, key: &str) -> Option<&String>;
}

impl Collection for Trie<String> {
    fn add(&mut self, key: &str) -> Option<String> {
        self.insert(key.as_bytes(), key.into())
    }

    fn find(&self, key: &str) -> Option<&String> {
        self.get(key.as_bytes())
    }
}

#[test]
fn insert_all_trie() {
    let mut trie = Trie::new();
    let now = Instant::now();
    insert_all(&mut trie);
    let elapsed = now.elapsed();
    println!("hardly_trie::Trie insert: {:?} - size: {}", elapsed, trie.len());
    let now = Instant::now();
    find_all(&trie);
    let elapsed = now.elapsed();
    println!("hardly_trie::Trie find: {:?}", elapsed);
}

impl Collection for HashMap<String, String> {
    fn add(&mut self, key: &str) -> Option<String> {
        self.insert(key.into(), key.into())
    }

    fn find(&self, key: &str) -> Option<&String> {
        self.get(key.into())
    }
}

#[test]
fn insert_all_hashmap() {
    let mut hashmap = HashMap::new();
    let now = Instant::now();
    insert_all(&mut hashmap);
    let elapsed = now.elapsed();
    println!("std::HashMap insert: {:?} - size: {}", elapsed, hashmap.len());
    let now = Instant::now();
    find_all(&hashmap);
    let elapsed = now.elapsed();
    println!("std::HashMap find: {:?}", elapsed);
}

impl Collection for RxTrie<String, String> {
    fn add(&mut self, key: &str) -> Option<String> {
        self.insert(key.into(), key.into())
    }

    fn find(&self, key: &str) -> Option<&String> {
        self.get(key.into())
    }
}

#[test]
fn insert_all_radixtrie() {
    let mut trie = RxTrie::new();
    let now = Instant::now();
    insert_all(&mut trie);
    let elapsed = now.elapsed();
    println!("radix_trie::Trie insert: {:?} - size: {}", elapsed, trie.len());
    let now = Instant::now();
    find_all(&trie);
    let elapsed = now.elapsed();
    println!("radix_trie::Trie find: {:?}", elapsed);
}

fn insert_all<C: Collection>(c: &mut C) {
    let contents = include_str!("./data/wordlist/wordlist-20210729.txt");
    for line in contents.lines() {
        let line = line.strip_prefix('"').unwrap_or(line);
        let line = line.strip_suffix('"').unwrap_or(line);
        if let Some(val) = c.add(line) {
            assert!(false, "What? {line} <> {val}");
        }
    }
}

fn find_all<C: Collection>(c: &C) {
    let contents = include_str!("./data/wordlist/wordlist-20210729.txt");
    for line in contents.lines() {
        let line = line.strip_prefix('"').unwrap_or(line);
        let line = line.strip_suffix('"').unwrap_or(line);
        assert!(c.find(line).is_some(), "What? {line}");
    }
}