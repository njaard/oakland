use crate::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MainWindow
{
	widget: WidgetBase,
	title: RefCell<String>,
	child_widgets: RefCell<std::vec::Vec<Rc<Widget>>>,
}

fn intern_atom(conn: &xcb::Connection, atom_name: &str)
	-> xcb::Atom
{
	xcb::intern_atom(conn, false, atom_name).get_reply()
		.unwrap()
		.atom()
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
		for w in self.child_widgets.borrow().iter()
		{
			if w.rectangle().contains(pt)
			{
				eprintln!("CHILD_AT {} has {:?}", self.name(), pt);
				return Some(w.clone());
			}
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

	pub fn set_fullscreen(&self, fullscreen: bool)
	{
		unsafe
		{
			let NET_WM_STATE_ADD = 1;
			let NET_WM_STATE_REMOVE = 0;

			let atom_fullscreen = intern_atom(
				&self.widget.det.as_ref().expect("det").borrow().connection,
				"_NET_WM_STATE_FULLSCREEN"
			);
			let atom_state = intern_atom(
				&self.widget.det.as_ref().expect("det").borrow().connection,
				"_NET_WM_STATE"
			);

			let mut data = [0u32; 20/4];
			data[0] = if fullscreen { NET_WM_STATE_ADD } else { NET_WM_STATE_REMOVE };
			data[1] = atom_fullscreen;
			data[3] = 1;

			let d = xcb::ffi::xproto::xcb_client_message_data_t
				{ data: [0u8; 20] };

			let mut ev = xcb::ffi::xproto::xcb_client_message_event_t
			{
				response_type: xcb::xproto::CLIENT_MESSAGE,
				format: 32,
				window: self.widget.true_window_id,
				type_: atom_state,
				data: d,
				sequence: 0,
			};
			std::ptr::copy(
				&data as *const u32 as *const u8,
				(&mut ev.data.data).as_ptr() as *mut u8, 20
			);

			xcb::ffi::xproto::xcb_send_event(
				self.widget.det.as_ref().expect("det").borrow().connection.get_raw_conn(),
				false as u8,
				self.widget.det.as_ref().expect("det").borrow().screen().root() as xcb::ffi::xcb_window_t,
				(xcb::xproto::EVENT_MASK_SUBSTRUCTURE_NOTIFY |
					xcb::xproto::EVENT_MASK_SUBSTRUCTURE_REDIRECT) as u32,
				&ev as *const xcb::ffi::xproto::xcb_client_message_event_t
					as *const libc::c_char
			);



			/*
                    let cookie = xcb::ffi::xcb_intern_atom(
				self.widget.det.as_ref().expect("det").borrow().connection.get_raw_conn(),
				0,
				13, b"_NET_WM_STATE\0".as_ptr() as *const i8
			);
			let reply = xcb::ffi::xcb_intern_atom_reply(
				self.widget.det.as_ref().expect("det").borrow().connection.get_raw_conn(),
				cookie,
				std::ptr::null_mut(),
			);
			let cookie2 = xcb::ffi::xcb_intern_atom(
				self.widget.det.as_ref().expect("det").borrow().connection.get_raw_conn(),
				0,
				24, b"_NET_WM_STATE_FULLSCREEN\0".as_ptr() as *const i8
			);
			let reply2 = xcb::ffi::xcb_intern_atom_reply(
				self.widget.det.as_ref().expect("det").borrow().connection.get_raw_conn(),
				cookie2,
				std::ptr::null_mut(),
			);
			xcb::ffi::xcb_change_property(
				self.widget.det.as_ref().expect("det").borrow().connection.get_raw_conn(),
				xcb::ffi::XCB_PROP_MODE_REPLACE as u8,
				self.widget.true_window_id,
				(*reply).atom,
				xcb::ATOM as u32,
				32,
				if fullscreen { 1 } else { 0 },
				(&(*reply2).atom) as *const u32 as *const std::ffi::c_void,
			); */
		}
	}
}

