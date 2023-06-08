use iced::{
    widget::button, widget::Button, widget::Column, widget::Container, widget::Row, widget::Slider,
    widget::Text, Alignment, Application, Color, Command, Element, Settings, Theme,
};

mod block;

fn main() -> iced::Result {
    Metronome::run(Settings::default())
}

struct Metronome {
    ticks: u64,
    start: bool,
    length: (usize, usize),
}

#[derive(Debug, Clone)]
pub enum PollMessage {
    Start,
    Continue,
    Update(f32),
    AddStep,
    DecreaseStep,
    Add,
    Decrease,
    Stop,
}

impl Metronome {
    // create a new Metronome
    fn new() -> Self {
        Metronome {
            ticks: 100,
            start: false,
            length: (2, 0),
        }
    }
}

impl Application for Metronome {
    type Message = PollMessage;
    type Executor = iced::executor::Default;
    type Flags = ();
    type Theme = Theme;
    fn new(_flags: ()) -> (Metronome, Command<PollMessage>) {
        (Self::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Metronome")
    }

    // message get from background
    fn update(&mut self, message: PollMessage) -> Command<PollMessage> {
        //if self.start {
        match message {
            PollMessage::Update(ticks) => {
                self.ticks = ticks as u64;
                Command::none()
            }
            PollMessage::Start => {
                let time = self.ticks;
                self.start = true;
                //self.left = !self.left;
                Command::perform(
                    async move {
                        tokio::time::sleep(tokio::time::Duration::from_nanos(time)).await;
                    },
                    |_| PollMessage::Continue,
                )
            }
            PollMessage::AddStep => {
                let (length, _) = self.length;
                if length < 6 {
                    self.length.0 += 1;
                    self.length.1 = 0;
                }
                Command::none()
            }
            PollMessage::DecreaseStep => {
                let (length, _) = self.length;
                if length > 2 {
                    self.length.0 -= 1;
                    self.length.1 = 0;
                }
                Command::none()
            }
            PollMessage::Add => {
                if self.ticks < 1000 {
                    self.ticks += 1;
                }
                Command::none()
            }
            PollMessage::Decrease => {
                if self.ticks > 1 {
                    self.ticks -= 1;
                }
                Command::none()
            }
            PollMessage::Continue => {
                let (length, local) = self.length;
                if local < length - 1 {
                    self.length.1 += 1;
                } else {
                    self.length.1 = 0;
                }
                let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
                let sink = rodio::Sink::try_new(&handle).unwrap();

                let file = std::fs::File::open("assets/metronome.wav").unwrap();
                let a = std::io::BufReader::new(file);
                sink.append(rodio::Decoder::new(a).unwrap());

                sink.sleep_until_end();
                let time = self.ticks;
                //self.left = !self.left;
                if self.start {
                    Command::perform(
                        async move {
                            //println!("Start");
                            tokio::time::sleep(tokio::time::Duration::from_millis(time)).await;
                        },
                        |_| PollMessage::Continue,
                    )
                } else {
                    Command::none()
                }
            }
            PollMessage::Stop => {
                //println!("Stop");
                self.start = false;
                Command::none()
            }
        }
        //} else {
        //    Command::none()
        //}
    }

    fn view(&self) -> Element<PollMessage> {
        Container::new(
            Column::new()
                .spacing(30)
                .padding(20)
                .max_width(540)
                .align_items(Alignment::Center)
                .push(Text::new("Metronome").size(60))
                .push({
                    let mut shown = Row::new();
                    let (length, location) = self.length;
                    for i in 0..length {
                        shown = shown.push(block::Block::new(
                            20.0,
                            if i == location {
                                Color::BLACK
                            } else {
                                Color::from_rgb(0.9, 0.8, 0.9)
                            },
                            if i == location && i == 0 {
                                block::Kind::Squre
                            } else {
                                block::Kind::Dot
                            },
                        ))
                    }
                    shown
                })
                .push(
                    Row::new()
                        .spacing(10)
                        .push(
                            button(" -").on_press(PollMessage::Decrease), //.style(style::Button::Liner),
                        )
                        .push(
                            Slider::new(1.0..=1000.0, self.ticks as f32, PollMessage::Update)
                                .step(1.0)
                                .width(iced::Length::Fill)
                                .height(30),
                        )
                        .push(
                            button("+").on_press(PollMessage::Add), //.style(style::Button::Liner),
                        ),
                )
                .push(Text::new(format!("{} BPM", self.ticks)).size(50))
                .push(
                    Row::new()
                        .push(button("D").on_press(PollMessage::DecreaseStep))
                        .push(block::Spring)
                        .push(
                            button("A").on_press(PollMessage::AddStep), //.style(style::Button::Liner),
                        ),
                )
                .push(
                    Button::new(if self.start {
                        Text::new("Stop").size(60)
                    } else {
                        Text::new("Start").size(60)
                    })
                    .on_press(if self.start {
                        PollMessage::Stop
                    } else {
                        PollMessage::Start
                    })
                    .width(iced::Length::Shrink), //.style(style::Button::Primary),
                ),
        )
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .center_y()
        .center_x()
        .into()
    }
}
//mod style {
//    use iced::{widget::button, Background, Color, Vector};
//    pub enum Button {
//        Primary,
//        Liner,
//    }
//    impl button::StyleSheet for Button {
//        fn active(&self) -> button::Style {
//            button::Style {
//                background: Some(Background::Color(match self {
//                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
//                    Button::Liner => Color::from_rgb(0.9, 0.7, 0.8),
//                })),
//                border_radius: 12.0,
//                shadow_offset: Vector::new(1.0, 1.0),
//                text_color: Color::WHITE,
//                ..button::Style::default()
//            }
//        }
//    }
//}
