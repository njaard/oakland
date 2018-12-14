use std;

use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::borrow::Borrow;
use std::sync::Arc;

use crate::draw::Color;
use crate::dimension::*;
use crate::queue;


enum Policy
{
	Minimum,
	Maximum,
	Expanding,
	Fixed,
}

pub struct SizePolicy
{
	vertical : Policy,
	horizontal : Policy,
}


extern
{
	fn cairo_xcb_surface_create(
		conn : *mut libc::c_void,
		drawable : u32,
		visual : *const xcb::ffi::xcb_visualtype_t,
		width : std::os::raw::c_int,
		height : std::os::raw::c_int,
	) -> *mut libc::c_void;
}

fn surface_from_x(
	conn : *mut libc::c_void,
	drawable : u32,
	visual : &xcb::ffi::xcb_visualtype_t,
	width : i32,
	height : i32
) -> cairo::surface::Surface
{
	let opaque = unsafe { cairo_xcb_surface_create(
		conn, drawable, visual, width, height
	) };
	
	cairo::surface::Surface { opaque : opaque }
}


pub struct GraphicalDetails
{
	pub(crate) connection : xcb::base::Connection,
	pub(crate) screen_num : i32,
	top_level_widgets: std::vec::Vec<Rc<Widget>>,
	repaint_everything : Cell<bool>,
	event_post: Arc<queue::EventPoster>,
}

impl GraphicalDetails
{
	pub(crate) fn screen<'a>(&'a self) -> xcb::StructPtr<'a, xcb::ffi::xcb_screen_t>
	{
		self
			.connection
			.get_setup()
			.roots()
			.nth(self.screen_num as usize)
			.unwrap()
	}
	
	pub(crate) fn make_real_window(&self) -> u32
	{
		let screen = self.screen();
		let foreground = self.connection.generate_id();
		xcb::create_gc(
			&self.connection, foreground, screen.root(),
			&[
				(xcb::GC_FOREGROUND, screen.black_pixel()),
				(xcb::GC_GRAPHICS_EXPOSURES, 1),
			]
		);

		let win = self.connection.generate_id();
		xcb::create_window(
			&self.connection,
			xcb::COPY_FROM_PARENT as u8,
			win,
			screen.root(),
			0, 0,
			150, 150,
			10,
			xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
			screen.root_visual(),
			&[
				(xcb::CW_BACK_PIXEL, Color::rgb(0xc2, 0xbb, 0xb8).xcb_color()),
				(
					xcb::CW_EVENT_MASK,
					xcb::EVENT_MASK_EXPOSURE
						| xcb::EVENT_MASK_KEY_PRESS
						| xcb::EVENT_MASK_BUTTON_PRESS
						| xcb::EVENT_MASK_BUTTON_RELEASE
						| xcb::EVENT_MASK_STRUCTURE_NOTIFY
				)
			]
		);
		xcb::map_window(&self.connection, win);

		&self.connection.flush();
		win
	}

	fn get_visual(&self) -> xcb::Visualtype
	{
		let screen = self.screen();
		for depth in screen.allowed_depths()
		{
			for visual in depth.visuals()
			{
				if screen.root_visual() == visual.visual_id()
				{
					return visual;
				}
			}
		}
		panic!("no visual found");
	}
	
	pub fn exec(&self)
	{
		//let screen = self.conn.screen();

		loop
		{
			if self.event_post.wait()
			{
				self.event_post.process_channels();
			}

			let event = self.connection.poll_for_event();
			match event
			{
				None => { () }
				Some(event) =>
				{
					let r = event.response_type() & !0x80;
					match r
					{
						xcb::KEY_PRESS =>
						{
							let key_press : &xcb::KeyPressEvent = unsafe { xcb::cast_event(&event) };
							println!("Key '{}' pressed", key_press.detail());
						},
						xcb::BUTTON_PRESS =>
						{
							let button_press : &xcb::ButtonPressEvent
								= unsafe { xcb::cast_event(&event) };

							let pos = Point { x: button_press.event_x() as i32, y: button_press.event_y() as i32 };

							for w in &self.top_level_widgets
							{
								let w = w.descendant_at(&pos);
								eprintln!("BUTTON_PRESS {:?} {:?}", w.name(), pos);
								(*w).mouse_event(MouseEvent::LeftPress);
							}
						},
						xcb::BUTTON_RELEASE =>
						{
							println!("button release");
							let button_press : &xcb::ButtonPressEvent
								= unsafe { xcb::cast_event(&event) };

							let pos = Point { x: button_press.event_x() as i32, y: button_press.event_y() as i32 };

							for w in &self.top_level_widgets
							{
								let w = w.descendant_at(&pos);
								(*w).mouse_event(MouseEvent::LeftRelease);
							}
						},
						xcb::CONFIGURE_NOTIFY =>
						{
							println!("resize");
							let resize_req : &xcb::ConfigureNotifyEvent
								= unsafe { xcb::cast_event(&event) };
							for w in &self.top_level_widgets
							{
								w
									.resized(
										Size
										{ width: resize_req.width() as u32, height: resize_req.height() as u32}
									);
							}
						},
						xcb::EXPOSE =>
						{
							self.paint_everything();
						},
						_ => {}
					}
				}
			}
			
			if self.repaint_everything.get()
			{
				self.paint_everything();
			}
		}
	}
	
	fn paint_everything(&self)
	{
		self.repaint_everything.set(false);
		let visual = self.get_visual();
		for w in &self.top_level_widgets
		{
			let w = w.as_ref().borrow();
			let wrect = w.rectangle();
			eprintln!("drawing surface {:?}", wrect);
			let mut surface = surface_from_x(
				self.connection.get_raw_conn() as *mut libc::c_void,
				w.as_widget().true_window_id(),
				&visual.base,
				wrect.width() as i32,
				wrect.height() as i32,
			);
			
			{
				use crate::draw::DrawPixel;
				let mut cr = cairo::Cairo::create(&mut surface);
				cr.fillcolor(Color::rgb(0xc2, 0xbb, 0xb8));
				w.draw(&mut cr);
			}
			surface.flush();
			surface.finish();
		}
		
		self.connection.flush();
	}

	fn repaint_everything(&self)
	{
		println!("marking a repaint as necessary");
		self.repaint_everything.set(true);
	}

	fn channel<T: 'static+Clone+Sized+Send, F: 'static+FnMut(T)>(
		&self,
		f: F,
	) -> queue::ChannelWrite<T>
	{
		self.event_post.channel(
			f,
		)
	}
}

