#[derive(Debug, Clone)]
pub struct AlgResult {
    pub sequence: Vec<usize>,
    pub schedule: Vec<Vec<(i32, i32)>>,
    pub makespan: i32,
    pub idle_times: Vec<i32>,
}

pub fn algorithm(matrix: &Vec<Vec<i32>>) -> Result<AlgResult, String> {
    if matrix.is_empty() {
        return Err("Матрица пуста".to_string());
    }

    let num_machines = matrix[0].len();

    if num_machines == 0 {
        return Err("Матрица содержит пустые строки".to_string());
    }

    for (i, row) in matrix.iter().enumerate() {
        if row.len() != num_machines {
            return Err(format!(
                "Неравномерная матрица: строка {} имеет {} элементов, ожидалось {}",
                i,
                row.len(),
                num_machines
            ));
        }
        for (j, &time) in row.iter().enumerate() {
            if time < 0 {
                return Err(format!(
                    "Отрицательное время обработки в работе {} на станке {}",
                    i, j
                ));
            }
        }
    }

    let sequence = if num_machines == 2 {
        johnson_two_machines(matrix)
    } else if num_machines > 2 {
        johnson_heuristic_multi_machine(matrix)
    } else {
        return Err("Требуется минимум 2 станка для алгоритма Джонсона".to_string());
    };

    let (schedule, makespan, idle_times) = build_schedule(matrix, &sequence)?;

    Ok(AlgResult {
        sequence,
        schedule,
        makespan,
        idle_times,
    })
}

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

fn johnson_two_machines(matrix: &Vec<Vec<i32>>) -> Vec<usize> {
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

    sequence
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

fn johnson_heuristic_multi_machine(matrix: &Vec<Vec<i32>>) -> Vec<usize> {
    let num_machines = matrix[0].len();
    let k = (num_machines + 1) / 2;

    let pseudo_jobs: Vec<(usize, i32, i32)> = matrix
        .iter()
        .enumerate()
        .map(|(idx, times)| {
            let a: i32 = times[0..k].iter().sum();
            let b: i32 = times[(num_machines - k)..num_machines].iter().sum();
            (idx, a, b)
        })
        .collect();

    let mut jobs = pseudo_jobs;
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

    sequence
}

pub fn build_schedule(
    matrix: &Vec<Vec<i32>>,
    sequence: &Vec<usize>,
) -> Result<(Vec<Vec<(i32, i32)>>, i32, Vec<i32>), String> {
    let num_jobs = sequence.len();
    let num_machines = matrix[0].len();

    let mut schedule = vec![vec![(0, 0); num_machines]; num_jobs];

    let first_job = sequence[0];
    let mut current_time = 0;
    for machine in 0..num_machines {
        let proc_time = matrix[first_job][machine];
        schedule[0][machine] = (current_time, current_time + proc_time);
        current_time += proc_time;
    }

    for (seq_idx, &job_idx) in sequence.iter().enumerate().skip(1) {
        let prev_out_m1 = schedule[seq_idx - 1][0].1;
        let proc_time_m1 = matrix[job_idx][0];
        schedule[seq_idx][0] = (prev_out_m1, prev_out_m1 + proc_time_m1);

        for machine in 1..num_machines {
            let out_prev_machine = schedule[seq_idx][machine - 1].1;
            let out_prev_job = schedule[seq_idx - 1][machine].1;
            let in_time = out_prev_machine.max(out_prev_job);
            let proc_time = matrix[job_idx][machine];
            schedule[seq_idx][machine] = (in_time, in_time + proc_time);
        }
    }

    let makespan = schedule[num_jobs - 1][num_machines - 1].1;

    let mut idle_times = vec![0; num_machines];
    for machine in 0..num_machines {
        let mut total_idle = schedule[0][machine].0;

        for seq_idx in 1..num_jobs {
            let gap = schedule[seq_idx][machine].0 - schedule[seq_idx - 1][machine].1;
            if gap > 0 {
                total_idle += gap;
            }
        }

        idle_times[machine] = total_idle;
    }

    Ok((schedule, makespan, idle_times))
}
