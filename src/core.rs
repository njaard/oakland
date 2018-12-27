use std;

use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::borrow::Borrow;
use std::sync::Arc;

use crate::draw::Color;
use crate::dimension::*;
use crate::queue;
use libc::{c_uchar, c_int};

use image::GenericImage;
use image::GenericImageView;

pub enum Policy
{
	Minimum,
	Maximum,
	Expanding,
	Fixed,
}

pub struct SizePolicy
{
	pub vertical : Policy,
	pub horizontal : Policy,
}


extern
{
	fn cairo_xcb_surface_create(
		conn : *mut xcb::ffi::xcb_connection_t,
		drawable : u32,
		visual : *const xcb::ffi::xcb_visualtype_t,
		width : std::os::raw::c_int,
		height : std::os::raw::c_int,
	) -> *mut libc::c_void;

	fn cairo_xcb_surface_create_with_xrender_format(
		conn: *mut xcb::ffi::xcb_connection_t,
		screen: *mut xcb::ffi::xcb_screen_t,
		drawable : u32,
		format: *const xcb::ffi::render::xcb_render_pictforminfo_t,
		width : std::os::raw::c_int,
		height : std::os::raw::c_int,
	) -> *mut libc::c_void;

	fn cairo_image_surface_create_for_data(
		data: *mut c_uchar,
		format: c_int,
		width: c_int,
		height: c_int,
		stride: c_int
	) -> *mut libc::c_void;

	fn cairo_xcb_surface_set_size(
		surface: *mut libc::c_void,
		width: c_int,
		height: c_int,
	);
}

fn surface_from_x(
	conn : *mut xcb::ffi::xcb_connection_t,
	screen : *mut xcb::ffi::xcb_screen_t,
	format: *const xcb::ffi::render::xcb_render_pictforminfo_t,
	drawable : u32,
	width : i32,
	height : i32
) -> cairo::surface::Surface
{
/*
	let opaque = unsafe { cairo_xcb_surface_create(
		conn, drawable, visual, width, height
	) };
	
	let mut s = cairo::surface::Surface { opaque : opaque };
	match s.status()
	{
		cairo::Status::Success => {},
		s => panic!("error creating cairo surface {:?}", s),
	}
	s
	*/


	let opaque = unsafe { cairo_xcb_surface_create_with_xrender_format(
		conn,
		screen,
		drawable,
		format,
		width, height
	) };

	let mut s = cairo::surface::Surface { opaque : opaque };
	match s.status()
	{
		cairo::Status::Success => {},
		s => panic!("error creating cairo surface {:?}", s),
	}
	s
}

unsafe fn surface_from_img(im: &image::DynamicImage)
	-> cairo::surface::Surface
{
	let w = im.width();
	let h = im.height();
	let mut v = im.to_rgba().into_raw();

	let mut v = v.as_mut_ptr();

	let opaque;
	{
		let pixels = v as *mut u32;
		let pixels = std::slice::from_raw_parts_mut(pixels, (w*h) as usize);
		for pixel in pixels
		{
			let alpha = *pixel >> 24;

			let m =
				|color: u32| -> u32
				{
					let a = alpha*color + 0x80;
					((a >> 8) + a) >> 8
				};

			let a = *pixel;
			//*pixel = ((a & 0xff000000) >> 24) | (a << 8);
			*pixel = (a & 0xff000000)
				| m((a & 0xff0000) >> 16)
				| m(a & 0x0000ff) << 16
				| m((a & 0xff00) >> 8) << 8;
			//*pixel = 0x33ff00ff;
		}

		opaque = cairo_image_surface_create_for_data(
			v,
			0, // ARGB32
			w as i32,
			h as i32,
			4*w as i32
		);
	}
	let mut s = cairo::surface::Surface { opaque : opaque };
	match s.status()
	{
		cairo::Status::Success => {},
		s => panic!("error creating cairo surface {:?}", s),
	}

	s
}

fn pict_formats(c: &xcb::base::Connection)
	-> (xcb::ffi::render::xcb_render_pictforminfo_t,
		xcb::ffi::render::xcb_render_pictforminfo_t)
{
	type PF = *const xcb::ffi::render::xcb_render_pictforminfo_t;
	let mut fmt_rgb = None;
	let mut fmt_rgba = None;

	let formats = xcb::render::query_pict_formats(c).get_reply().unwrap();
	for f in formats.formats()
	{
		if f.type_() == xcb::ffi::render::XCB_RENDER_PICT_TYPE_DIRECT as u8
		{
			if f.direct().red_mask() != 0xff &&
				f.direct().red_shift() != 16
				{ continue; }
			if f.depth() == 32
			{
				if f.direct().alpha_mask() == 0xff &&
					f.direct().alpha_shift() == 24
				{
					eprintln!("PICT {} matches", f.id());
					fmt_rgba = Some(f.base);
				}
			}
			if f.depth() == 24
			{
				if fmt_rgb.is_some() { continue; }
				fmt_rgb = Some(f.base);
			}
		}
	}

	(fmt_rgba.unwrap(), fmt_rgb.unwrap())
}


