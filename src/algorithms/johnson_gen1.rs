use crate::algorithms::common::{AlgResult, create_result};

pub fn johnson_gen1(matrix: &Vec<Vec<i32>>) -> Result<AlgResult, String> {
    let mut jobs: Vec<(usize, i32)> = matrix.iter().enumerate().map(|(i, r)| (i, r[0])).collect();
    jobs.sort_by_key(|k| k.1);
    let sequence = jobs.into_iter().map(|(i, _)| i).collect();

    create_result(matrix, sequence, "Джонсон (мин. время на 1-м станке)")
}
