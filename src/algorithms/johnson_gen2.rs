fn johnson_gen2(matrix: &Vec<Vec<i32>>) -> Result<Vec<usize>, String> {
    let last = matrix[0].len() - 1;
    let mut jobs: Vec<(usize, i32)> = matrix
        .iter()
        .enumerate()
        .map(|(i, r)| (i, r[last]))
        .collect();
    jobs.sort_by_key(|k| std::cmp::Reverse(k.1));
    Ok(jobs.into_iter().map(|(i, _)| i).collect())
}
