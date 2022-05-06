use iced::{
    button, Alignment, Application, Button, Column, Command, Element, Row, Settings, Space, Text,
};
mod backend;
mod page;
fn main() -> iced::Result {
    Counter::run(Settings::default())
}
use backend::start;
use page::{StepMessage, Steps};
use tokio::sync::mpsc::{channel, Sender};
//#[derive(Default)]
struct Counter {
    steps: Steps,
    io_tx: Sender<Events>,
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
    jump_button: button::State,
    log_button: button::State,
    say_button: button::State,
    back_button: button::State,
    font_button: button::State,
}
impl Counter {
    fn new(sender: Sender<Events>) -> Self {
        Counter {
            steps: Steps::new(),
            io_tx: sender,
            value: 0,
            increment_button: button::State::default(),
            decrement_button: button::State::default(),
            jump_button: button::State::default(),
            log_button: button::State::default(),
            say_button: button::State::default(),
            back_button: button::State::default(),
            font_button: button::State::default(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Events {
    Log,
    Say,
}
#[derive(Debug, Clone)]
pub enum Arrow {
    Front,
    Back,
}
#[derive(Debug, Clone)]
pub enum Message {
    JumpIncreasePressed(i32),
    EndIncreasePressed(i32),
    IncrementPressed,
    DecrementPressed,
    Docommand(Events),
    SendDone,
    StepMessage(StepMessage),
    Go(Arrow),
}


impl Application for Counter {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();
    fn new(_flags: ()) -> (Counter, Command<Message>) {
        let (sync_io_tx, sync_io_rx) = channel::<Events>(100);
        (
            Self::new(sync_io_tx),
            Command::perform(start(sync_io_rx), Message::JumpIncreasePressed),
        )
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Go(arrow) => match arrow {
                Arrow::Back => self.steps.preview(),
                Arrow::Front => self.steps.next(),
            },
            Message::StepMessage(message) => {
                self.steps.update(message);
            }
            Message::JumpIncreasePressed(number) => {
                return Command::perform(update(number), Message::EndIncreasePressed);
            }
            Message::EndIncreasePressed(number) => {
                self.value += number;
            }
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::Docommand(command) => {
                let io_tx = self.io_tx.clone();
                return Command::perform(
                    async move {
                        io_tx.send(command).await.unwrap();
                    },
                    |_| Message::SendDone,
                );
            }
            Message::SendDone => {}
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let (pre, bak, col) = self.steps.viewmessages();
        let mut controls = Row::new();
        if bak {
            controls = controls.push(
                Button::new(&mut self.back_button, Text::new("back"))
                    .on_press(Message::Go(Arrow::Back)),
            );
        }
        controls = controls.push(Space::with_width(iced::Length::Fill));
        if pre {
            //let mut state = button::State::default();
            controls = controls.push(
                Button::new(&mut self.font_button, Text::new("Go"))
                    .on_press(Message::Go(Arrow::Front)),
            );
        }

        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .push(
                Button::new(&mut self.jump_button, Text::new("Jump"))
                    .on_press(Message::JumpIncreasePressed(self.value)),
            )
            .push(
                Button::new(&mut self.log_button, Text::new("Log"))
                    .on_press(Message::Docommand(Events::Log)),
            )
            .push(
                Button::new(&mut self.say_button, Text::new("Say"))
                    .on_press(Message::Docommand(Events::Say)),
            )
            .push(col)
            .push(controls)
            .width(iced::Length::Fill)
            .into()
    }
}
async fn update(input: i32) -> i32 {
    tokio::time::sleep(tokio::time::Duration::from_secs(input as u64)).await;
    (input + 1) * 2
}
