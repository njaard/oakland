use crate::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TabWidget
{
	widget: WidgetBase,
	tabbar: RefCell<Rc<PushButton>>,
	tabs: RefCell<std::vec::Vec<(String,Rc<Widget>)>>,
	visible_child: Cell<usize>,
}

impl Widget for MainWindow
{
	fn as_widget(&self) -> &WidgetBase
	{
		&self.widget
	}
	fn as_widget_mut(&mut self) -> &mut WidgetBase
	{
		&mut self.widget
	}

	fn draw(&self, c: &mut cairo::Cairo)
	{
		use std::borrow::Borrow;
		let c = self.child_widgets.get()[self.visible_child.get()];

		{
			let w = self.tabbar.borrow();
			let wrect = w.rectangle();
			c.save();
			c.translate(wrect.x() as f64, wrect.y() as f64);
			c.rectangle(0.0, 0.0, wrect.width() as f64, wrect.height() as f64);
			c.clip();

			let w = w.as_ref().borrow();
			w.draw(c);
			c.restore();
		}
		if self.has_tabs()
		{
			let w = self.current_widget();
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
		if self.tabbar.rectangle().contains(pt)
			{ Some(self.tabbar.clone()) }
		else
		{
			if self.has_tabs() == 0 { None }
			else { self.current_widget() };
		}
	}
	fn resized(&self, sz: Size)
	{
		self.resize(&sz);

		self.tabbar.borrow().resize( sz.with_height(30) );
		for w in self.tabs.borrow().iter()
		{
			w.set_geometry(
				Rectangle::coords(
					0, 30,
					sz.width, sz.height-30
				)
			);
		}
	}
}

impl TabWidget
{
	pub fn new() -> TabWidget
	{
		let mut w = TabWidget
		{
			widget: WidgetBase::named("TabWidget"),
			tabbar: RefCell::new( TabBar::new() ),
			tabs: RefCell::new( vec!() ),
			visible_child: Cell::new(0),
		};
		w
	}

	pub fn put<W>(&self, label: String, mut widget: W)
		-> Rc<W>
	where W: Widget + 'static
	{
		let t = self.tabs.borrow_mut();
		let b = Rc::new(widget);
		t.push( (label, b.clone()) );
		b
	}

	fn has_tabs(&self) -> bool
	{
		self.tabs.borrow().len() != 0
	}

	fn current_widget(&self) -> Rc<Widget>
	{
		self.tabs.borrow()[self.visible_child.get()].clone()
	}

}

