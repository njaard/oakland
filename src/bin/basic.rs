extern crate display;


fn main()
{
	let g = display::Graphical::new();
	
	display::Widget::new(&g);
	
	g.exec();
}

