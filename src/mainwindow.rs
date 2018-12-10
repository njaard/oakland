use crate::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MainWindow
{
	widget: WidgetBase,
	title: RefCell<String>,
	child_widgets: RefCell<std::vec::Vec<Rc<Widget>>>,
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

	fn setup(&mut self)
	{
		let det = self.widget.det.as_mut().expect("det");
		self.widget.true_window_id = det.borrow_mut().make_real_window();
		self.setup_title();

		/*
		let hints = xcb_util::icccm::SizeHints::empty()
			.max_size(max_size.width as i32, max_size.height as i32)
			.build();

		xcb_util::icccm::set_wm_normal_hints(
			&mut gd.borrow_mut().connection,
			new_real_window,
			&hints
		); */
	}
	fn draw(&self, c: &mut cairo::Cairo)
	{
		use std::borrow::Borrow;
		for w in self.child_widgets.borrow().iter()
		{
			let wrect = w.rectangle();
			eprintln!("drawing {} at {:?}", w.name(), wrect);
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
		println!("checking {:?} on {:?}{{{:?}}}", pt, self.name(), self.rectangle());
		for w in self.child_widgets.borrow().iter()
		{
			return Some(w.clone());
		}
		None
	}
	fn resized(&self, sz: Size)
	{
		self.resize(&sz);
	}
}

impl MainWindow
{
	pub fn new(title: &str) -> MainWindow
	{
		let mut w = MainWindow
		{
			widget: WidgetBase::named("MainWindow"),
			title: RefCell::new(title.to_string()),
			child_widgets: RefCell::new(vec!()),
		};
		w
	}

	pub fn set_title(&self, title: String)
	{
		self.title.replace(title);
		if self.widget.det.is_some()
		{
			self.setup_title();
		}
	}

	fn setup_title(&self)
	{
		xcb_util::icccm::set_wm_name(
			&self.widget.det.as_ref().expect("det").borrow().connection,
			self.widget.true_window_id,
			&*self.title.borrow(),
		);
	}

	pub fn put<'a, W>(&'a self, mut widget: W)
		-> Rc<W>
	where W: Widget + 'static
	{
		let mut max_size = widget.maximum_size();
		if max_size.width > (i32::max_value() as u32)
			{ max_size.width = i32::max_value() as u32; }
		if max_size.height > (i32::max_value() as u32)
			{ max_size.height = i32::max_value() as u32; }

		widget.as_widget_mut().det = Some(self.det().clone());
		widget.setup();

		let b = Rc::new(widget);

		self.child_widgets.borrow_mut().push( b.clone() );
		b
	}
}

