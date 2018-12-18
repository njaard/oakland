use antidote::RwLock;

use std::sync::mpsc;

pub(crate) struct EventPoster
{
	xcb_fd: std::os::unix::io::RawFd,
	event_r_fd: std::os::unix::io::RawFd,
	event_w_fd: std::os::unix::io::RawFd,
	pub(crate) receivers: RwLock<Vec<Box<FnMut()>>>,
}

impl EventPoster
{
	pub(crate) fn new(xcb: &xcb::base::Connection)
		-> EventPoster
	{
		let xcb_fd = unsafe { xcb::ffi::base::xcb_get_file_descriptor(xcb.get_raw_conn()) };

		let (event_r_fd, event_w_fd) =
			nix::unistd::pipe().expect("pipe");
		nix::fcntl::fcntl(
			event_r_fd,
			nix::fcntl::FcntlArg::F_SETFL(
				nix::fcntl::OFlag::O_NONBLOCK
			)
		).expect("fnctl nonblock on event notifier");
		EventPoster
		{
			xcb_fd,
			event_r_fd,
			event_w_fd,
			receivers: RwLock::new(vec!()),
		}
	}

	pub(crate) fn wait(&self) -> bool
	{
		use nix::sys::select::*;
		let mut rr = FdSet::new();
		rr.insert(self.event_r_fd);
		rr.insert(self.xcb_fd);
		let _ = select(None, Some(&mut rr), None, None, None);

		let c = rr.contains(self.event_r_fd);
		loop
		{
			let mut trash = [0u8; 4096];
			let r = nix::unistd::read( self.event_r_fd, &mut trash);
			if let Ok(r) = r
			{
				if r == 0 { break; }
			}
			else
			{
				break;
			}
		}
		c
	}

	pub(crate) fn process_channels(&self)
	{
		let mut e = self.receivers.write();
		for c in &mut *e
		{
			c();
		}
	}

	pub(crate) fn channel<T, F>(
		&self,
		mut f: F,
	) -> ChannelWrite<T>
	where
		T: 'static+Clone+Sized+Send,
		F: 'static+FnMut(T)
	{
		let (sr, rr) = mpsc::channel();
		let mut e = self.receivers.write();
		e.push(
			Box::new(
				move ||
				{
					while let Ok(o) = rr.try_recv()
					{
						f(o);
					}
				}
			)
		);
		ChannelWrite
		{
			event_w_fd: self.event_w_fd,
			sender: sr,
		}
	}
}


pub struct ChannelWrite<T>
{
	event_w_fd: std::os::unix::io::RawFd,
	sender: mpsc::Sender<T>,
}

impl<T> ChannelWrite<T>
{
	pub fn send(&self, obj: T)
	{
		self.sender.send(obj).unwrap();
		let _ = nix::unistd::write(self.event_w_fd, b"1");
	}
}
