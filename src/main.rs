use std::collections::HashMap;

fn collatz_steps(mut c: u128, precalc_step_cnt: & mut HashMap<u128, u128>) -> u128 {
    let initial_c = c;
    let mut step_cnt = 1;
    while c > 1 {
        match precalc_step_cnt.get(&c) {
            Some(num) => { step_cnt += *num; break;},
            None => {}
        };
    
        if c % 2 == 0 {
            c = c / 2;
        } else {
            c = (3 * c) + 1;
        }

        step_cnt += 1;
    }
    precalc_step_cnt.insert(initial_c, step_cnt);
    
    step_cnt
}

fn main() {
    let mut precalculated_step: HashMap<u128, u128> = HashMap::new();

    let most_steps = (2..=50_000_000).map(|n| (n, collatz_steps(n, & mut precalculated_step)))
                .max_by(|x, y| x.1.cmp(&y.1)).unwrap();

    println!("Most steps for: {:?}", most_steps);
}
