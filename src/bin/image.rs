#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

use display::Widget;

fn main()
{
	let icon = image::open("/usr/share/icons/oxygen/base/128x128/actions/go-top.png")
		.unwrap();

	let g = display::Graphical::new();

	let icon = g.pixmap(&icon);

	let main = g.put(display::MainWindow::new("Window Title"));

	let label = main.put(display::Label::new(""));
	label.set_image(icon);
	label.set_geometry(display::Rectangle::coords(0, 0, 128, 128));

	g.exec();
}

