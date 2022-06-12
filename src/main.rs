use iced::{
    button, slider, Alignment, Application, Button, Color, Column, Command, Container, Element,
    Row, Settings, Slider, Text,
};

//use iced::{text_input, TextInput};
fn main() -> iced::Result {
    Metronome::run(Settings::default())
    //Counter::run(Settings::default())
}
//#[derive(Default)]
struct Metronome {
    ticks: u64,
    start: bool,
    left: bool,
    start_button: button::State,
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
    Add,
    Decrease,
    Stop,
}
impl Metronome {
    fn new() -> Self {
        Metronome {
            ticks: 100,
            start: false,
            left: true,
            start_button: button::State::default(),
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
                self.left = !self.left;
                Command::perform(
                    async move {
                        tokio::time::sleep(tokio::time::Duration::from_nanos(time)).await;
                    },
                    |_| PollMessage::Continue,
                )
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
                let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
                let sink = rodio::Sink::try_new(&handle).unwrap();

                let file = std::fs::File::open("assets/metronome.wav").unwrap();
                let a = std::io::BufReader::new(file);
                sink.append(rodio::Decoder::new(a).unwrap());

                sink.sleep_until_end();
                let time = self.ticks;
                self.left = !self.left;
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
                )
                .push(
                    Row::new()
                        .push(block::Block::new(
                            100.0,
                            if self.left {
                                Color::BLACK
                            } else {
                                Color::from_rgb(0.9, 0.8, 0.9)
                            },
                        ))
                        .push(block::Block::new(
                            100.0,
                            if !self.left {
                                Color::BLACK
                            } else {
                                Color::from_rgb(0.9, 0.8, 0.9)
                            },
                        )),
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

mod block {
    use iced_native::layout::{self, Layout};
    use iced_native::renderer;
    use iced_native::{Color, Element, Length, Point, Rectangle, Size, Widget};

    pub struct Block {
        radius: f32,
        color: Color,
    }

    impl Block {
        pub fn new(radius: f32, color: Color) -> Self {
            Self { radius, color }
        }
    }

    impl<Message, Renderer> Widget<Message, Renderer> for Block
    where
        Renderer: renderer::Renderer,
    {
        fn width(&self) -> Length {
            Length::Shrink
        }

        fn height(&self) -> Length {
            Length::Shrink
        }

        fn layout(&self, _renderer: &Renderer, _limits: &layout::Limits) -> layout::Node {
            layout::Node::new(Size::new(self.radius * 2.0, self.radius * 2.0))
        }

        fn draw(
            &self,
            renderer: &mut Renderer,
            _style: &renderer::Style,
            layout: Layout<'_>,
            _cursor_position: Point,
            _viewport: &Rectangle,
        ) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border_radius: 0.0,
                    border_width: 10.0,
                    border_color: Color::TRANSPARENT,
                },
                self.color,
            );
        }
    }

    impl<'a, Message, Renderer> Into<Element<'a, Message, Renderer>> for Block
    where
        Renderer: renderer::Renderer,
    {
        fn into(self) -> Element<'a, Message, Renderer> {
            Element::new(self)
        }
    }
}
