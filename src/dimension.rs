#[derive(Debug, Copy, Clone)]
pub struct Point
{
	pub x : i32,
	pub y : i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Size
{
	pub width : u32,
	pub height : u32,
}

impl Size
{
	pub fn with_width(&self, width: u32) -> Size
	{
		let mut s = *self;
		s.width = width;
		s
	}
	pub fn with_height(&self, height: u32) -> Size
	{
		let mut s = *self;
		s.height = height;
		s
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Rectangle
{
	pub pos : Point,
	pub size : Size,
}


impl Rectangle
{
	pub fn coords(x : i32, y : i32, width : u32, height : u32) -> Rectangle
	{
		Rectangle
		{
			pos: Point{x, y},
			size: Size{width, height},
		}
	}

	pub fn contains(&self, point : &Point) -> bool
	{
		point.x >= self.x() && point.y >= self.y()
			&& point.x < self.x()+(self.width() as i32)&& point.y < self.y()+(self.height() as i32)
	}

	///
	/// consider `self` and `point` to have the same coordinate system,
	/// then return `point` on `self`'s coordinate system
	pub fn map_to(&self, point : &Point) -> Point
	{
		let mut point = *point;
		point.x -= self.pos.x;
		point.x -= self.pos.y;
		point
	}

	pub fn x(&self) -> i32
	{
		self.pos.x
	}
	pub fn y(&self) -> i32
	{
		self.pos.y
	}
	pub fn width(&self) -> u32
	{
		self.size.width
	}
	pub fn height(&self) -> u32
	{
		self.size.height
	}
	pub fn set_position(&mut self, pos: &Point)
	{
		self.pos = *pos;
	}
	pub fn resize(&mut self, sz: &Size)
	{
		self.size = *sz;
	}
}