pub struct GraphicalDetails
{
	pub(crate) connection : xcb::base::Connection,
	pub(crate) screen_num : i32,
	top_level_widgets: std::vec::Vec<Rc<Widget>>,
	repaint_everything : Cell<bool>,
	event_post: Arc<queue::EventPoster>,
	pict_formats: (
		xcb::ffi::render::xcb_render_pictforminfo_t,
		xcb::ffi::render::xcb_render_pictforminfo_t
	),
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
		let visual = self.get_visual();
		let screen = self.screen();

		let colormap_id = self.connection.generate_id();
		xcb::create_colormap_checked(
			&self.connection,
			xcb::ffi::XCB_COLORMAP_ALLOC_NONE as u8,
			colormap_id, screen.root(), visual
		).request_check().map_err(|g| panic!("e = {}", g.error_code())).unwrap();

		let win = self.connection.generate_id();
		xcb::create_window_checked(
			&self.connection,
			32,
			win,
			screen.root(),
			0, 0,
			150, 150,
			1,
			xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
			visual,
			&[
				(xcb::CW_COLORMAP, colormap_id),
				(xcb::CW_BACK_PIXEL, 0xffc2bbb8),
				(xcb::CW_BORDER_PIXEL, 0),
				(
					xcb::CW_EVENT_MASK,
					xcb::EVENT_MASK_EXPOSURE
						| xcb::EVENT_MASK_KEY_PRESS
						| xcb::EVENT_MASK_BUTTON_PRESS
						| xcb::EVENT_MASK_BUTTON_RELEASE
						| xcb::EVENT_MASK_STRUCTURE_NOTIFY
				)
			]
		).request_check().map_err(|g| panic!("e = {}", g.error_code())).unwrap();
		xcb::map_window(&self.connection, win);

