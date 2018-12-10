use core::*;

use std::rc::Rc;
use std::cell::{RefCell, Cell};

struct LayoutBase
{

}

trait Layout
{
	fn layout(&mut self, size : Size);
	fn spacing(&self) -> u32;
	fn margin(&self) -> u32;
	
	fn invalidate(&mut self);
}


pub struct VBoxLayout
{
	widget : WidgetBase,
	elements : Vec<Rc<RefCell<Widget>>>,
}

impl VBoxLayout
{
	pub fn new() -> VBoxLayout
	{
		VBoxLayout
		{
			widget : WidgetBase::named("VBoxLayout"),
			elements : vec!(),
		}
	}
	
	pub fn put<W>(&mut self, widget : W)
		-> Rc<RefCell<W>>
		where W: Widget + 'static
	{
		let w : Rc<RefCell<W>> = Rc::new(RefCell::new(widget) as RefCell<W>);
		let q = w.clone() as Rc<RefCell<W>>;
		self.elements.push(q);
		w
	}
}

impl Layout for VBoxLayout
{
	fn layout(&mut self, size : Size)
	{
		println!("laying out");
		let mut remaining_space = size.height as i32;
		let mut pos = self.margin() as i32;
		remaining_space -= pos*2;
		
		let spacing = self.spacing() as i32;
		
		let left = self.margin() as i32;
		let width = (size.width - self.margin()*2) as i32;
		
		let num = self.elements.len() as i32;
		
		for e in &self.elements
		{
			let mut q = (e as &RefCell<Widget>).borrow_mut();
			println!("putting {} at {}", q.name(), pos);
			let h = (remaining_space-spacing*(num-1))/num;
			q.set_geometry( Rectangle::coords(
				left, pos,
				width as u32, h as u32
			) );
			remaining_space -= h + spacing;
			pos += h + spacing;
		}
	}
	
	fn spacing(&self) -> u32 { 7 }
	fn margin(&self) -> u32 { 11 }
	
	fn invalidate(&mut self)
	{
	
	}

}


impl Widget for VBoxLayout
{
	fn as_widget(&self) -> &WidgetBase
	{
		&self.widget
	}
	fn as_widget_mut(&mut self) -> &mut WidgetBase
	{
		&mut self.widget
	}

	fn resized(&mut self, size : Size)
	{
		self.layout(size);
	}
	fn child_at(&self, pt : &Point) -> Option<Rc<RefCell<Widget>>>
	{
		for e in &self.elements
		{
			if e.borrow_mut().rectangle().contains(pt)
				{ return Some( e.clone() ); }
		}
		None
	}

	fn setup(&mut self, conn : Rc<RefCell<GraphicalDetails>>, window_id : u32)
	{
		self.as_widget_mut().setup(conn.clone(), window_id);
		for e in &self.elements
		{
			e.borrow_mut().setup(conn.clone(), window_id);
		}
	}

	fn draw(&self, c : &mut cairo::Cairo)
	{
		for e in &self.elements
		{
			c.save();
			c.translate(self.rectangle().x() as f64, self.rectangle().y() as f64);
			e.borrow_mut().draw(c);
			c.restore();
		}
	}

	fn trace(&self, depth : usize, o : &mut ::std::io::Write)
	{
		self.traceme(depth, o);
		for e in &self.elements
		{
			e.borrow().trace(depth+1, o);
		}
	}
}

