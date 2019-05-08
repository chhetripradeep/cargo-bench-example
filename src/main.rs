#![feature(test)]

extern crate test;

// most trivial: iterate over nums,
// convert nums to string and push 
// them to final result string.
fn concat_first(val: i64) -> String {
    let mut result = String::new();

    for i in 0..val {
        result.push_str(&i.to_string());
    }

    result
}

// pre-decide the size of final result string.
fn concat_second(val: i64) -> String {
    let length: usize = val.to_string().len();
    let capacity: usize = (val-1) as usize * length;

    let mut result = String::with_capacity(capacity);

    for i in 0..val {
        result.push_str(&i.to_string());
    }

    result
}

// create a vector of numbers and using fold
// to generate the resultant string.
fn concat_third(val: i64) -> String {
    let mut vec: Vec<i64> = Vec::new();

    for i in 0..val {
        vec.push(i)
    }

    vec.iter().fold(
        String::new(),
        |accumulator, s| format!("{}{}", accumulator, s)
    )
}

// heaplessly converting numbers into their string
fn concat_fourth(val: i64) -> String {
    use numtoa::NumToA;

    let mut buffer = [0u8; 20];
    let length: usize = val.to_string().len();
    let capacity: usize = (val-1) as usize * length;
    let mut result = String::with_capacity(capacity);

    for i in 0..val {
        result.push_str(i.numtoa_str(10, &mut buffer));
    }

    result
}

// discussion: https://www.reddit.com/r/learnrust/comments/bjyrgf/feedback_on_blogpost/
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn benchmark_first(b: &mut Bencher) {
        b.iter(|| concat_first(1000));
    }

    #[bench]
    fn benchmark_second(b: &mut Bencher) {
        b.iter(|| concat_second(1000));
    }

    #[bench]
    fn benchmark_third(b: &mut Bencher) {
        b.iter(|| concat_third(1000));
    }

    #[bench]
    fn benchmark_fourth(b: &mut Bencher) {
        b.iter(|| concat_fourth(1000));
    }
}