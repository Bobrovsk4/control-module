use iced::widget::{button, column, container, pick_list, row, scrollable, space, text};
use iced::{Alignment, Color, Element, Length, Theme, Length::Fill};
use std::fs;

#[derive(Default)]
struct State {
   selected_alg: Option<Algorithm>,
   table_data: Vec<Vec<i32>>,
   error: Option<String>,
}

#[derive(Debug, Clone)]
enum Message {
    AlgSelected(Algorithm),
    LoadTable,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Algorithm {
    #[default]
    Jonson,
    Petrov_Sokolicyn,
    BrandAndBound, // ветвей и границ
}

impl Algorithm {
    const ALL: [Algorithm; 3] = [
        Algorithm::Jonson,
        Algorithm::Petrov_Sokolicyn,
        Algorithm::BrandAndBound
    ];
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Algorithm::Jonson => "Алгоритм Джонсона",
                Algorithm::Petrov_Sokolicyn => "Метод Петрова-Соколицына",
                Algorithm::BrandAndBound => "Метод ветвей и границ"
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
            
            // Пропускаем пустые строки после фильтрации пробелов
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

                row(cells)
                    .spacing(4)
                    .align_y(Alignment::Center)
                    .into()
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
            },
            Message::LoadTable => {
                // Синхронная загрузка (блокирует UI на время операции)
                match self.parse_csv(
                    rfd::FileDialog::new()
                    .add_filter("CSV only", &["csv"])
                    .pick_file()
                    .expect("Ошибка выбора файла")
                    .to_str()
                    .expect("Ошибка преобразования названия файла в строку")
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
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let pick_list = pick_list(&Algorithm::ALL[..], self.selected_alg, Message::AlgSelected);

        let error_message = if let Some(err) = &self.error {
            Some(
                text(format!("{}", err))
            )
        } else {
            None
        };

        row![
            column![
                space().height(10),
                "Алгоритм",
                pick_list,
                space().height(10),
                row![
                    text("Таблица"),
                    space().width(20),
                    button(
                        text("Выбрать файл")
                        .width(140)
                        .align_x(Alignment::Center),
                    ).on_press(Message::LoadTable)
                ],
                space().height(10),
                error_message,
                self.generate_table()
            ],
            
        ]
        .width(500).height(1000)
        .spacing(10)
        .into()
    }
}

fn main() -> iced::Result {
    iced::run(State::update, State::view)
}
