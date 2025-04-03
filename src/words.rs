use std::{collections::HashSet, time::Duration, u64};
use crate::histogram::Histogram;
use crate::solutions::TransformAndPossibilitiesList;
/*
type TransformHash = HashMap<char,char>;
type Possibilities = Vec<Vec<String>>;
type TransformAndPossibilities = (TransformHash,Possibilities);
type TransformAndPossibilitiesList =  Vec<TransformAndPossibilities>; */

#[allow(dead_code)]
enum UpdatableValue<T> {
    Value(T),
    Updating,
    Invalid,
}
pub struct Words {
    state: TransformAndPossibilitiesList,
    word_hist: Histogram<String>,//UpdatableValue<Histogram<String>>
    total_combinations: u64,
    critical_words: Histogram<String>,
    positional_word_hists: Vec<Histogram<String>>,
    history: Vec<String>,
}

impl Words {
    pub fn new(data: TransformAndPossibilitiesList) -> Words {

        let mut me = Words {
            state: data,
            word_hist: Histogram::new(),//UpdatableValue::Invalid,
            total_combinations: 0,
            critical_words: Histogram::new(),
            positional_word_hists: vec![],
            history: vec![],
        };

        println!("Calculating stats on possiblities...");
        me.update_all();
        me
    }

    pub fn word_hist(&self) -> &Histogram<String> {
        &self.word_hist
    }

    fn update_all(&mut self) {
        //second pass to remove any transforms that have no possible complete sentences
        self.state
        .retain(|(_transform, words)| {
            words.iter().map(|word_options| word_options.len() as u64).product::<u64>() != 0
        });

        //probably could be much-optimized but hasn't been necessary so far, so a brute-force update is what you get
        self.update_word_hist();
        self.update_total_combinations();
        self.update_critical_words();
        self.update_positional_word_hist();
    }

    fn update_word_hist(&mut self) {
        let mut word_hist = Histogram::new();
        self.state.iter()
        .for_each(|(_,possibilities)| {
            let number_of_possibilities : usize = possibilities.iter().map(|v| v.len()).product();
            for i in 0..possibilities.len() {
                for j in 0..possibilities[i].len() {
                    let word = &possibilities[i][j];
                    let increment = (number_of_possibilities/possibilities[i].len()) as u64;
                    word_hist.push_multiple(word, increment);
                }
            }
        });
        self.word_hist = word_hist;//UpdatableValue::Value(word_hist);
    }

    pub fn positional_histograms(&self) -> &Vec<Histogram<std::string::String>> {
        &self.positional_word_hists
    }

    fn update_positional_word_hist(&mut self) {
        let mut word_hists : Vec<Histogram<String>> = (0..self.state[0].1.len()).map(|_| Histogram::new()).collect();
        self.state.iter()
            .for_each(|(_,sentence)| {
                for (i,wordlist) in sentence.iter().enumerate() {
                    wordlist.iter().for_each(|word| word_hists[i].push(word));
                }
            });
        self.positional_word_hists = word_hists;
    }

    pub fn get_top(&self, range: std::ops::Range<usize>, filter: impl FnMut(&(String,u64)) -> bool) -> Vec<(String,u64)> {
        self.word_hist()
            .most_popular()
            .into_iter().filter(filter).skip(range.start).take(range.end - range.start).collect()
    }       

    pub fn total_combinations(&self) -> u64 {
        self.total_combinations
    }

    fn update_total_combinations(&mut self) {
        self.total_combinations = self.state.iter()
            .map(|(_transform, words)| {
                words.iter()
                    .map(|word_possibilities| word_possibilities.len() as u64)
                    .product::<u64>()
            })
            .sum::<u64>()
    }

    pub fn total_transforms(&self) -> usize {
        return self.state.len();
    }

    pub fn remove_words(&mut self, words: Vec<String>) {
        let word_list = words.join(",");
        for word in words {
            for (_transform, words) in &mut self.state {
                for word_list in words {
                    if let Some(pos) = word_list.iter().position(|w| *w == *word) {
                        word_list.swap_remove(pos);
                    }
                }
            }
        }
        
        self.update_all();

        self.history.push("X Words: ".to_string() + &word_list);  
        self.history.rotate_right(1);
    }

    pub fn remove_words_positional(&mut self, words: Vec<String>, position: usize) {
        let word_list = words.join(",");

        if position > self.state[0].1.len() { panic!(); }

        for target_word in words {
            for (_transform, sentences) in &mut self.state {
                if let Some(pos) = sentences[position].iter().position(|w| *w == *target_word) {
                    sentences[position].swap_remove(pos);
                }
            }
        }

        self.update_all();

        self.history.push(format!("X Word {}: {}", position, word_list));
        self.history.rotate_right(1);
    }

    pub fn require_word(&mut self, word: String) {
        self.state.retain(|(_, sentence)| {
            sentence.iter()
                .any(|wordlist| wordlist.contains(&word))
        });

        self.update_all();

        self.history.push(format!("+ Words: {}", word));
        self.history.rotate_right(1);
    }

    pub fn require_word_positional(&mut self, target_word: String, position: usize) {
        self.state.iter_mut().for_each(|(_,sentence)| {
            sentence[position].retain(|word| word == &target_word)
        });

        self.update_all();
        self.history.push(format!("+ Word {}: {}", position, target_word));
        self.history.rotate_right(1);
    }

    pub fn critical_words(&self) -> Vec<(String,u64)> {
        self.critical_words.most_popular()
    }

    fn update_critical_words(&mut self) {
        self.critical_words = Histogram::new();
        self.state.iter()
            .for_each(|(_,sentences)| {
                sentences.iter().for_each(|words| {
                    if words.len() == 1 {
                        self.critical_words.push(&words[0]);
                    }
                    if words.len() == 0 {
                        panic!("empty wordlist found!");
                    }
                });
            });
    }

    pub fn total_words(&self) -> usize {        
        return self.state.iter()
            .flat_map(|(_transform,sentences)| {
                sentences.iter().flatten()
            })
            .collect::<HashSet<&String>>()
            .len()
    }

    //spits out a random transform and solution from that transform
    pub fn random_solution(&self) -> (String,String) {
        fn bad_rand(from: usize, to: usize) -> usize {
            let now = std::time::SystemTime::now();
            let rand = now.duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap_or(Duration::new(1,0))
                .as_nanos();
            let range = to - from;
            ((rand % range as u128) + from as u128).try_into().unwrap_or(0)
        }
        let i = bad_rand(0, self.state.len());
        let (transform,sentence) = self.state[i].clone();
        (
            format!("{:?}",transform)
            ,sentence.iter()
            .map(|wordlist| {
                let j = bad_rand(0, wordlist.len());
                wordlist[j].clone() + " "
            })
            .collect::<String>()
        )
    }

    pub fn sentence_length(&self) -> usize {
        if let Some((_,sentences)) = self.state.get(0) {
            sentences.len()
        } else {
            0
        }
    }

    pub fn history(&self) -> String {
        self.history.clone().join("\n")
    }
}














