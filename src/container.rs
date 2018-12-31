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

		let mut child = None;

		for w in self.children.borrow().iter()
		{
			let w2: Rc<Widget> = w.clone();
			if Rc::ptr_eq(&w2, &b)
			{
				child = Some(w2);
				break;
			}
		}
		if let Some(w) = child
		{
			w.mouse_event(e, &w.pt_from_parent(*pos));
		}
	}
}

pub trait MaybeRc
{
	type W;
	fn convert(self) -> Rc<Self::W>;
}

impl<T: Widget> MaybeRc for Rc<T>
{
	type W = T;
	fn convert(self) -> Rc<T>
	{
		self
	}
}

impl<T: Widget> MaybeRc for T
{
	type W = T;
	fn convert(self) -> Rc<T>
	{
		Rc::new(self)
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

	pub fn put<'a, M>(&'a self, widget: M)
		-> Rc<M::W>
	where M: MaybeRc,
		M::W: Widget + 'static
	{
		let b = widget.convert();
		if let Some(c) = self.det()
		{
			b.setup(c);
		}

		self.children.borrow_mut().push( b.clone() );
		b
	}

	pub fn remove<U: Widget>(&self, widget: &Rc<U>)
	{
		let widget: Rc<Widget> = widget.clone();
		use std::ops::Deref;
		use std::mem::transmute;
		let mut remove_at = None;
		let (a,_) = unsafe { transmute::<_, (usize, usize)>(widget.deref()) };
		for (idx,w) in self.children.borrow().iter().enumerate()
		{
			let (b,_) = unsafe { transmute::<_, (usize, usize)>(w.deref()) };
			if a == b
			{
				remove_at = Some(idx);
				break;
			}
		}

		eprintln!("********* REMOVEDing");
		if let Some(idx) = remove_at
		{
			eprintln!("********* REMOVED AT {}", idx);
			self.children.borrow_mut().remove(idx);
			self.repaint();
		}
	}
}

