use super::{container, StepMessage, TourView};
use crate::Message;
use iced::{text_input, TextInput};
#[derive(Default)]
pub struct Input {
    value: String,
    state: text_input::State,
}
impl TourView for Input {
    fn update(&mut self, msg: StepMessage) {
        if let StepMessage::InputChanged(input) = msg {
            self.value = input;
        }
    }
    fn can_back(&self) -> bool {
        true
    }
    fn can_continue(&self) -> bool {
        !self.value.is_empty()
    }
    fn view(&mut self) -> iced::Column<Message> {
        container("Input Text").push(TextInput::new(
            &mut self.state,
            "Type to continut",
            &self.value,
            |input| Message::StepMessage(StepMessage::InputChanged(input)),
        ))
    }
}
