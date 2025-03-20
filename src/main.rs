use std::{collections::HashMap, hash::Hash, time::Instant};
use regex::Regex;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use std::sync::mpsc::channel;

use combinations::Combinations;

const PUZZLE : &str = "t. .i.d t..rt. .o .o. t... na. ..ne y.. .w.lm. ..cy d.ne";

//fs.readFileSync('./node_modules/word-list/commons.txt', 'utf8').split('\n')
//.filter(word => word.length < 7);

struct Transform {
    //mappings: [u8; 10],
    left: Combinations<char>,
    right: Combinations<char>,
    current_l: Vec<char>,
    current_r: Vec<char>,
}

impl Transform {
    //tidronaeywlmc

    const L_LETTERS : [char; 13] = ['t','i','d','r','o','n','a','e','y','w','l','m','c'];
    const R_LETTERS : [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

    fn init() -> Transform {
        let mut l = Combinations::new(Self::L_LETTERS.into(), 5);
        let mut r = Combinations::new(Self::R_LETTERS.into(), 5);
        let curr_l = l.next().unwrap();
        let curr_r = r.next().unwrap();
        Transform {
            left: l,
            right: r,
            current_l: curr_l,
            current_r: curr_r,
        }
    }

    fn next(&mut self) -> bool {

        let done = !self.right.next_combination(&mut self.current_r);
        if done {
            self.right = Combinations::new(Self::R_LETTERS.into(), 5);
            return self.left.next_combination(&mut self.current_l);
        }
        return true;
    }

    fn apply(&self, mut str: String) -> String {
        let mut result = std::mem::take(&mut str).into_bytes();
        //println!("Applying transform: {}",self);
        for t in std::iter::zip(self.current_l.iter(),self.current_r.iter()) {
            //println!("Replacing {} with {}",*t.0,*t.1);
            replace(
                &mut result,
                *t.0,
                *t.1,
            )
        }
        String::from_utf8(result).unwrap()
    }
    
}

impl std::fmt::Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "| ")?;
        for i in 0..5 {
            write!(f, "{}➔ {} | ", self.current_l[i], self.current_r[i])?;
        }
        Ok(())
    }
}

fn replace(search: &mut[u8], find: char, replace: char) {

    for b in search {
        if *b == find as u8 {
            *b = replace as u8;
        }
    }
}

fn get_possibilities(t: &Transform, puzzle: &String, wordlist: &HashMap<usize,String>) -> Vec<Vec<String>> {
    puzzle.split(' ')
        .map(|word| {
            let regex = Regex::new(format!(r"(?m)^{}$", word).as_str()).unwrap();
            //println!("regex: {}",regex);
            let possibilities : Vec<String> = regex
                .find_iter(
                wordlist.get(&word.len()).unwrap()
                )
                .map(|x| String::from(x.as_str()))
                .collect();
            //println!("Using regex {}, possibilities are: {:?}", word, possibilities);
            possibilities
        })
        .collect()
}

fn main() {
    /*let test = Combinations::new([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30].to_vec(), 10);
        let r = test.into_iter().par_bridge().map(|item| {
            item.iter().sum::<i32>()
        }).sum::<i32>();

        println!("{}",r);
        return;*/
    let word_list = {
        use std::fs::read_to_string;
        use std::path::Path;
    
        let path = Path::new("./node_modules/word-list/commons.txt");
        let file_content: String = read_to_string(path).unwrap();
        let all_words: Vec<String> = file_content
            .lines()  // split the string into an iterator of string slices
            .map(String::from)
            .collect();
        let words: HashMap<usize,String> = vec![1,2,3,4,5,6]
            .into_iter()
            .map(|word_length| {
                (word_length, all_words.clone()
                                        .into_iter()
                                        .filter(|word| word.len() == word_length)
                                        .collect::<Vec<_>>()
                                        .join("\n")
                                    )
            })
            .collect();
        
        words
    };

    println!("{}",PUZZLE);
    println!("Loaded wordlist: {:?}",word_list.clone().into_iter().map(|(size,words)| (size,words.len())).collect::<HashMap<_,_>>());
    let mut str = String::from("abc");
    let mut x = std::mem::take(&mut str).into_bytes();
    replace(&mut x, 'a', 'i');
    println!("Replacing 'a' in abc with i: {} ",std::str::from_utf8(&x).unwrap());

    let mut result = String::from(PUZZLE);
    let mut transform = Transform::init();
    println!("Transform initialized: {}",transform);

    let mut now = std::time::Instant::now();
    let mut count: u64 = 0;
    println!("{}",result);
    result = transform.apply(result);
    println!("{}",result);

    let mut possibility_histogram: Histogram<usize> = Histogram::new();
    let mut limiting_words: Histogram<String> = Histogram::new();
    loop {
        count += 1;
        
        let done = !transform.next();
        if done {return;}
        result = transform.apply(PUZZLE.to_string());
        let mut possibilities = get_possibilities(&transform, &result,&word_list);

        possibilities.sort_by(|a,b| a.len().cmp(&b.len()));
        possibility_histogram.push(&possibilities[0].len());
        possibilities[0].iter()
            .for_each(|word| limiting_words.push(word));

        if now.elapsed().as_secs() > 10 {
            println!("Transform updated x{}: {}",count, transform);
            println!("updates/sec: {}", count/10);
            println!("Possibilities: {:?}",possibility_histogram.data);
            println!("Limiting words: {:?}",limiting_words.data);
            now = Instant::now();
        }
    }

}


struct Histogram<T> {
    data: HashMap<T,usize>
}

impl<T: Eq + Hash + Clone> Histogram<T> {
    fn new() -> Histogram<T> {
        Histogram {
            data: HashMap::new()
        }
    }

    fn push(&mut self, s: &T) {
        *self.data.entry(s.clone()).or_insert(0) += 1;
    }
}