pub trait Widget
{
	fn as_widget(&self) -> &WidgetBase;
	fn as_widget_mut(&mut self) -> &mut WidgetBase;

	fn det(&self) -> Rc<RefCell<GraphicalDetails>>
	{
		self.as_widget().det.borrow().as_ref().expect("connection").clone()
	}

	fn name(&self) -> &str { &self.as_widget().name }

	fn trace(&self, depth : usize, o : &mut std::io::Write)
	{
		self.traceme(depth, o);
	}
	fn traceme(&self, depth : usize, o : &mut std::io::Write)
	{
		writeln!(
			o,
			"{}{} {:?}",
			" ".repeat(depth),
			self.as_widget().name,
			self.rectangle(),
		).unwrap();
	}

	fn setup(&mut self);

	fn draw(&self, _ : &mut cairo::Cairo)
	{
		eprintln!("blank painted {}", self.name());
	}
	fn rectangle(&self) -> Rectangle { self.as_widget().rectangle() }
	fn width(&self) -> u32 { self.as_widget().rectangle().width() }
	fn height(&self) -> u32 { self.as_widget().rectangle().height() }
	fn mouse_event(&self, _ : MouseEvent) { }
	fn resized(&self, _ : Size) { }
	fn repaint(&self)
	{
		self.as_widget().repaint();
	}

	fn minimum_size(&self) -> Size
	{
		//self.as_widget().minimum_size()
		Size{ width:0, height:0 }
	}
	fn maximum_size(&self) -> Size
	{
		self.as_widget().maximum_size()
	}
	fn size_hint(&self) -> Size
	{
		Size{ width: 100, height: 100 }
	}
	
	fn size_policy(&self) -> SizePolicy
	{
		SizePolicy{ vertical: Policy::Expanding, horizontal: Policy::Expanding }
	}
	fn is_visible(&self) -> bool
	{
		true
	}

