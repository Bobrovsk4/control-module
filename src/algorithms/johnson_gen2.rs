use crate::{
    algorithms::common::{AlgResult, create_result},
    gantt_chart::draw_gantt,
};

pub fn johnson_gen2(matrix: &Vec<Vec<i32>>) -> Result<(AlgResult, i32), String> {
    let last = matrix[0].len() - 1;
    let mut jobs: Vec<(usize, i32)> = matrix
        .iter()
        .enumerate()
        .map(|(i, r)| (i, r[last]))
        .collect();
    jobs.sort_by_key(|k| std::cmp::Reverse(k.1));
    let sequence = jobs.into_iter().map(|(i, _)| i).collect();

    let orig_seq: Vec<usize> = (0..matrix.len()).collect();
    let orig_result = create_result(
        matrix,
        orig_seq,
        "Джонсон макс. время на последнем станке (исходный)",
    );

    let final_result = create_result(
        matrix,
        sequence,
        "Джонсон макс. время на последнем станке (финальный)",
    );

    draw_gantt(&orig_result.clone()?, &matrix.clone(), "orig.svg");
    draw_gantt(&final_result.clone()?, &matrix.clone(), "final.svg");

    Ok((final_result.unwrap(), orig_result.unwrap().makespan))
}
