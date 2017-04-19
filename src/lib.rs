extern crate xcb;

use std::rc::Rc;

pub struct GraphicalDetails
{
	connection : xcb::base::Connection,
	screen_num : i32
}

impl GraphicalDetails
{
	fn screen(&self) -> xcb::StructPtr<xcb::ffi::xcb_screen_t>
	{
		self
			.connection
			.get_setup()
			.roots()
			.nth(self.screen_num as usize)
			.unwrap()
	}
}

pub struct Graphical
{
	conn : Rc<GraphicalDetails>
}


impl Graphical
{
	pub fn new() -> Graphical
	{
		let conn = xcb::Connection::connect(None).unwrap();
		let g = GraphicalDetails
		{
			connection : conn.0,
			screen_num : conn.1,
		};
		Graphical
		{
			conn : Rc::new(g)
		}
	}
	
	pub fn exec(&self)
	{
		loop
		{
			let event = self.conn.connection.wait_for_event();
			match event
			{
				None => { break; }
				Some(event) =>
				{
					let r = event.response_type() & !0x80;
					match r
					{
						xcb::KEY_PRESS =>
						{
							let key_press : &xcb::KeyPressEvent = xcb::cast_event(&event);
							println!("Key '{}' pressed", key_press.detail());
							break;
						},
						_ => {}
					}
				}
			}
		}
	}
}


pub trait WidgetParent
{
	fn conn(&self) -> Rc<GraphicalDetails>;
	fn true_window(&self) -> u32;
}

pub struct Widget
{
	true_window_id : u32,
	conn : Rc<GraphicalDetails>,
}

impl WidgetParent for Graphical
{
	fn conn(&self) -> Rc<GraphicalDetails>
	{
		self.conn.clone()
	}
	fn true_window(&self) -> u32
	{
		let conn = &self.conn;
		let screen = conn.screen();
		let foreground = conn.connection.generate_id();
		xcb::create_gc(
			&conn.connection, foreground, screen.root(),
			&[
				(xcb::GC_FOREGROUND, screen.black_pixel()),
				(xcb::GC_GRAPHICS_EXPOSURES, 0),
			]
		);
		
		let win = conn.connection.generate_id();
		xcb::create_window(
			&conn.connection,
			xcb::COPY_FROM_PARENT as u8,
			win,
			screen.root(),
			0, 0,
			150, 150,
			10,
			xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
			screen.root_visual(),
			&[
				(xcb::CW_BACK_PIXEL, screen.white_pixel()),
				(xcb::CW_EVENT_MASK,
				xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
			]
		);
		xcb::map_window(&conn.connection, win);
		
		&conn.connection.flush();
		win
	}
}

impl WidgetParent for Widget
{
	fn conn(&self) -> Rc<GraphicalDetails>
	{
		self.conn.clone()
	}
	fn true_window(&self) -> u32
	{
		panic!("no");
	}
}


impl Widget
{
	pub fn new(display : &WidgetParent) -> Widget
	{
		//let display = display.as_ref();
		Widget
		{
			true_window_id : display.true_window(),
			conn : display.conn()
		}
	}
}

