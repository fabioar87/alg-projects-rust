// ************************
// Milestone 2 - quicksort
// Implementation

use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

// get_i32
//      function to parse an i32 argument entered by stdin.
fn get_i32(prompt: &str) -> i32 {

    println!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");    

    let trimmed = str_value.trim();
    return trimmed.parse::<i32>()
        .expect("Error parsing integer");
}

// PRNG:
//      function to generate pseudo-random numbers.
struct Prng {
    seed: u32,
}

impl Prng {
    fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        return prng;
    }

    fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    // Return a pseudorandom value in the range [0, 2147483647].
    fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        return self.seed;
    }

    // Return a pseudorandom value in the range [0.0, 1.0).
    fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        return f / (2147483647.0 + 1.0);
    }

    // Return a pseudorandom value in the range [min, max).
    fn next_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        return result as i32;
    }
}

// make_rand_vec: 
//      function to return a random vector of i32 elements.
fn make_rand_vec(num_items: i32, max: i32) -> Vec<i32> {
    
    let mut prng = Prng::new();
    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
    
    for _ in 0..num_items {
        vec.push(prng.next_i32(0, max));
    }

    return vec;
}

// print_vec function: 
//      printing all the entries of a vector of i32.
fn print_vec(vec: &Vec<i32>, num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }
    string.push_str("]");
    println!("{string}");
}

// quick_sort:
//      function implementing the basic version of quick sort. 
//      The following implements the lomuto partioning strategy

fn quick_sort(vec: &mut Vec<i32>, lo: i32, hi: i32) {
    
    if lo >= hi || lo < 0 {
        return
    } 

    let p = lomuto(vec, lo, hi);
    quick_sort(vec, lo, p - 1);
    quick_sort(vec, p + 1, hi);
}

fn lomuto(vec: &mut Vec<i32>, lo: i32, hi: i32) -> i32 {
    
    let pivot = vec[hi as usize];
    let mut flag = lo - 1;
    
    for j in lo..hi {
        if vec[j as usize] <= pivot {
            flag += 1;
            (vec[flag as usize], vec[j as usize]) = (vec[j as usize], vec[flag as usize]);
        }
    }

    flag += 1;
    (vec[hi as usize], vec[flag as usize]) = (vec[flag as usize], pivot);
    flag
}

// check_sorted:
//          function to inspect if a given vector is sorted.
fn check_sorted(vec: &Vec<i32>) {
    
    let n = vec.len();
    let mut sorted: bool = true;
    for i in 1..n {
        if vec[i-1] > vec[i]{
            println!("The vector is NOT sorted!");
            sorted = false;
            break;
        }
    }

    if sorted {
         println!("The vector is sorted!");
    }
}

fn main() {
    
    let num_items = get_i32("Enter the number of items: ");
    let max = get_i32("Enter the maximum item value: ");
    
    let mut v = make_rand_vec(num_items, max);
    print_vec(&v, num_items);
    quick_sort(&mut v, 0, num_items - 1);
    
    print_vec(&v, num_items);
    check_sorted(&v);
}