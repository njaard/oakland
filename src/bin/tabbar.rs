use oakland::Widget;

fn main()
{
	let g = oakland::Graphical::new();

	let main = g.put(oakland::MainWindow::new("Window Title"));

	let tabs = main.put(oakland::TabBar::new());
	tabs.add("Temp".into());
	tabs.add("Program".into());
	tabs.add("Settings".into());
	tabs.set_geometry(oakland::Rectangle::coords(0, 0, 1000, 30));

	g.exec();
}

