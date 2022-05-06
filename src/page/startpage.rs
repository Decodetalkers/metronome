use iced::Text;

use super::container;
use super::StepMessage;
use super::TourView;
use crate::Message;
pub struct Welcome {}
impl TourView for Welcome {
    fn update(&mut self, _msg: StepMessage) {}
    fn can_continue(&self) -> bool {
        true
    }
    fn can_back(&self) -> bool {
        false
    }
    fn view(&mut self) -> iced::Column<Message> {
        container("Welcome!").push(Text::new(
            "Welcome this \
                bate",
        ))
    }
}
