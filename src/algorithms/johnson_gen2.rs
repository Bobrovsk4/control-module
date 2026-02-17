use crate::algorithms::common::{AlgResult, create_result};

pub fn johnson_gen2(matrix: &Vec<Vec<i32>>) -> Result<AlgResult, String> {
    let last = matrix[0].len() - 1;
    let mut jobs: Vec<(usize, i32)> = matrix
        .iter()
        .enumerate()
        .map(|(i, r)| (i, r[last]))
        .collect();
    jobs.sort_by_key(|k| std::cmp::Reverse(k.1));
    let sequence = jobs.into_iter().map(|(i, _)| i).collect();

    create_result(
        matrix,
        sequence,
        "Джонсон (макс. время на последнем станке)",
    )
}