		&self.connection.flush();
		win
	}

	fn get_visual(&self) -> u32
	{
		let screen = self.screen();
		for depth in screen.allowed_depths()
		{
			if depth.depth() == 32
			{
				for visual in depth.visuals()
				{
					return visual.visual_id();
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

			while let Some(event) = self.connection.poll_for_event()
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
							w.mouse_event(MouseEvent::LeftPress, &pos);
						}
					},
					xcb::BUTTON_RELEASE =>
					{
						let button_press : &xcb::ButtonPressEvent
							= unsafe { xcb::cast_event(&event) };

						let pos = Point { x: button_press.event_x() as i32, y: button_press.event_y() as i32 };

						for w in &self.top_level_widgets
						{
							w.mouse_event(MouseEvent::LeftRelease, &pos);
						}
					},
					xcb::CONFIGURE_NOTIFY =>
					{
						let resize_req : &xcb::ConfigureNotifyEvent
							= unsafe { xcb::cast_event(&event) };
						let sz = Size
						{
							width: resize_req.width() as u32,
							height: resize_req.height() as u32,
						};
						for w in &self.top_level_widgets
						{
							w.as_widget().set_size(sz);
							w.resized(sz);
						}
					},
					xcb::EXPOSE =>
					{
						self.paint_everything();
					},
					_ => {}
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
		for w in &self.top_level_widgets
		{
			let w = w.as_ref().borrow();
			let wrect = w.rectangle();
			let mut surface = surface_from_x(
				self.connection.get_raw_conn(),
				self.screen().ptr,
				&self.pict_formats.0,
				w.as_widget().true_window_id(),
				wrect.width() as i32,
				wrect.height() as i32,
			);
			/*unsafe
			{
				cairo_xcb_surface_set_size(
					surface.opaque,
					wrect.width() as c_int,
					wrect.height() as c_int,
				);
			}*/

			{
				use crate::draw::DrawPixel;
				let mut cr = cairo::Cairo::create(&mut surface);
				cr.fillcolor(crate::draw::Color::rgb(0xc2,0xbb, 0xb8));
				w.draw(&mut cr);
			}
			//surface.flush();
			surface.finish();
		}
		
		self.connection.flush();
	}

	pub fn pixmap(&self, from: &image::DynamicImage)
		-> cairo::surface::Surface
	{
		let mut dest_surface;
		unsafe
		{
			let dest_id = xcb::ffi::base::xcb_generate_id(self.connection.get_raw_conn());

			xcb::xproto::create_pixmap_checked(
				&self.connection,
				32,
				dest_id,
				self.screen().root(),
				from.width() as u16,
				from.height() as u16,
			).request_check().map_err(|g| panic!("e = {}", g.error_code())).unwrap();
			dest_surface = surface_from_x(
				self.connection.get_raw_conn(),
				self.screen().ptr,
				&self.pict_formats.0,
				dest_id,
				from.width() as i32,
				from.height() as i32,
			);

			let mut source = surface_from_img(from);
			use crate::draw::DrawPixel;
			let mut cr = cairo::Cairo::create(&mut dest_surface);
			cr.set_operator(cairo::operator::Operator::Source);
			cr.set_source_surface(&mut source, 0.0, 0.0);
			cr.paint();
		}
		dest_surface
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

	fn det(&self) -> Option<Rc<RefCell<GraphicalDetails>>>
	{
		self.as_widget()
			.det
			.borrow()
			.clone()
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

	fn setup(&self, det: Rc<RefCell<GraphicalDetails>>)
	{
		*self.as_widget().det.borrow_mut() = Some(det.clone());
		self.setup_children(det);
	}

	fn setup_children(&self, _det: Rc<RefCell<GraphicalDetails>>)
	{
	}

	fn draw(&self, _ : &mut cairo::Cairo)
	{
		eprintln!("blank painted {}", self.name());
	}
	fn rectangle(&self) -> Rectangle { self.as_widget().rectangle() }
	fn width(&self) -> u32 { self.as_widget().rectangle().width() }
	fn height(&self) -> u32 { self.as_widget().rectangle().height() }
	fn mouse_event(&self, e: MouseEvent, pt: &Point)
	{
		if let Some(c) = self.child_at(pt)
		{
			c.mouse_event(e, &pt);
		}
	}
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
		self.resized(*sz);
	}
	fn set_geometry(&self, rect : Rectangle)
	{
		self.as_widget().rectangle.set(rect);
		self.resized(rect.size);
	}
	fn child_at(&self, _pt: &Point) -> Option<Rc<Widget>>
	{
		None
	}

	fn pt_from_parent(&self, mut pt: Point) -> Point
	{
		pt.x -= self.rectangle().x();
		pt.y -= self.rectangle().y();
		pt
	}
	fn rect_from_parent(&self, mut r: Rectangle) -> Rectangle
	{
		r.pos.x -= self.rectangle().x();
		r.pos.y -= self.rectangle().y();
		r
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
		let pict_formats = pict_formats(&conn.0);
		let g = GraphicalDetails
		{
			connection : conn.0,
			screen_num : conn.1,
			top_level_widgets : vec!(),
			repaint_everything : Cell::new(false),
			event_post,
			pict_formats,
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
	
	pub fn put<'a, W>(&'a self, widget: W)
		-> Rc<W>
	where W: Widget + 'static
	{
		let mut max_size = widget.maximum_size();
		if max_size.width > (i32::max_value() as u32)
			{ max_size.width = i32::max_value() as u32; }
		if max_size.height > (i32::max_value() as u32)
			{ max_size.height = i32::max_value() as u32; }

		widget.setup(self.det.clone());

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
	pub fn pixmap(&self, from: &image::DynamicImage)
		-> cairo::surface::Surface
	{
		let a : &RefCell<GraphicalDetails> = self.det.borrow();
		a.borrow().pixmap(from)
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
	pub(crate) true_window_id: Cell<u32>,
	pub(crate) det: RefCell<Option<Rc<RefCell<GraphicalDetails>>>>,
	pub(crate) rectangle : Cell<Rectangle>,
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
			true_window_id: Cell::new(0),
			det : RefCell::new(None),
			rectangle: Cell::new(Rectangle::coords(0,0, 100, 100)),
			name : name.to_string(),
			maximum_size : Size{ width : u32::max_value(), height : u32::max_value() },
		}
	}
	pub fn new() -> WidgetBase
	{
		Self::named("")
	}

	pub fn true_window_id(&self) -> u32
	{
		self.true_window_id.get()
	}
	
	fn rectangle(&self) -> Rectangle
	{
		self.rectangle.get()
	}
	fn set_size(&self, sz: Size)
	{
		let mut r = self.rectangle.get();
		r.resize(&sz);
		self.rectangle.set(r);
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
