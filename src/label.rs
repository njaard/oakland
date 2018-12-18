use std::cell::{Cell,RefCell};

use crate::*;

pub struct Label
{
	widget: WidgetBase,
	text: RefCell<String>,
	image: RefCell<Option<cairo::surface::Surface>>,
	font_size: Cell<f64>,
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
	fn draw(&self, draw: &mut cairo::Cairo)
	{
		if let Some(image) = self.image.borrow().as_ref()
		{
			draw.paste(image);
		}
		else
		{
			draw.set_font_size(self.font_size.get());
			let height = self.height();
			//let width = self.width();

			draw.set_color(Color::black());
			draw.move_to(0.0, (height-5) as f64);
			draw.show_text(&self.text.borrow());
		}
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
			image: RefCell::new(None),
			font_size: Cell::new(20.0),
		};

		w.widget.set_maximum_size(Size{ width:u32::max_value(), height:22 });
		w
	}

	pub fn set_text(&self, text : String)
	{
		self.text.replace(text);
		*self.image.borrow_mut() = None;
		self.repaint();
	}

	pub fn set_image(&self, image: cairo::surface::Surface)
	{
		*self.image.borrow_mut() = Some(image);
		self.text.replace(String::new());
		self.repaint();
	}

	pub fn set_font_size(&self, points: f64)
	{
		self.font_size.set(points);
		self.repaint();
	}
}
