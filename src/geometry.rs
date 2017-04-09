use std::ops::Add;


pub type Mat2 = [[f64;2];2];
pub const SQRT3: f64 = 1.7320508;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64
}


impl PartialEq for Point {
    fn eq(&self, rhs: &Point) -> bool {
    	(self.x - rhs.x).abs() < 1.0e-6
    }
}

impl Eq for Point {}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
    	Point {
    		x: self.x + rhs.x,
    		y: self.y + rhs.y,
    	}
    }
}

impl<'a> Add<Point> for &'a Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
    	Point {
    		x: self.x + rhs.x,
    		y: self.y + rhs.y,
    	}
    }
}

impl<'a> Add<&'a Point> for Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Point {
    	Point {
    		x: self.x + rhs.x,
    		y: self.y + rhs.y,
    	}
    }
}

impl<'a, 'b> Add<&'a Point> for &'b Point {
    type Output = Point;

    fn add(self, rhs: &'a Point) -> Point {
    	Point {
    		x: self.x + rhs.x,
    		y: self.y + rhs.y,
    	}
    }
}

impl From<[f64; 2]> for Point {
	fn from(p: [f64; 2]) -> Point {
		Point { x: p[0], y: p[1] }
	}
}

impl<'a> From<&'a [f64; 2]> for Point {
	fn from(p: &[f64; 2]) -> Point {
		Point { x: p[0], y: p[1] }
	}
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PointPair {
    pub a: Point,
    pub b: Point
}

impl From<[f64; 4]> for PointPair {
    fn from(p: [f64; 4]) -> PointPair {
    	PointPair {
    		a: Point { x: p[0], y: p[1] },
    		b: Point { x: p[2], y: p[3] },
    	}
    }
}

impl<'a> From<&'a [f64; 4]> for PointPair {
    fn from(p: &[f64; 4]) -> PointPair {
    	PointPair {
    		a: Point { x: p[0], y: p[1] },
    		b: Point { x: p[2], y: p[3] },
    	}
    }
}

impl From<[Point; 2]> for PointPair {
    fn from(p: [Point; 2]) -> PointPair {
    	PointPair {
    		a: p[0],
    		b: p[1],
    	}
    }
}

impl<'a> From<&'a [Point; 2]> for PointPair {
    fn from(p: &[Point; 2]) -> PointPair {
    	PointPair {
    		a: p[0],
    		b: p[1],
    	}
    }
}