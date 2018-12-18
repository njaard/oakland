use crate::*;
use std::cell::{Cell,RefCell};
use std::rc::Rc;

pub struct TabBar
{
	widget: WidgetBase,
	buttons: RefCell<std::vec::Vec<(Rc<PushButton>)>>,
	current_button: Cell<usize>,
}

impl Widget for TabBar
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
		for b in self.buttons.borrow_mut().iter_mut()
		{
			b.setup(det.clone());
		}
	}

	fn draw(&self, c: &mut cairo::Cairo)
	{
		use std::borrow::Borrow;

		for w in self.buttons.borrow().iter()
		{
			let wrect = w.rectangle();
			eprintln!("tabbar drawing {} at {:?}", w.name(), wrect);
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
		for w in self.buttons.borrow().iter()
		{
			if w.rectangle().contains(pt)
			{
				eprintln!("CHILD_AT {} has {:?}", self.name(), pt);
				return Some(w.clone());
			}
		}
		None
	}
	fn resized(&self, _sz: Size)
	{
		let mut x=0;
		for w in self.buttons.borrow().iter()
		{
			w.set_geometry(
				Rectangle::coords(x, 0, 100, 30)
			);
			x += 107;
		}

	}
	fn mouse_event(&self, e: MouseEvent, pos: &Point)
	{
		let c = self.current_button.get();
		if e == MouseEvent::LeftPress
		{
			self.buttons.borrow()[c].set_toggled(false);
		}

		let b = self.child_at(pos);
		if b.is_none() { return; }
		let b = b.unwrap();

		for (idx,w) in self.buttons.borrow().iter().enumerate()
		{
			let w2: Rc<Widget> = w.clone();
			if Rc::ptr_eq(&w2, &b)
			{
				eprintln!("clicked on {}", w.text());
				self.current_button.set(idx);
				if e == MouseEvent::LeftPress
				{
					w.set_toggled(true);
				}
				break;
			}
		}
		b.mouse_event(e, pos);

		self.repaint();
	}
}

impl TabBar
{
	pub fn new() -> TabBar
	{
		let w = TabBar
		{
			widget: WidgetBase::named("TabBar"),
			buttons: RefCell::new( vec!() ),
			current_button: Cell::new(0),
		};
		w
	}

	pub fn add(&self, label: String) -> usize
	{
		let button = PushButton::new(label);

		let mut t = self.buttons.borrow_mut();
		if let Some(c) = self.det()
		{
			button.setup(c);
		}
		t.push( Rc::new(button) );
		t.len()-1
	}

	pub fn current_button(&self) -> usize
	{
		self.current_button.get()
	}

}

