use crate::*;

pub struct LineEdit
{
	widget : WidgetBase,
	text : String,
}

impl Widget for LineEdit
{
	fn as_widget(&self) -> &WidgetBase
	{
		&self.widget
	}
	fn as_widget_mut(&mut self) -> &mut WidgetBase
	{
		&mut self.widget
	}
	fn mouse_event(&self, _e: MouseEvent, _pos: &Point)
	{
		println!("got the event");
		self.repaint();
	}

	fn draw(&self, draw : &mut cairo::Cairo)
	{
		let height = 30i32;
		let width = 100i32;
		
		draw.fillcolor(Color::white());
		
		draw.set_antialias(cairo::antialias::Antialias::None);
		{ // top left
			draw.set_color(Color::rgb(0xc2, 0xbb, 0xb8));
			draw.pixel(0, 0);
			draw.set_color(Color::rgb(0xb3, 0xad, 0xab));
			draw.pixel(1, 0);
			draw.set_color(Color::rgb(0x94, 0x8f, 0x8c));
			draw.pixel(3, 0);
			
			draw.set_color(Color::rgb(0xb5, 0xaf, 0xac));
			draw.pixel(0, 1);
			draw.set_color(Color::rgb(0x9e, 0x9c, 0x99));
			draw.pixel(1, 1);
			draw.set_color(Color::rgb(0xbb, 0xba, 0xb7));
			draw.pixel(2, 1);
			
			draw.set_color(Color::rgb(0x9e, 0x99, 0x97));
			draw.pixel(0, 2);
			draw.set_color(Color::rgb(0xc1, 0xbc, 0xba));
			draw.pixel(1, 2);
			draw.set_color(Color::rgb(0xc5, 0xc4, 0xc1));
			draw.pixel(2, 2);
			
			draw.set_color(Color::rgb(0xb2, 0xab, 0xa8));
			draw.pixel(0, 3);
			draw.set_color(Color::rgb(0xdf, 0xde, 0xdb));
			draw.pixel(1, 3);
			draw.set_color(Color::rgb(0xe0, 0xde, 0xda));
			draw.pixel(2, 3);
			
			draw.set_color(Color::rgb(0xa4, 0x9d, 0x9b));
			draw.pixel(0, 4);
			draw.set_color(Color::rgb(0xe7, 0xe7, 0xe4));
			draw.pixel(1, 4);
		}
		
		{
			// left wall
			let gray18 = Color::rgb(0x9c, 0x97, 0x94);
			let gray19 = Color::rgb(0xe7, 0xe5, 0xe3);
			draw.set_color(gray18);
			draw.line(0, 5, 0, height-5);
			draw.set_color(gray19);
			draw.line(1, 5, 1, height-5);
		}
		
		{ // bottom left
			draw.set_color(Color::rgb(0xa7,0xa1,0x9e));
			draw.pixel(0, height-4);
			draw.set_color(Color::rgb(0xcc,0xcb,0xc9));
			draw.pixel(1, height-4);
			draw.set_color(Color::rgb(0xc8,0xc4,0xc0));
			draw.pixel(2, height-4);
			
			draw.set_color(Color::rgb(0xa7,0xa1,0x9e));
			draw.pixel(0, height-3);
			draw.set_color(Color::rgb(0xa1,0x9e,0x9c));
			draw.pixel(1, height-3);
			draw.set_color(Color::rgb(0xd2,0xd1,0xce));
			draw.pixel(2, height-3);
			draw.set_color(Color::rgb(0xc8,0xc4,0xbf));
			draw.pixel(3, height-3);
		
			draw.set_color(Color::rgb(0xb3,0xac,0xaa));
			draw.pixel(0, height-2);
			draw.set_color(Color::rgb(0x98,0x95,0x92));
			draw.pixel(1, height-2);
			draw.set_color(Color::rgb(0x9a,0x97,0x95));
			draw.pixel(2, height-2);
			draw.set_color(Color::rgb(0xc0,0xbe,0xbd));
			draw.pixel(3, height-2);

			draw.set_color(Color::rgb(0xba,0xb5,0xb2));
			draw.pixel(0, height-1);
			draw.set_color(Color::rgb(0xaf,0xa8,0xa6));
			draw.pixel(1, height-1);
			draw.set_color(Color::rgb(0x9a,0x95,0x93));
			draw.pixel(2, height-1);
			draw.set_color(Color::rgb(0x89,0x85,0x82));
			draw.pixel(3, height-1);
			
			draw.set_color(Color::rgb(0xbc,0xb6,0xb3));
			draw.pixel(0, height);
			draw.set_color(Color::rgb(0xba,0xb5,0xb2));
			draw.pixel(1, height);
			draw.set_color(Color::rgb(0xb5,0xae,0xab));
			draw.pixel(2, height);
			draw.set_color(Color::rgb(0xab,0xa5,0xa3));
			draw.pixel(3, height);
		}
		
		{ // top wall
			draw.set_color(Color::rgb(0xba, 0xb2, 0xaf));
			draw.line(4, 0, width, 0);
			draw.set_color(Color::rgb(0xe8, 0xe6, 0xe4));
			draw.line(4, 1, width, 1);
		}
		{ // bottom wall
			draw.set_color(Color::rgb(0xda, 0xd8, 0xd7));
			draw.line(4, height-2, width, height-2);
			draw.set_color(Color::rgb(0x79, 0x75, 0x72));
			draw.line(4, height-1, width, height-1);
			draw.set_color(Color::rgb(0xa6, 0x9f, 0x9d));
			draw.line(4, height, width, height);
		}
		
		{ // top-right
			draw.set_color(Color::rgb(0xbe,0xb7,0xb4));
			draw.pixel(width-4, 0);
			draw.set_color(Color::rgb(0xc0,0xba,0xb7));
			draw.pixel(width-3, 0);
			draw.set_color(Color::rgb(0xc2,0xbb,0xb8));
			draw.pixel(width-2, 0);
			draw.set_color(Color::rgb(0xc2,0xbb,0xb8));
			draw.pixel(width-1, 0);
			
			draw.set_color(Color::rgb(0xe2,0xe0,0xde));
			draw.pixel(width-4, 1);
			draw.set_color(Color::rgb(0xc5,0xc0,0xbe));
			draw.pixel(width-3, 1);
			draw.set_color(Color::rgb(0xbe,0xb7,0xb4));
			draw.pixel(width-2, 1);
			draw.set_color(Color::rgb(0xc2,0xbb,0xb8));
			draw.pixel(width-1, 1);
			
			draw.set_color(Color::rgb(0xe1,0xdf,0xdc));
			draw.pixel(width-4, 1);
			draw.set_color(Color::rgb(0xea,0xe8,0x8e));
			draw.pixel(width-3, 1);
			draw.set_color(Color::rgb(0xc1,0xbc,0xba));
			draw.pixel(width-2, 1);
			draw.set_color(Color::rgb(0xbe,0xb8,0xb5));
			draw.pixel(width-1, 1);
			
			draw.set_color(Color::rgb(0xdd,0xda,0xd6));
			draw.pixel(width-4, 2);
			draw.set_color(Color::rgb(0xe0,0xde,0xda));
			draw.pixel(width-3, 2);
			draw.set_color(Color::rgb(0xe0,0xde,0xda));
			draw.pixel(width-2, 2);
			draw.set_color(Color::rgb(0xb2,0xab,0xa8));
			draw.pixel(width-1, 2);
			
			draw.set_color(Color::rgb(0xe8,0xe7,0xe4));
			draw.pixel(width-2, 3);
			draw.set_color(Color::rgb(0xa4,0x9d,0x9b));
			draw.pixel(width-1, 3);
		}
		
		{ // right wall
			draw.set_color(Color::rgb(0xe7, 0xe6, 0xe4));
			draw.line(width-2, 4, width-2, height-4);
			draw.set_color(Color::rgb(0x9c, 0x97, 0x94));
			draw.line(width-1, 4, width-1, height-4);
		}


		{ // bottom-right
			draw.set_color(Color::rgb(0xc9,0xc4,0xbe));
			draw.pixel(width-4, height-4);
			draw.set_color(Color::rgb(0xd1,0xcd,0xc8));
			draw.pixel(width-3, height-4);
			draw.set_color(Color::rgb(0xd3,0xd2,0xd0));
			draw.pixel(width-2, height-4);
			draw.set_color(Color::rgb(0x9e,0x99,0x97));
			draw.pixel(width-1, height-4);

			draw.set_color(Color::rgb(0xd0,0xcd,0xc8));
			draw.pixel(width-4, height-3);
			draw.set_color(Color::rgb(0xe0,0xdd,0xdc));
			draw.pixel(width-3, height-3);
			draw.set_color(Color::rgb(0xa1,0x9e,0x9c));
			draw.pixel(width-2, height-3);
			draw.set_color(Color::rgb(0xa9,0xa3,0xa0));
			draw.pixel(width-1, height-3);

			draw.set_color(Color::rgb(0xc8,0xc6,0xc3));
			draw.pixel(width-4, height-2);
			draw.set_color(Color::rgb(0x97,0x95,0x92));
			draw.pixel(width-3, height-2);
			draw.set_color(Color::rgb(0x98,0x94,0x91));
			draw.pixel(width-2, height-2);
			draw.set_color(Color::rgb(0xb8,0xb0,0xae));
			draw.pixel(width-1, height-2);

			draw.set_color(Color::rgb(0x86,0x81,0x7e));
			draw.pixel(width-4, height-1);
			draw.set_color(Color::rgb(0x98,0x95,0x93));
			draw.pixel(width-3, height-1);
			draw.set_color(Color::rgb(0xb3,0xac,0xaa));
			draw.pixel(width-2, height-1);
			draw.set_color(Color::rgb(0xc0,0xba,0xb7));
			draw.pixel(width-1, height-1);

			draw.set_color(Color::rgb(0xae,0xa7,0xa5));
			draw.pixel(width-4, height-0);
			draw.set_color(Color::rgb(0xba,0xb2,0xaf));
			draw.pixel(width-3, height-0);
			draw.set_color(Color::rgb(0xc0,0xba,0xb7));
			draw.pixel(width-2, height-0);
			draw.set_color(Color::rgb(0xc0,0xba,0xb7));
			draw.pixel(width-1, height-0);
		}
		draw.set_color(Color::black());
		draw.move_to(0.0, (height-5) as f64);
		draw.show_text(&self.text);
	}
}


impl LineEdit
{
	pub fn new() -> LineEdit
	{
		let mut w = LineEdit
		{
			widget : WidgetBase::named("LineEdit"),
			text : "".into(),
		};
		
		w.widget.set_maximum_size(Size{ width:u32::max_value(), height:22 });
		w
	}
	
	pub fn set_text(&mut self, text : String)
	{
		self.text = text;
	}
}



