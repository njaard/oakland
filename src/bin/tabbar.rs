use display::Widget;

fn main()
{
	let g = display::Graphical::new();

	let main = g.put(display::MainWindow::new("Window Title"));

	let tabs = main.put(display::TabBar::new());
	tabs.add("Temp".into());
	tabs.add("Program".into());
	tabs.add("Settings".into());
	tabs.set_geometry(display::Rectangle::coords(0, 0, 1000, 30));

	g.exec();
}

