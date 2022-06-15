use iced::{
    button, slider, Alignment, Application, Button, Color, Column, Command, Container, Element,
    Row, Settings, Slider, Text,
};

mod block; 
//use iced::{text_input, TextInput};
fn main() -> iced::Result {
    Metronome::run(Settings::default())
    //Counter::run(Settings::default())
}
/// Metronome has
/// lenght in , first is length, second is location
struct Metronome {
    ticks: u64,
    start: bool,
    /// length and location
    length: (usize, usize),
    start_button: button::State,
    add_step_button: button::State,
    decrease_step_button: button::State,
    add_button: button::State,
    decrease_button: button::State,
    slider: slider::State,
    //text: text_input::State,
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
            start_button: button::State::default(),
            add_step_button: button::State::default(),
            decrease_step_button: button::State::default(),
            add_button: button::State::default(),
            decrease_button: button::State::default(),
            slider: slider::State::default(),
            //text: text_input::State::default(),
        }
    }
}
impl Application for Metronome {
    type Message = PollMessage;
    type Executor = iced::executor::Default;
    type Flags = ();
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

    fn view(&mut self) -> Element<PollMessage> {
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
                            Button::new(&mut self.decrease_button, Text::new(" -").size(20))
                                .on_press(PollMessage::Decrease)
                                .style(style::Button::Liner),
                        )
                        .push(
                            Slider::new(
                                &mut self.slider,
                                1.0..=1000.0,
                                self.ticks as f32,
                                PollMessage::Update,
                            )
                            .step(1.0)
                            .width(iced::Length::Fill)
                            .height(30),
                        )
                        .push(
                            Button::new(&mut self.add_button, Text::new("+").size(20))
                                .on_press(PollMessage::Add)
                                .style(style::Button::Liner),
                        ),
                )
                .push(Text::new(format!("{} BPM", self.ticks)).size(50))
                .push(
                    Row::new()
                        .push(
                            Button::new(&mut self.decrease_step_button, Text::new("D").size(30))
                                .on_press(PollMessage::DecreaseStep)
                                .style(style::Button::Liner),
                        )
                        .push(block::Spring)
                        .push(
                            Button::new(&mut self.add_step_button, Text::new("A").size(30))
                                .on_press(PollMessage::AddStep)
                                .style(style::Button::Liner),
                        ),
                )
                .push(
                    Button::new(
                        &mut self.start_button,
                        if self.start {
                            Text::new("Stop").size(60)
                        } else {
                            Text::new("Start").size(60)
                        },
                    )
                    .on_press(if self.start {
                        PollMessage::Stop
                    } else {
                        PollMessage::Start
                    })
                    .width(iced::Length::Shrink)
                    .style(style::Button::Primary),
                ),
        )
        .height(iced::Length::Fill)
        .width(iced::Length::Fill)
        .center_y()
        .center_x()
        .into()
    }
}
mod style {
    use iced::{button, Background, Color, Vector};
    pub enum Button {
        Primary,
        Liner,
    }
    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Liner => Color::from_rgb(0.9, 0.7, 0.8),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}
