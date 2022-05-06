use super::{container, StepMessage, TourView};
use crate::Message;
use iced::{slider, Slider};
#[derive(Default)]
pub struct Silderview {
    value: u8,
    state: slider::State,
}
impl TourView for Silderview {
    fn update(&mut self, msg: StepMessage) {
        if let StepMessage::SliderChanged(size) = msg {
            self.value = size;
        }
    }
    fn can_back(&self) -> bool {
        true
    }
    fn can_continue(&self) -> bool {
        false
    }
    fn view(&mut self) -> iced::Column<Message> {
        container("Final").push(Slider::new(&mut self.state, 0..=100, self.value, |input| {
            Message::StepMessage(StepMessage::SliderChanged(input))
        }))
    }
}
