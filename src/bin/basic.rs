extern crate display;

use display::Widget;
use std::borrow::Borrow;

fn main()
{
	let mut g = display::Graphical::new();
	
	let mut layout = display::VBoxLayout::new();
	
	let mut w = display::PushButton::new();
	w.set_text("Hello".into());
	{
		let g = g.clone();
		w.on_click(
			move || g.trace(0, &mut std::io::stderr())
		);
	}
	layout.put(w);

	let mut w = display::LineEdit::new();
	w.set_text("Hello".into());
	layout.put(w);
	
	std::rc::Rc::get_mut(&mut g).unwrap().put(layout);

	g.trace(0, &mut std::io::stderr());

	g.exec();
}

