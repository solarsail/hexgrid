use std::ops::{Add, Sub, Mul};

//              q
//          --7
//       --
//    -- 
//   |
//   |
//   |
//   v
//   r
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Coordinates {
	pub q: i32, // column
	pub r: i32, // row
}

// Counter-clock wise
const DIRECTIONS: [Coordinates; 6] = [
    Coordinates { q: 1, r: 0 },  // E
    Coordinates { q: 1, r:-1 },  // NE
    Coordinates { q: 0, r:-1 },  // NW
    Coordinates { q:-1, r: 0 },  // W
    Coordinates { q:-1, r: 1 },  // SW
    Coordinates { q: 0, r: 1 },  // SE
];

impl Coordinates {
    /// Create a new Coordinates at the specified location.
	pub fn at(q: i32, r: i32) -> Coordinates {
		Coordinates { q: q, r: r }
	}

    pub fn round(q: f64, r: f64) -> Coordinates {
        let mut rq = q.round();
        let mut rr = r.round();
        let rs = (-q-r).round();
        let dq = (rq - q).abs();
        let dr = (rr - r).abs();
        let ds = (rs - (-q-r)).abs();
        if dq > dr && dq > ds {
            rq = -rr-rs;
        } else if dr > ds {
            rr = -rq-rs;
        }
        Coordinates::at(rq as i32, rr as i32)
    }

    /// Get the distance from origin, in tiles.
	pub fn length(&self) -> i32 {
    	(self.q.abs() + self.r.abs() + (-self.q-self.r).abs()) / 2
    }

    /// Get the distance to another grid, in tiles.
    pub fn distance_to(&self, rhs: Coordinates) -> i32 {
        (self - rhs).length()
    }

    /// Get a neighbour grid in direction 'd' (range 0..6).
    pub fn neighbour(&self, d: usize) -> Result<Coordinates, String> {
        Coordinates::unit(d).map(|u| self + u)
    }

    /// Obtain a unit vector, in direction 'd' (range 0..6).
    pub fn unit(d: usize) -> Result<Coordinates, String> {
        if d < 6 {
            Ok(DIRECTIONS[d])
        } else {
            Err(format!("Invalid direction: {}", d))
        }
    }

    /// Coordinates sequence composing a line to the specified location.
    pub fn line_to(&self, rhs: Coordinates) -> Vec<Coordinates> {
        let len = self.distance_to(rhs);
        (0..len+1).map(|i| Coordinates::round(
            // Ensure the grids are to the same direction when points fall on edges
            // c.f. http://www.redblobgames.com/grids/hexagons/#line-drawing
            interpolate(self.q as f64 + 1e-6, rhs.q as f64 + 1e-6, i as f64 / len as f64),
            interpolate(self.r as f64, rhs.r as f64, i as f64 / len as f64),
            )).collect::<Vec<Coordinates>>()
    }
}

fn interpolate(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

impl Add for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Coordinates) -> Coordinates {
        Coordinates::at(self.q + rhs.q, self.r + rhs.r)
    }
}

impl<'a> Add<Coordinates> for &'a Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Coordinates) -> Coordinates {
        Coordinates::at(self.q + rhs.q, self.r + rhs.r)
    }
}

impl<'a> Add<&'a Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: &Coordinates) -> Coordinates {
        Coordinates::at(self.q + rhs.q, self.r + rhs.r)
    }
}

impl<'a, 'b> Add<&'a Coordinates> for &'b Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: &'a Coordinates) -> Coordinates {
        Coordinates::at(self.q + rhs.q, self.r + rhs.r)
    }
}

impl Sub for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Coordinates) -> Coordinates {
        Coordinates::at(self.q - rhs.q, self.r - rhs.r)
    }
}

impl<'a> Sub<Coordinates> for &'a Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Coordinates) -> Coordinates {
        Coordinates::at(self.q - rhs.q, self.r - rhs.r)
    }
}

impl<'a> Sub<&'a Coordinates> for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: &Coordinates) -> Coordinates {
        Coordinates::at(self.q - rhs.q, self.r - rhs.r)
    }
}

impl<'a, 'b> Sub<&'a Coordinates> for &'b Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: &'a Coordinates) -> Coordinates {
        Coordinates::at(self.q - rhs.q, self.r - rhs.r)
    }
}

impl Mul<i32> for Coordinates {
    type Output = Coordinates;

    fn mul(self, rhs: i32) -> Coordinates {
        Coordinates::at(self.q * rhs, self.r * rhs)
    }
}

impl<'a> Mul<i32> for &'a Coordinates {
    type Output = Coordinates;

    fn mul(self, rhs: i32) -> Coordinates {
        Coordinates::at(self.q * rhs, self.r * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbours() {
        let c = Coordinates::at(3, 4);
        let expected = vec![
            Coordinates::at(4, 4),
            Coordinates::at(4, 3),
            Coordinates::at(3, 3),
            Coordinates::at(2, 4),
            Coordinates::at(2, 5),
            Coordinates::at(3, 5),
        ];
        let neighbours = (0..6).flat_map(|d| c.neighbour(d)).collect::<Vec<Coordinates>>();
        assert_eq!(neighbours, expected);
        let expected = Err("Invalid direction: 6".into());
        assert_eq!(c.neighbour(6), expected);
    }

    #[test]
    fn add() {
        let a = Coordinates::at(3, 4);
        let b = Coordinates::at(6, 7);
        let c = Coordinates::at(9, 11);
        assert_eq!(a + b, c);
    }

    #[test]
    fn sub() {
        let a = Coordinates::at(3, 4);
        let b = Coordinates::at(6, 7);
        let c = Coordinates::at(-3, -3);
        assert_eq!(a - b, c);
    }

    #[test]
    fn mul() {
        let a = Coordinates::at(3, 4);
        let c = Coordinates::at(9, 12);
        assert_eq!(a * 3, c);
    }

    #[test]
    fn length() {
        let a = Coordinates::at(3, -4);
        assert_eq!(a.length(), 4);
        let a = Coordinates::at(-3, -4);
        assert_eq!(a.length(), 7);
        let a = Coordinates::at(3, 4);
        assert_eq!(a.length(), 7);
        let a = Coordinates::at(-3, 4);
        assert_eq!(a.length(), 4);
    }

    #[test]
    fn distance_to() {
        let a = Coordinates::at(3, -4);
        let b = Coordinates::at(-3, 4);
        assert_eq!(a.distance_to(b), 8);
        assert_eq!(b.distance_to(a), 8);
    }

    #[test]
    fn round() {
        let a = Coordinates::at(4, 5);
        let b = Coordinates::round(4.4, 4.8);
        assert_eq!(a, b);
        let b = Coordinates::round(3.7, 5.2);
        assert_eq!(a, b);
    }

    #[test]
    fn line_to() {
        let a = Coordinates::at(-5, 0);
        let b = Coordinates::at(1, -3);
        let expected = vec![
            Coordinates::at(-5, 0),
            Coordinates::at(-4, 0),
            Coordinates::at(-3,-1),
            Coordinates::at(-2,-1),
            Coordinates::at(-1,-2),
            Coordinates::at( 0,-2),
            Coordinates::at( 1,-3),
        ];
        assert_eq!(a.line_to(b), expected);
        let b = Coordinates::at(-1, 1);
        let expected = vec![
            Coordinates::at(-5, 0),
            Coordinates::at(-4, 0),
            Coordinates::at(-3, 0),
            Coordinates::at(-3, 1),
            Coordinates::at(-2, 1),
            Coordinates::at(-1, 1),
        ];
        assert_eq!(a.line_to(b), expected);
    }
}
