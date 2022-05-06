use super::Message;
use iced::{Column, Text};
mod finalpage;
mod secondpage;
mod startpage;
pub struct Steps {
    steps: Vec<Box<dyn TourView>>,
    current: usize,
}
impl Steps {
    pub fn new() -> Self {
        Steps {
            steps: vec![
                Box::new(startpage::Welcome {}),
                Box::new(secondpage::Input::default()),
                Box::new(finalpage::Silderview::default()),
            ],
            current: 0,
        }
    }
    pub fn viewmessages(&mut self) -> (bool, bool, Column<Message>) {
        (
            self.steps[self.current].can_continue(),
            self.steps[self.current].can_back(),
            self.steps[self.current].view(),
        )
    }
    pub fn update(&mut self, msg: StepMessage) {
        self.steps[self.current].update(msg);
    }
    pub fn next(&mut self) {
        if self.steps[self.current].can_continue() {
            self.current += 1;
        }
    }
    pub fn preview(&mut self) {
        if self.steps[self.current].can_back() {
            self.current -= 1;
        }
    }
}
#[derive(Debug, Clone)]
pub enum StepMessage {
    SliderChanged(u8),
    InputChanged(String),
}

fn container(title: &str) -> Column<Message> {
    Column::new().spacing(20).push(Text::new(title).size(50))
}
pub trait TourView {
    fn update(&mut self, msg: StepMessage);
    fn can_back(&self) -> bool;
    fn can_continue(&self) -> bool;
    fn view(&mut self) -> Column<Message>;
}
