use std::collections::HashMap;
use std::hash::Hash;

pub struct Histogram<T> {
    data: HashMap<T,u64>
}

impl<T: Eq + Hash + Clone> Histogram<T> {
    pub fn new() -> Histogram<T> {
        Histogram {
            data: HashMap::new()
        }
    }

    pub fn push(&mut self, s: &T) {
        *self.data.entry(s.clone()).or_insert(0) += 1;
    }
}

impl<T: Eq + std::hash::Hash + Clone> Histogram<T> {

    pub fn push_multiple(&mut self, s: &T, increment: u64) {
        *self.data.entry(s.clone()).or_insert(0) += increment;
    }

    pub fn most_popular(&self) -> Vec<(T, u64)> {
        let mut result = self.data.iter().map(|(s,n)| (s.clone(),n.clone())).collect::<Vec<_>>();
        result.sort_by_key(|(_,count)| u64::MAX-(*count));
        result
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.data.clear();
    }
}
