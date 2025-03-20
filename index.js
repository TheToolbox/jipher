fs = require('node:fs');
const puzzle = "t_ _i_d t__rt_ _o _o_ t___ na_ __ne y__ _w_lm_ __cy d_ne";
const letters = "abcdefghijklmnopqrstuvwxyz".split("");

// load english word list
const wordArray = fs.readFileSync('./node_modules/word-list/commons.txt', 'utf8').split('\n')
    .filter(word => word.length < 7);

console.log(wordArray.length + " words loaded");

const x = list_words_that_fit_pattern("_w_lm_");

const transformations = transformation_generator();

let limiters = new Set();


let transform;

iterate();
function iterate() {
    let result = apply_transform(puzzle,transformations.next().value);
    //console.log(result);
    let solutions = count_possible_solutions(result);
    if (solutions > 0) {
        console.log(result);
        console.log(`Possible solutions: ${solutions}`);
    }
    setTimeout(iterate,solutions > 0 ? 10 : 0);    
}

function* transformation_generator() {

    const used_letters = "tidronaeywlmc".split("");

    //this constructs a full list that starts with the puzzle-used letters so that
    //L-side of transform can reference into this list and count only up to the 
    //13 letters that were used, but they still reference the same letters when 
    //checking if there are duplicates
    let transform_characters = used_letters.slice();//the .slice() clones the array
    for (let i = 0; i < letters.length; i++) {
        if (transform_characters.includes(letters[i])) { continue; }
        transform_characters.push(letters[i]);
    }

    const nam_tranformations = {
        "a": "y",
        "j": "t",
        "h": "e",
        "p": "s",
        "r": "u",
    };

    const left_transform_characters  = used_letters;
    const right_transform_characters = letters;

    let left_indices  = [0,0,0,0];
    let right_indices = [9,8,7,6,5];

    setInterval(() => {
        console.log('==================');
        console.log(`[${left_indices}] [${right_indices}]`);
        console.log(`limiters: ${Array.from(limiters)}`);
        console.log('==================');
    }, 20000);
    
    
    let iter = 0;
    while (true) {
      //console.log(`iteration #${iter++}`);
      //console.log(`[${left_indices}] [${right_indices}]`);
      const overflow = increment_array(left_indices,used_letters.length);
      //console.log(`incremented L indices to ${left_indices}`);
      if (overflow) {
        left_indices = [0,0,0,0,0];
        increment_array(right_indices,transform_characters.length);
      }

      if (
        //repeats of L or R indices
        duplicates(left_indices,right_indices)
        //other things that should disqualify a transform?
        || left_indices.includes('t')
        || left_indices.includes('r')
      ) {
        //skip
        //console.log('dup detected');
        continue;
      }


      yield {
        [transform_characters[left_indices[0]]]: transform_characters[right_indices[0]],
        [transform_characters[left_indices[1]]]: transform_characters[right_indices[1]],
        [transform_characters[left_indices[2]]]: transform_characters[right_indices[2]],
        [transform_characters[left_indices[3]]]: transform_characters[right_indices[3]],
        t:"g",

        //[transform_characters[left_indices[4]]]: transform_characters[right_indices[4]],
      }

    }
    return iterationCount;
}

function duplicates(arr, arr2) {
    return (new Set([...arr, ...arr2])).size !== (arr.length + arr2.length);
}

function increment_array(arr,limit) {
    let i = 0;
    while (i < arr.length) {
        arr[i]++;
        if (arr[i] < limit) {
            return false;
        }
        arr[i] = 0;
        i++;
    }
    return true;
}

function count_possible_solutions(puzzle) {
    let x = puzzle.split(' ')
        .map(word => [word, list_words_that_fit_pattern(word).length])
        .sort(([word1,count1],[word2,count2]) => count1-count2 )
    if (x[0][1] > 0 ) {
        const new_limiters = list_words_that_fit_pattern(x[0][0]);
        console.log(`limiting pattern is ${x[0][0]}: ${new_limiters}`);
        limiters.add(...new_limiters);
    }
    return x[0][1];
}

function list_words_that_fit_pattern(pattern) {    
    const regex = new RegExp("^" + pattern.replaceAll('_','.') + "$",'g');

    return wordArray.filter(x => regex.test(x));
}

function apply_transform(str, transform) {
    //this is slightly awkward to prevent double-applying transformations in case there are rules like e.g. e=>n and n=>p
    let transformed = [];
    //console.log(transform);
    for (let i = 0; i < str.length; i++) {
        transformed.push(transform[str[i]] || str[i]);
    }
    let result = transformed.join("");
    //console.log(result);
    return result;
}

function cull_candidates() {}
    

    //remove candidates that are missing nam's right side letters

    //nam's right side letters must exist within the blanks

    //likely the e's and y in the existing puzzle are fake since nam's R side contains it
