#![allow(dead_code)]
use std::collections::{BTreeMap, HashMap};

pub struct BigramModel {
    word_to_id: HashMap<String, usize>,
    id_to_word: HashMap<usize, String>,
    counts: BTreeMap<(usize, usize), u32>,
    total: u32,
    next_id: usize,
}

impl BigramModel {
    pub fn new() -> BigramModel {
        BigramModel {
            word_to_id: HashMap::new(),
            id_to_word: HashMap::new(),
            counts: BTreeMap::new(),
            total: 0,
            next_id: 1,
        }
    }

    pub fn update(&mut self, word1: &str, word2: &str) {
        let id1 = self.get_word_id(word1);
        let id2 = self.get_word_id(word2);
        let count = self.counts.entry((id1, id2)).or_insert(0);
        *count += 1;
        self.total += 1;
    }

    pub fn two_word_logprob(&self, word1: &str, word2: &str) -> f64 {
        let id1 = *self.word_to_id.get(word1).unwrap_or(&0);
        let id2 = *self.word_to_id.get(word2).unwrap_or(&0);
        let key = (id1, id2);
        let count = match self.counts.get(&key) {
            Some(count) => *count,
            None => 1,
        };
        let prob = (count as f64 + 1.0) / (self.total as f64 + self.counts.len() as f64);
        prob.log(std::f64::consts::E)
    }

    pub fn logprob(&self, words: &[&str]) -> f64 {
        if words.len() < 1 {
            return 0.0;
        }
        if words.len() == 1 {
            return self.one_word_logprob(words[0]);
        }

        let mut logprob = 0.0;
        for i in 1..words.len() {
            logprob += self.two_word_logprob(words[i - 1], words[i]);
        }
        logprob
    }

    pub fn two_word_conditional_logprob(&self, word1: &str, word2: &str) -> f64 {
        let id1 = *self.word_to_id.get(word1).unwrap_or(&0);
        let id2 = *self.word_to_id.get(word2).unwrap_or(&0);

        let range = self.counts.range((id1, 0)..=(id1, usize::MAX));
        let count_word1_bigrams: u32 = range.map(|(_, &count)| count).sum();
        if count_word1_bigrams == 0 {
            return 0.0; // TODO is this good?
        }

        let key = (id1, id2);
        let count_bigram = match self.counts.get(&key) {
            Some(count) => *count,
            None => 0,
        } as f64;

        let prob = count_bigram / (count_word1_bigrams as f64);
        prob.log(std::f64::consts::E)
    }

    pub fn one_word_logprob(&self, word: &str) -> f64 {
        let id = *self.word_to_id.get(word).unwrap_or(&0);
        let range = self.counts.range((id, 0)..=(id, usize::MAX));
        let count: u32 = range.map(|(_, &count)| count).sum();
        let prob = (count as f64 + 1.0) / (self.total as f64 + self.counts.len() as f64);
        prob.log(std::f64::consts::E)
    }

    pub fn conditional_logprob(&self, words: &[&str]) -> f64 {
        if words.len() < 1 {
            return 0.0;
        }
        if words.len() == 1 {
            return self.one_word_logprob(words[0]);
        }

        let mut logprob = self.one_word_logprob(words[0]);
        for i in 1..words.len() {
            logprob += self.two_word_conditional_logprob(words[i - 1], words[i]);
        }

        logprob
    }

    fn get_word_id(&mut self, word: &str) -> usize {
        match self.word_to_id.get(word) {
            Some(&id) => id,
            None => {
                let id = self.next_id;
                self.word_to_id.insert(word.to_string(), id);
                self.id_to_word.insert(id, word.to_string());
                self.next_id += 1;
                id
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update() {
        let mut bigram = BigramModel::new();
        bigram.update("foo", "bar");
        assert_eq!(bigram.counts.get(&(1, 2)), Some(&1));
    }
}
