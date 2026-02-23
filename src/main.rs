mod libs;

use iced::widget::{button, column, container, pick_list, row, scrollable, space, text};
use iced::{Alignment, Border, Color, Element, Length, Length::Fill, Theme, color};
use std::ffi::CStr;
use std::fs;

use crate::libs::DLLAlgorithm;

#[derive(Default)]
struct State {
    selected_alg: Option<String>,
    table_data: Vec<Vec<i32>>,
    error: Option<String>,
    result_str: Option<String>,
    algs: Vec<DLLAlgorithm>,
}

#[derive(Debug, Clone)]
enum Message {
    AlgSelected(String),
    LoadTable,
    RunTask,
    LoadDLLs,
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
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("CSV", &["csv"])
                    .pick_file()
                {
                    match self.parse_csv(&path.to_string_lossy()) {
                        Ok(data) => {
                            self.table_data = data;
                            self.error = None;
                            self.generate_table();
                        }
                        Err(e) => self.error = Some(format!("Ошибка парсинга: {}", e)),
                    }
                }
            }
            Message::RunTask => match self.selected_alg.clone() {
                Some(alg_name) => {
                    self.result_str = Some("Выполнение алгоритма".to_string());

                    self.result_str = self
                        .algs
                        .iter()
                        .find(|alg| unsafe {
                            CStr::from_ptr((alg.name)())
                                .to_str()
                                .map(|n| n == alg_name)
                                .unwrap_or(false)
                        })
                        .map(|alg| alg.run(&self.table_data));
                    self.error = None;
                    println!("{:?}", self.result_str);
                }
                None => {
                    self.result_str = None;
                    self.error = Some("Алгоритм не выбран".to_string());
                }
            },
            Message::LoadDLLs => {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    let directory = path.as_path().to_str().unwrap();
                    println!("dir: {}", directory);

                    self.algs = DLLAlgorithm::load_all(directory);

                    let mut names = String::from("Загружены библиотеки:\n");
                    for alg in self.algs.iter().clone() {
                        let name = unsafe {
                            std::ffi::CStr::from_ptr((alg.name)())
                                .to_str()
                                .unwrap()
                                .to_string()
                        } + "\n";
                        names.push_str(name.as_str());
                    }
                    self.result_str = Some(names);
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let all_alg_names: Vec<String> = self
            .algs
            .iter()
            .map(|alg| unsafe { CStr::from_ptr((alg.name)()).to_str().unwrap().to_string() })
            .collect();
        let pick_list = pick_list(
            all_alg_names,
            self.selected_alg.clone(),
            Message::AlgSelected,
        );

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
                    button(text("Загрузить").align_x(Alignment::Center))
                        .width(100)
                        .on_press(Message::LoadDLLs),
                    text("Алгоритм").width(80).align_y(Alignment::Center),
                    space().width(20),
                    pick_list.width(450),
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
