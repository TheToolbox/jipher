use combinations::Combinations;
use std::{collections::HashMap, time::Instant, time::Duration};
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use std::io::prelude::*;
use std::sync::atomic;
use regex::Regex;

pub type TransformHash = HashMap<char,char>;
pub type Possibilities = Vec<Vec<String>>;
pub type TransformAndPossibilities = (TransformHash,Possibilities);

pub type TransformAndPossibilitiesList =  Vec<TransformAndPossibilities>;

static COUNT : atomic::AtomicUsize = atomic::AtomicUsize::new(0);
const PUZZLE : &str = "t. .i.d t..rt. .o .o. t... na. ..ne y.. .w.lm. ..cy d.ne";

struct AllTransforms {}
impl AllTransforms {
    fn parallel_map(inner: impl Fn(&Transform) -> Option<TransformAndPossibilities> + Sync) -> TransformAndPossibilitiesList {
        //spawn a thread to output progress info
        rayon::spawn(|| {
            let mut count = 0;
            let now = Instant::now();
            println!("");
            while count < 1287 {
                count = COUNT.load(atomic::Ordering::Relaxed);
                print!("\r{}/1287 completed in {} seconds",
                    COUNT.load(atomic::Ordering::Relaxed),
                    now.elapsed().as_secs()
                );
                std::io::stdout().flush().unwrap();
                std::thread::sleep(Duration::from_secs(1));
            }
        });
        
        //iterate over all possible L sides of the transform in parallel
        let left_combinator = Combinations::new(Transform::L_LETTERS.into(),5);
        left_combinator.par_bridge()
            .map(|left| {
                // Given our L side of the transform, try all possible R-side combinations
                let right_combinator = Combinations::new(Transform::R_LETTERS.into(), 5);
                let result = right_combinator.filter_map(|right| {
                    let transform = Transform::new(&left,&right);
                    //apply our money function to this transform and include only if it's Some(value)
                    inner(&transform)
                })
                .collect::<TransformAndPossibilitiesList>();
                COUNT.fetch_add(1, atomic::Ordering::Relaxed);
                result
            })
            .flatten()
            .collect()
    }
}

pub fn get_all_solutions() -> TransformAndPossibilitiesList {
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

    
    let result = AllTransforms::parallel_map(|transform| {
        
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

    result
}

fn get_possibilities(puzzle: &String, wordlist: &HashMap<usize,String>) -> Vec<Vec<String>> {
    puzzle.split(' ')
        .map(|word| {
            let regex = Regex::new(format!(r"(?m)^{}$", word).as_str()).unwrap();
            let possibilities : Vec<String> = regex
                .find_iter(
                wordlist.get(&word.len()).unwrap()
                )
                .map(|x| String::from(x.as_str()))
                .collect();
            possibilities
        })
        .collect()
}

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

    fn apply(&self, mut str: String) -> String {
        let mut result = std::mem::take(&mut str).into_bytes();
        //println!("Applying transform: {}",self);
        for t in std::iter::zip(self.left.iter(),self.right.iter()) {
            for b in &mut result {
                if *b == *t.0 as u8 {
                    *b = *t.1 as u8
                }
            }
            //println!("Replacing {} with {}",*t.0,*t.1);
        }
        String::from_utf8(result).unwrap()
    }

    fn into_hashmap(&self) -> HashMap<char,char>{
        std::iter::zip(
            self.left.iter(),
            self.right.iter())
            .map(|(a,b)| {(*a,*b)})
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

