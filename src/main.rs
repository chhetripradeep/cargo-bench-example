#![feature(test)]

extern crate test;

fn concat_first(val: i64) -> String {
    let mut result = String::new();

    for i in 0..val {
        result.push_str(&i.to_string());
    }

    result
}

fn concat_second(val: i64) -> String {
    let mut result = String::new();
    let length: usize = val.to_string().len();
    let reservation: usize = (val-1) as usize * length;

    result.reserve(reservation);

    for i in 0..val {
        result.push_str(&i.to_string());
    }

    result
}

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
}