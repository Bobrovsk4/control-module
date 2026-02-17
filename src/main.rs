mod algorithms;

use iced::widget::{button, column, container, pick_list, row, scrollable, space, text};
use iced::{Alignment, Border, Color, Element, Length, Length::Fill, Theme, color};
use std::fs;

use crate::algorithms::{branch_and_bound, johnson_classic, petrov_sokolicyn};

#[derive(Default)]
struct State {
    selected_alg: Option<Algorithm>,
    table_data: Vec<Vec<i32>>,
    error: Option<String>,
    result_str: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    AlgSelected(Algorithm),
    LoadTable,
    RunTask,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Algorithm {
    #[default]
    JohnsonClassic, // 1.1 (2 станка)
    JohnsonGen1,      // 1.1 (Min M1 -> start)
    JohnsonGen2,      // 1.1 (Max Mn -> end)
    JohnsonGen3,      // 1.1 (Bottleneck index -> priority)
    JohnsonGen4,      // 1.1 (Max Sum -> start)
    PriorityRule,     // 1.2 (Pq formula)
    BruteForce,       // 1.3 (All permutations)
    PetrovSokolitsyn, // 2 (3 candidates)
    BranchAndBound,
}

impl Algorithm {
    const ALL: [Algorithm; 9] = [
        Algorithm::JohnsonClassic,
        Algorithm::JohnsonGen1,
        Algorithm::JohnsonGen2,
        Algorithm::JohnsonGen3,
        Algorithm::JohnsonGen4,
        Algorithm::PriorityRule,
        Algorithm::BruteForce,
        Algorithm::PetrovSokolitsyn,
        Algorithm::BranchAndBound,
    ];
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Algorithm::Johnson => "Алгоритм Джонсона",
                Algorithm::Petrov_Sokolicyn => "Метод Петрова-Соколицына",
                Algorithm::BrandAndBound => "Метод ветвей и границ",
            }
        )
    }
}

impl State {
    fn parse_csv(&mut self, path: &str) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path).expect("Ошибка чтения файла");

        let mut table = Vec::new();

        for (line_num, line) in content
            .lines()
            .enumerate()
            .filter(|(_, line)| !line.trim().is_empty() && !line.trim().starts_with('#'))
        {
            let cells: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

            if cells.iter().all(|s| s.is_empty()) {
                continue;
            }

            let row: Result<Vec<i32>, _> = cells
                .iter()
                .map(|cell| {
                    cell.parse::<i32>().map_err(|_| {
                        format!(
                            "Ошибка в строке {}: не удалось распарсить \"{}\" как число",
                            line_num + 1,
                            cell
                        )
                    })
                })
                .collect();

            table.push(row?);
        }

        if table.is_empty() {
            return Err("Данные в файле не найдены".into());
        }

        Ok(table)
    }

    fn generate_table(&self) -> Element<'_, Message> {
        let data = &self.table_data;
        let table_rows: Vec<Element<'_, Message>> = (0..data.len())
            .map(|row_idx| {
                let cells: Vec<Element<'_, Message>> = (0..data[0].len())
                    .map(|col_idx| {
                        container(
                            text(format!("{}", data[row_idx][col_idx].to_string()))
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .align_x(Alignment::Center)
                                .align_y(Alignment::Center),
                        )
                        .width(50)
                        .height(25)
                        .padding(8)
                        .style(|_theme: &Theme| container::Style {
                            border: iced::Border {
                                radius: 4.0.into(),
                                width: 1.5,
                                color: Color::from_rgb8(80, 90, 110),
                            },
                            ..Default::default()
                        })
                        .into()
                    })
                    .collect();

                row(cells).spacing(4).align_y(Alignment::Center).into()
            })
            .collect();

        scrollable(column(table_rows).spacing(4).padding(8))
            .direction(scrollable::Direction::Both {
                horizontal: scrollable::Scrollbar::new(),
                vertical: scrollable::Scrollbar::new(),
            })
            .width(Fill)
            .height(300)
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AlgSelected(alg) => {
                self.selected_alg = Some(alg);
            }
            Message::LoadTable => {
                match self.parse_csv(
                    rfd::FileDialog::new()
                        .add_filter("CSV only", &["csv"])
                        .pick_file()
                        .expect("Ошибка выбора файла")
                        .to_str()
                        .expect("Ошибка преобразования названия файла в строку"),
                ) {
                    Ok(data) => {
                        self.table_data = data;
                        self.error = None;
                        self.generate_table();
                    }
                    Err(e) => {
                        self.error = Some(format!("Ошибка парсинга: {}", e));
                    }
                }
            }
            Message::RunTask => match self.selected_alg {
                Some(Algorithm::Johnson) => {
                    self.result_str = Some(johnson_classic::format_result(
                        &johnson_classic::algorithm(&self.table_data)
                            .expect("Ошибка выполнения алгоритма"),
                        &self.table_data,
                    ));
                    self.error = None;
                }
                Some(Algorithm::Petrov_Sokolicyn) => {
                    self.result_str = Some(petrov_sokolicyn::format_result(
                        &petrov_sokolicyn::algorithm(&self.table_data)
                            .expect("Ошибка выполнения алгоритма"),
                        &self.table_data,
                    ));
                    self.error = None;
                }
                Some(Algorithm::BrandAndBound) => {
                    match branch_and_bound::algorithm(&self.table_data, 5000, 1_000_000) {
                        Ok((result, stats)) => {
                            self.result_str = Some(branch_and_bound::format_result(
                                &result,
                                &stats,
                                &self.table_data,
                            ));
                            self.error = None;
                        }
                        Err(e) => {
                            self.error = Some(format!("Ошибка метода ветвей и границ:\n{}", e));
                            self.result_str = None;
                        }
                    }
                }
                None => {
                    self.error = Some("Алгоритм не выбран".to_string());
                }
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let pick_list = pick_list(&Algorithm::ALL[..], self.selected_alg, Message::AlgSelected);

        let error_message = if let Some(err) = &self.error {
            Some(text(format!("{}", err)))
        } else {
            None
        };

        let res_str = if let Some(res) = &self.result_str {
            Some(text(res).width(Length::Fill))
        } else {
            None
        };

        container(row![
            column![
                space().height(10),
                row![
                    text("Алгоритм").width(80).align_y(Alignment::Center),
                    space().width(20),
                    pick_list.width(300),
                ],
                space().height(10),
                row![
                    text("Таблица").width(80).align_y(Alignment::Center),
                    space().width(20),
                    button(text("Выбрать файл").align_x(Alignment::Center),)
                        .width(300)
                        .on_press(Message::LoadTable)
                ],
                space().height(10),
                error_message,
                self.generate_table(),
                space().height(10),
                button(text("Выполнить").align_x(Alignment::Center))
                    .width(380)
                    .on_press(Message::RunTask)
            ]
            .align_x(Alignment::Center),
            column![
                space().height(10),
                container(
                    scrollable(res_str)
                        .direction(scrollable::Direction::Both {
                            horizontal: scrollable::Scrollbar::new(),
                            vertical: scrollable::Scrollbar::new(),
                        })
                        .width(Length::Fill)
                        .height(Length::Fill)
                )
                .padding(10)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_theme: &Theme| container::Style {
                    background: Some(color!(40, 40, 40).into()),
                    border: Border {
                        radius: 4.0.into(),
                        color: color!(110, 110, 110).into(),
                        width: 1.0
                    },
                    ..Default::default()
                })
            ]
            .width(Length::Fill)
            .align_x(Alignment::Center),
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .into()
    }
}

fn main() -> iced::Result {
    iced::run(State::update, State::view)
}
