use std::collections::HashMap;

pub mod client; 
pub mod network;

pub fn mean(v: &Vec<u64>) -> f64 {
    let mut total = 0;
    for i in v.iter() {
        total += *i;
    }
    (total as f64)/(v.len() as f64)
}

pub fn mode(v: &Vec<u64>) -> u64 {
    let mut m = HashMap::new();
    for i in v.iter() {
        let count = m.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    let mut last = 0;
    for (k,v) in m {
        if v>max {
            last = k;
            max = v;
        }
    }
    last
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
