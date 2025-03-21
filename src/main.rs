use core::time;
use std::{collections::HashMap, hash::Hash, time::Instant};
use regex::Regex;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use serde_json;
use combinations::Combinations;
use std::io::prelude::*;

const PUZZLE : &str = "t. .i.d t..rt. .o .o. t... na. ..ne y.. .w.lm. ..cy d.ne";

//fs.readFileSync('./node_modules/word-list/commons.txt', 'utf8').split('\n')
//.filter(word => word.length < 7);

#[derive(Clone)]
struct Transform<'a> {
    //mappings: [u8; 10],
    left: &'a Vec<char>,
    right: &'a Vec<char>,
}

impl<'a> Transform<'a> {
    //tidronaeywlmc

    const L_LETTERS : [char; 13] = ['t','i','d','r','o','n','a','e','y','w','l','m','c'];
    const R_LETTERS : [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

    fn new(l: &'a Vec<char>, r: &'a Vec<char>) -> Transform<'a> {
        Transform {
            left: l,
            right: r,
        }
    }

    /*fn next(&mut self) -> bool {

        let done = !self.right.next_combination(&mut self.current_r);
        if done {
            self.right = Combinations::new(Self::R_LETTERS.into(), 5);
            return self.left.next_combination(&mut self.current_l);
        }
        return true;
    }*/

    fn apply(&self, mut str: String) -> String {
        let mut result = std::mem::take(&mut str).into_bytes();
        //println!("Applying transform: {}",self);
        for t in std::iter::zip(self.left.iter(),self.right.iter()) {
            //println!("Replacing {} with {}",*t.0,*t.1);
            replace(
                &mut result,
                *t.0,
                *t.1,
            )
        }
        String::from_utf8(result).unwrap()
    }

    fn into_hashmap(&self) -> HashMap<char,char>{
        std::iter::zip(
            self.right.iter(),
            self.right.iter())
            .map(|(a,b)| {(*a,*b)})
            .collect()
    }


    
}

type TransformHash = HashMap<char,char>;
type Possibilities = Vec<Vec<String>>;
type TransformAndPossibilities = (TransformHash,Possibilities);
type TransformAndPossibilitiesList =  Vec<TransformAndPossibilities>;
struct Transforms {}

//use std::sync::atomic::{AtomicUsize, Ordering};
static COUNT : std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
impl Transforms {
    fn parallel_map(inner: impl Fn(&Transform) -> Option<TransformAndPossibilities> + Sync) -> TransformAndPossibilitiesList {
        

        rayon::spawn(|| {
            let now = Instant::now();
            println!("");
            loop {
                std::thread::sleep(
                    time::Duration::from_secs(1)
                );
                print!("\r{}/1287 completed in {} seconds",
                    COUNT.load(std::sync::atomic::Ordering::Relaxed),
                    now.elapsed().as_secs()
                );
                std::io::stdout().flush().unwrap();
                //println!("\r{} seconds elapsed",);

            }
            /*if now.elapsed().as_secs() > 10 {
                //println!("Transform updated x{}: {}",count);
                //println!("updates/sec: {}", count/10);
                println!("Possibilities: {:?}",possibility_histogram.data);
                println!("All words: {:?}",limiting_words.data);
                now = Instant::now();
            }*/
        });


        
        let left_combinator = Combinations::new(Transform::L_LETTERS.into(),5);
        left_combinator.par_bridge()
            .map(|left| {

                let right_combinator = Combinations::new(Transform::R_LETTERS.into(), 5);
                let result = right_combinator.filter_map(|right| {
                    let transform = Transform::new(&left,&right);
                    inner(&transform)
                })
                .collect::<TransformAndPossibilitiesList>();
                COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                result
            })
            .flatten()
            .collect()
    }
}

impl std::fmt::Display for Transform<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "| ")?;
        for i in 0..5 {
            write!(f, "{}➔ {} | ", self.left[i], self.right[i])?;
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

fn get_possibilities(puzzle: &String, wordlist: &HashMap<usize,String>) -> Vec<Vec<String>> {
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

    println!("Puzzle: {}",PUZZLE);
    println!("Loaded wordlist: {:?}",word_list.clone().into_iter().map(|(size,words)| (size,words.len())).collect::<HashMap<_,_>>());

    /*let mut result = String::from(PUZZLE);

    let mut now = std::time::Instant::now();
    let mut count: u64 = 0;

    let mut possibility_histogram: Histogram<usize> = Histogram::new();
    let mut limiting_words: Histogram<String> = Histogram::new();*/
    
    let result = Transforms::parallel_map(|transform| {
        
        let result = transform.apply(PUZZLE.to_string());
        let possibilities = get_possibilities(&result,&word_list);

        if possibilities.iter().map(|x| x.len()).min().unwrap() > 0 {
            Some((transform.into_hashmap(),possibilities))
        } else {
            None
        }
        //possibilities.sort_by(|a,b| a.len().cmp(&b.len()));
        //possibility_histogram.push(&possibilities[0].len());
        //possibilities[0].iter()
        //    .for_each(|word| limiting_words.push(word));

    });



    let mut file = std::fs::File::create("output.json").unwrap();
            
    file.write_all(
        serde_json::to_string(&result)
            .unwrap()
            .as_bytes()
        )
        .unwrap();
}

#[allow(dead_code)]
struct Histogram<T> {
    data: HashMap<T,usize>
}

#[allow(dead_code)]
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
