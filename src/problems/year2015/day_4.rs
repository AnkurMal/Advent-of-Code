use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn part_1() {
    println!("Part 1: {}", generate(5));
}

pub fn part_2() {
    println!("Part 2: {}", generate(6));
}

fn generate(n: usize) -> u32 {
    const CHUNK_SIZE: u32 = 10000;
    let check = "0".repeat(n);
    let mut start = 1;
    
    loop {
        let end = start + CHUNK_SIZE;
        
        if let Some(result) = (start..end)
            .into_par_iter()
            .find_first(|&num| {
                let key = format!("yzbqklnj{num}");
                let hash = format!("{:x}", md5::compute(key));
                hash.starts_with(&check)
            })
        {
            return result;
        }
        
        start = end;
    }
}