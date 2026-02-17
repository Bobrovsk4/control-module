use crate::algorithms::common::{AlgResult, create_result};

pub fn johnson_gen3(matrix: &Vec<Vec<i32>>) -> Result<AlgResult, String> {
    let mut jobs: Vec<(usize, usize)> = matrix
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let (max_idx, _) = r.iter().enumerate().max_by_key(|&(_, v)| v).unwrap();
            (i, max_idx)
        })
        .collect();
    jobs.sort_by_key(|k| std::cmp::Reverse(k.1));
    let sequence = jobs.into_iter().map(|(i, _)| i).collect();

    create_result(matrix, sequence, "Джонсон (приоритет «узкого места»)")
}
