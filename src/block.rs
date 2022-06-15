use iced_native::layout::{self, Layout};
use iced_native::renderer;
use iced_native::{Color, Element, Length, Point, Rectangle, Size, Widget};
pub enum Kind {
    Dot,
    Squre,
}
pub struct Spring;
impl<Message, Renderer> Widget<Message, Renderer> for Spring
where
    Renderer: renderer::Renderer,
{
    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, _renderer: &Renderer, _limits: &layout::Limits) -> layout::Node {
        layout::Node::new(Size::new(100.0, 30.0))
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
            Color::TRANSPARENT,
        );
    }
}

impl<'a, Message, Renderer> Into<Element<'a, Message, Renderer>> for Spring
where
    Renderer: renderer::Renderer,
{
    fn into(self) -> Element<'a, Message, Renderer> {
        Element::new(self)
    }
}

pub struct Block {
    radius: f32,
    color: Color,
    kind: Kind,
}

impl Block {
    pub fn new(radius: f32, color: Color, kind: Kind) -> Self {
        Self {
            radius,
            color,
            kind,
        }
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
        match self.kind {
            Kind::Squre => {
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
            Kind::Dot => {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: layout.bounds(),
                        border_radius: self.radius,
                        border_width: 10.0,
                        border_color: Color::TRANSPARENT,
                    },
                    self.color,
                );
            }
        }
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
