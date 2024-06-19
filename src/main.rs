use rayon::prelude::*;
use sha256::digest;

fn src(n: usize) -> String {
    format!("zdimension/5900X/60MHs/{}", n)
}

fn hash(n: usize) -> String {
    digest(src(n))
}

fn score(n: usize) -> usize {
    hash(n).find(|c| c != '0').unwrap()
}

fn main() {
    let mut global_best = 0;
    let mut start = 57_700_000_000;
    const BATCH_SIZE: usize = 100_000_000;
    loop {
        let range = start..start + BATCH_SIZE;

        let begin = chrono::Utc::now();
        let best = range.into_par_iter().max_by_key(|n| score(*n)).unwrap();
        let score = score(best);
        let elapsed = chrono::Utc::now() - begin;
        let hps = BATCH_SIZE as f64 / elapsed.num_milliseconds() as f64 * 1000.0;

        start += BATCH_SIZE;

        print!("{start:>20} @ {:5.2} MH/s -- ", hps / 1e6);

        if score > global_best {
            println!("Best hash in {:>3} : {:<25} -> {}", score, src(best), hash(best));
            global_best = score;
        } else {
            println!("No better hash found (best: {:>3}, global: {:>3})", score, global_best);
        }
    }
}
