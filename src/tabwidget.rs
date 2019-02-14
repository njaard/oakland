use crate::*;
use std::cell::{RefCell};
use std::rc::Rc;

pub struct TabWidget
{
	widget: WidgetBase,
	tabbar: RefCell<Rc<TabBar>>,
	tabs: RefCell<std::vec::Vec<Rc<Widget>>>,
}

impl Widget for TabWidget
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
		self.tabbar.borrow().setup(det.clone());
		for b in self.tabs.borrow().iter()
		{
			b.setup(det.clone());
		}
	}

	fn draw(&self, c: &mut cairo::Cairo)
	{
		use std::borrow::Borrow;

		{
			let w = self.tabbar.borrow();
			let wrect = w.rectangle();
			c.save();
			c.translate(wrect.x() as f64, wrect.y() as f64);
			c.rectangle(0.0, 0.0, wrect.width() as f64, wrect.height() as f64);
			c.clip();

			let w = w.borrow();
			w.draw(c);
			c.restore();
		}
		if let Some(w) = self.current_widget()
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
		if self.tabbar.borrow().rectangle().contains(pt)
			{ Some(self.tabbar.borrow().clone()) }
		else
			{ self.current_widget() }
	}
	fn resized(&self, sz: Size)
	{
		self.tabbar.borrow().resize( &sz.with_height(45).with_width(1000) );
		for w in self.tabs.borrow().iter()
		{
			w.set_geometry(
				Rectangle::coords(
					0, 45+7,
					1000, 1000-45+7
				)
			);
		}
	}
	fn mouse_event(&self, e: MouseEvent, pt: &Point)
	{
		if self.tabbar.borrow().rectangle().contains(pt)
		{
			self.tabbar.borrow().mouse_event(e, &self.tabbar.borrow().pt_from_parent(*pt));
		}
		else if let Some(current) = self.current_widget()
		{
			current.mouse_event(e, &current.pt_from_parent(*pt));
		}
	}

}

impl TabWidget
{
	pub fn new() -> TabWidget
	{
		let w = TabWidget
		{
			widget: WidgetBase::named("TabWidget"),
			tabbar: RefCell::new( Rc::new(TabBar::new()) ),
			tabs: RefCell::new( vec!() ),
		};
		w
	}

	pub fn put<W>(&self, label: String, widget: W)
		-> Rc<W>
	where W: Widget + 'static
	{
		let mut t = self.tabs.borrow_mut();
		if let Some(c) = self.det()
			{ widget.setup(c); }
		let b = Rc::new(widget);
		t.push( b.clone() );
		self.tabbar.borrow().add(label);
		b
	}

	fn has_tabs(&self) -> bool
	{
		self.tabs.borrow().len() != 0
	}

	fn current_widget(&self) -> Option<Rc<Widget>>
	{
		if !self.has_tabs() { return None; }
		Some(self.tabs.borrow()[self.tabbar.borrow().current_button()].clone())
	}

}

