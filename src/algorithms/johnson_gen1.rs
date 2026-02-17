fn johnson_gen1(matrix: &Vec<Vec<i32>>) -> Result<Vec<usize>, String> {
    let mut jobs: Vec<(usize, i32)> = matrix.iter().enumerate().map(|(i, r)| (i, r[0])).collect();
    jobs.sort_by_key(|k| k.1);
    Ok(jobs.into_iter().map(|(i, _)| i).collect())
}
