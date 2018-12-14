use std::cell::{RefCell};

use crate::*;

pub struct Label
{
	widget: WidgetBase,
	text: RefCell<String>,
}

impl Widget for Label
{
	fn as_widget(&self) -> &WidgetBase
	{
		&self.widget
	}
	fn as_widget_mut(&mut self) -> &mut WidgetBase
	{
		&mut self.widget
	}

	fn setup(&mut self)
	{

	}

	fn draw(&self, draw: &mut cairo::Cairo)
	{
		draw.set_font_size(20.0);
		let height = self.height();
		let width = self.width();

		draw.set_color(Color::black());
		draw.move_to(0.0, (height-5) as f64);
		draw.show_text(&self.text.borrow());
	}
}


impl Label
{
	pub fn new(text: &str) -> Label
	{
		let mut w = Label
		{
			widget: WidgetBase::named("Label"),
			text: RefCell::new(text.to_string()),
		};

		w.widget.set_maximum_size(Size{ width:u32::max_value(), height:22 });
		w
	}

	pub fn set_text(&self, text : String)
	{
		self.text.replace(text);
		self.repaint();
	}
}
