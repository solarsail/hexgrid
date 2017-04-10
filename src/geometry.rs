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

impl From<[[f64; 2]; 2]> for PointPair {
    fn from(p: [[f64; 2]; 2]) -> PointPair {
        PointPair {
            a: Point { x: p[0][0], y: p[0][1] },
            b: Point { x: p[1][0], y: p[1][1] },
        }
    }
}

impl<'a> From<&'a [[f64; 2]; 2]> for PointPair {
    fn from(p: &[[f64; 2]; 2]) -> PointPair {
        PointPair {
            a: Point { x: p[0][0], y: p[0][1] },
            b: Point { x: p[1][0], y: p[1][1] },
        }
    }
}

impl From<[f64; 4]> for PointPair {
    fn from(p: [f64; 4]) -> PointPair {
        PointPair{
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

impl From<PointPair> for [f64; 4] {
    fn from(p: PointPair) -> [f64; 4] {
        [p.a.x, p.a.y, p.b.x, p.b.y]
    }
}

impl<'a> From<&'a PointPair> for [f64; 4] {
    fn from(p: &PointPair) -> [f64; 4] {
        [p.a.x, p.a.y, p.b.x, p.b.y]
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

impl<'a> From<&'a Point> for [f64; 2] {
    fn from(p: &Point) -> [f64;2] {
        [p.x, p.y]
    }
}

/*
impl From<[Point; 6]> for [[f64; 2]; 6] {
    fn from(p: [Point; 6]) -> [[f64; 2]; 6] {
        [
            [p[0].x, p[0].y],
            [p[1].x, p[1].y],
            [p[2].x, p[2].y],
            [p[3].x, p[3].y],
            [p[4].x, p[4].y],
            [p[5].x, p[5].y],
        ]
    }
}
*/