	fn set_position(&self, pos: &Point)
	{
		let mut rect = self.as_widget().rectangle.get();
		rect.set_position(pos);
		self.as_widget().rectangle.set(rect);
	}

	fn resize(&self, sz: &Size)
	{
		let mut rect = self.as_widget().rectangle.get();
		rect.resize(sz);
		self.as_widget().rectangle.set(rect);
	}
	fn set_geometry(&self, rect : Rectangle)
	{
		self.as_widget().rectangle.set(rect);
	}
	fn child_at(&self, pt: &Point) -> Option<Rc<Widget>>
	{
		None
	}
}

impl std::fmt::Debug for Widget
{
	fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(f, "Widget{{'{}'}}", self.as_widget().name)
	}
}

pub struct Graphical
{
	det : Rc<RefCell<GraphicalDetails>>,
	widget : WidgetBase,
}


impl Graphical
{
	pub fn new() -> Graphical
	{
		let conn = xcb::Connection::connect(None).unwrap();
		let event_post = Arc::new(queue::EventPoster::new(&conn.0));
		let g = GraphicalDetails
		{
			connection : conn.0,
			screen_num : conn.1,
			top_level_widgets : vec!(),
			repaint_everything : Cell::new(false),
			event_post,
		};
		
		Graphical
		{
			det : Rc::new(RefCell::new(g)),
			widget : WidgetBase::named("root"),
		}
	}

	pub fn channel<T: 'static+Clone+Sized+Send, F: 'static+FnMut(T)>(
		&self,
		f: F,
	) -> queue::ChannelWrite<T>
	{
		let a : &RefCell<GraphicalDetails> = self.det.borrow();
		a.borrow().channel(f)
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

		widget.as_widget_mut().det = Some(self.det.clone());
		widget.setup();

		let b = Rc::new(widget);

		self.det.borrow_mut().top_level_widgets.push( b.clone() );
		b
	}

	pub(crate) fn make_real_window(&self) -> u32
	{
		let a : &RefCell<GraphicalDetails> = self.det.borrow();
		a.borrow().make_real_window()
	}
	pub fn exec(&self)
	{
		let a : &RefCell<GraphicalDetails> = self.det.borrow();
		a.borrow().exec();
	}
}


#[derive(PartialEq, Eq)]
pub enum MouseEvent
{
	LeftPress,
	LeftRelease,
	RightPress,
	RightRelease
}


pub struct WidgetBase
{
	pub(crate) true_window_id : u32,
	pub(crate) det : Option<Rc<RefCell<GraphicalDetails>>>,
	rectangle : Cell<Rectangle>,
	name : String,
	maximum_size : Size,
}


impl WidgetBase
{
	pub fn named(name : &str) -> WidgetBase
	{
		//let display = display.as_ref();
		WidgetBase
		{
			true_window_id : 0,
			det : None,
			rectangle: Cell::new(Rectangle::coords(0,0, 100, 100)),
			name : name.to_string(),
			maximum_size : Size{ width : u32::max_value(), height : u32::max_value() },
		}
	}
	pub fn new() -> WidgetBase
	{
		Self::named("")
	}

	fn true_window_id(&self) -> u32
	{
		self.true_window_id
	}
	
	fn rectangle(&self) -> Rectangle
	{
		self.rectangle.get()
	}

	fn maximum_size(&self) -> Size
	{
		self.maximum_size
	}
	
	pub fn set_maximum_size(&mut self, size : Size)
	{
		self.maximum_size = size;
	}
	
	fn repaint(&self)
	{
		let d = self.det.borrow();
		if let Some(ref a) = *d
		{
			let a : &RefCell<GraphicalDetails> = a.borrow();
			a.borrow().repaint_everything();
		}
		else
		{
			panic!("no connection in {}", self.name);
		}
	}
}

trait Descending
{
	fn descendant_at(&self, pt: &Point) -> Rc<Widget>;
}

impl Descending for Rc<Widget>
{
	fn descendant_at(&self, pt: &Point) -> Rc<Widget>
	{
		eprintln!("getting children on {}", self.name());
		match (*self).child_at(pt)
		{
			Some(a) => a.descendant_at(pt),
			None => self.clone(),
		}
	}
}
