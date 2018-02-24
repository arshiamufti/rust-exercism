use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut r = HashSet::new();

    for i in 1..limit {
        for &f in factors {
            if (i >= f) && (i % f == 0) {
                println!("{}", i);
                r.insert(i);
            }
        }
    }

    r.iter().sum()
}
