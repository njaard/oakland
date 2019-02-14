/*
use oakland::Widget;
use std::borrow::Borrow;

fn main()
{
	let mut g = oakland::Graphical::new();
	
	let mut layout = oakland::VBoxLayout::new();
	
	let mut w = oakland::PushButton::new();
	w.set_text("Hello".into());
	{
		let g = g.clone();
		w.on_click(
			move || g.trace(0, &mut std::io::stderr())
		);
	}
	layout.put(w);

	let mut w = oakland::LineEdit::new();
	w.set_text("Hello".into());
	layout.put(w);
	
	std::rc::Rc::get_mut(&mut g).unwrap().put(layout);

	g.trace(0, &mut std::io::stderr());

	g.exec();
}

*/
fn main() {}
