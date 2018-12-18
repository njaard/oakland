extern crate cairo;

#[derive(Debug, Copy, Clone)]
pub struct Color
{
	r : f64,
	g : f64,
	b : f64,
	a : f64,
}

impl Color
{
	pub fn black() -> Color
	{
		Color { r:0., g:0., b:0., a:1. }
	}
	pub fn white() -> Color
	{
		Color { r:1., g:1., b:1., a:1. }
	}
	pub fn gray() -> Color
	{
		Color { r:0.76, g:0.76, b:0.76, a:1. }
	}
	pub fn rgb(r : u32, g : u32, b : u32) -> Color
	{
		Color { r:r as f64/255.0, g:g as f64/255.0, b:b as f64/255.0, a:1.0}
	}
	pub fn rgba(r : u32, g : u32, b : u32, a : u32) -> Color
	{
		Color { r:r as f64/255.0, g:g as f64/255.0, b:b as f64/255.0, a:a as f64/255.0}
	}

	pub fn xcb_color(&self) -> u32
	{
		let mut v = 0u32;
		v |= ((self.a*255.0) as u32) << 24u32;
		v |= ((self.b*255.0) as u32) << 16u32;
		v |= ((self.g*255.0) as u32) << 8u32;
		v |= ((self.r*255.0) as u32) << 0u32;
		v
	}
}

pub trait ColorSetter
{
	fn set_color(&mut self, color : Color);
}

impl ColorSetter for cairo::Cairo
{
	fn set_color(&mut self, color : Color)
	{
		self.set_source_rgba(color.r, color.g, color.b, color.a)
	}
}

pub trait DrawPixel
{
	fn pixel(&mut self, x : i32, y : i32);
	fn line(&mut self, x1 : i32, y1 : i32, x2 : i32, y2 : i32);
	fn vgradient(&mut self, y1 : i32, y2 : i32, c1 : Color, c2 : Color);
	fn fillcolor(&mut self, c : Color);
	fn paste(&mut self, src: &cairo::surface::Surface);
}

impl DrawPixel for cairo::Cairo
{
	fn pixel(&mut self, x : i32, y : i32)
	{
		self.rectangle(x as f64, y as f64, 1., 1.);
		self.fill();
	}
	fn line(&mut self, x1 : i32, y1 : i32, x2 : i32, y2 : i32)
	{
		let (w,_) = self.device_to_user_distance(1.0, 1.0);
		self.set_line_width(w);
		self.move_to(x1 as f64+0.5, y1 as f64+0.5);
		self.line_to(x2 as f64+0.5, y2 as f64+0.5);
		self.stroke();
	}
	fn fillcolor(&mut self, c : Color)
	{
		self.set_source_rgba(c.r, c.g, c.b, c.a);
		self.rectangle(0., 0., 1000., 1000.);
		self.fill();
	}
	fn vgradient(&mut self, y1 : i32, y2 : i32, c1 : Color, c2 : Color)
	{
		self.save();
		let mut pattern = cairo::pattern::Pattern::create_linear(
			0.0, 0.0, 0.0, y2 as f64
		);
		
		pattern.add_color_stop_rgba(0.0, c1.r, c1.g, c1.b, c1.a);
		pattern.add_color_stop_rgba(1.0, c2.r, c2.g, c2.b, c2.a);
		self.rectangle(0.0, y1 as f64, 100000.0, y2 as f64);
		self.set_source(&mut pattern);
		self.fill();
		self.restore();
	}

	fn paste(&mut self, src: &cairo::surface::Surface)
	{

		let mut src=
			unsafe
			{
				&mut* std::mem::transmute::<
					*const cairo::surface::Surface,
					*mut cairo::surface::Surface
				>(src as *const cairo::surface::Surface)
			};

		self.save();
		self.set_source_surface(&mut src, 0.0, 0.0);
		self.paint();
		self.restore();

	}
}
