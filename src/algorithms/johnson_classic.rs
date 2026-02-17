use crate::algorithms::common::{AlgResult, create_result};

pub fn format_result(result: &AlgResult, matrix: &Vec<Vec<i32>>) -> String {
    let mut output = String::new();

    if matrix[0].len() == 2 {
        output.push_str("Оптимальная последовательность:\n");
    } else {
        output.push_str("Последовательность:\n");
    }

    output.push_str(&format!(
        "  {}\n",
        result
            .sequence
            .iter()
            .map(|&idx| format!("J{}", idx + 1))
            .collect::<Vec<_>>()
            .join(" → ")
    ));

    output.push_str("\nРасписание (вход → выход):\n");
    output.push_str("Работа| ");
    for machine in 0..matrix[0].len() {
        output.push_str(&format!("   M{}  | ", machine + 1));
    }
    output.push_str("\n");
    output.push_str(&"-------+".repeat(matrix[0].len()));
    output.push_str("-\n");

    for (seq_idx, &job_idx) in result.sequence.iter().enumerate() {
        output.push_str(&format!("   J{}    | ", job_idx + 1));
        for machine in 0..matrix[0].len() {
            let (in_time, out_time) = result.schedule[seq_idx][machine];
            output.push_str(&format!(" {:2}→{:2} |", in_time, out_time));
        }
        output.push_str("\n");
    }

    output.push_str(&format!(
        "\nДлительность производственного цикла: {}\n",
        result.makespan
    ));
    output.push_str("Простои станков:\n");
    for (machine, &idle) in result.idle_times.iter().enumerate() {
        output.push_str(&format!("M{}: {}\n", machine + 1, idle));
    }

    output
}

pub fn johnson_classic(matrix: &Vec<Vec<i32>>) -> Result<AlgResult, String> {
    let mut jobs: Vec<(usize, i32, i32)> = matrix
        .iter()
        .enumerate()
        .map(|(idx, times)| (idx, times[0], times[1]))
        .collect();

    let mut sequence = vec![0usize; jobs.len()];
    let mut left = 0;
    let mut right = jobs.len() - 1;

    while !jobs.is_empty() {
        let (min_idx, min_machine) = find_min_job(&jobs);
        let (job_idx, _, _) = jobs.remove(min_idx);

        if min_machine == 0 {
            sequence[left] = job_idx;
            left += 1;
        } else {
            sequence[right] = job_idx;
            if right > 0 {
                right -= 1;
            }
        }
    }

    create_result(matrix, sequence, "Джонсон (классический)")
}

fn find_min_job(jobs: &[(usize, i32, i32)]) -> (usize, usize) {
    let mut min_value = i32::MAX;
    let mut min_idx = 0;
    let mut min_machine = 0;

    for (i, &(_, m1, m2)) in jobs.iter().enumerate() {
        if m1 < min_value {
            min_value = m1;
            min_idx = i;
            min_machine = 0;
        }
        if m2 < min_value {
            min_value = m2;
            min_idx = i;
            min_machine = 1;
        }
    }

    (min_idx, min_machine)
}
