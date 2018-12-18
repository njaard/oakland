use crate::*;
use std::cell::{RefCell};
use std::rc::Rc;

pub struct Container
{
	widget: WidgetBase,
	children: RefCell<std::vec::Vec<(Rc<Widget>)>>,
}

impl Widget for Container
{
	fn as_widget(&self) -> &WidgetBase
	{
		&self.widget
	}
	fn as_widget_mut(&mut self) -> &mut WidgetBase
	{
		&mut self.widget
	}

	fn setup_children(&self, det: Rc<RefCell<GraphicalDetails>>)
	{
		for b in self.children.borrow_mut().iter_mut()
		{
			b.setup(det.clone());
		}
	}

	fn draw(&self, c: &mut cairo::Cairo)
	{
		use std::borrow::Borrow;

		for w in self.children.borrow().iter()
		{
			let wrect = w.rectangle();
			c.save();
			c.translate(wrect.x() as f64, wrect.y() as f64);
			c.rectangle(0.0, 0.0, wrect.width() as f64, wrect.height() as f64);
			c.clip();

			let w = w.as_ref().borrow();
			w.draw(c);
			c.restore();
		}
	}

	fn child_at(&self, pt: &Point) -> Option<Rc<Widget>>
	{
		for w in self.children.borrow().iter()
		{
			if w.rectangle().contains(pt)
			{
				return Some(w.clone());
			}
		}
		None
	}
	fn resized(&self, _sz: Size)
	{
	}
	fn mouse_event(&self, e: MouseEvent, pos: &Point)
	{
		let b = self.child_at(pos);
		if b.is_none() { return; }
		let b = b.unwrap();

		for w in self.children.borrow().iter()
		{
			let w2: Rc<Widget> = w.clone();
			if Rc::ptr_eq(&w2, &b)
			{
				w.mouse_event(e, pos);
				break;
			}
		}
	}
}

impl Container
{
	pub fn new() -> Container
	{
		let w = Container
		{
			widget: WidgetBase::named("Container"),
			children: RefCell::new( vec!() ),
		};
		w
	}

	pub fn put<'a, W>(&'a self, widget: W)
		-> Rc<W>
	where W: Widget + 'static
	{
		if let Some(c) = self.det()
		{
			widget.setup(c);
		}

		if let Some(c) = self.det()
			{ widget.setup(c); }
		let b = Rc::new(widget);

		self.children.borrow_mut().push( b.clone() );
		b
	}
}

