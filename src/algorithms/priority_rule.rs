use crate::algorithms::common::{AlgResult, create_result};

pub fn priority_rule(matrix: &Vec<Vec<i32>>) -> Result<AlgResult, String> {
    if matrix[0].len() != 2 {
        return Err("Нужно 2 станка".into());
    }
    let max_val = matrix.iter().flatten().copied().max().unwrap_or(1);
    let mut jobs: Vec<(usize, i32)> = matrix
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let (a, b) = (r[0], r[1]);
            let sign = if a < b { 1 } else { -1 };
            let p = sign * (max_val - a.min(b));
            (i, p)
        })
        .collect();
    jobs.sort_by_key(|k| std::cmp::Reverse(k.1));
    let sequence = jobs.into_iter().map(|(i, _)| i).collect();

    create_result(matrix, sequence, "Метод приоритетов")
}
