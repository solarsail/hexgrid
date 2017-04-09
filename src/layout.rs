use geometry::{Mat2, Point, SQRT3};
use std::f64::consts::PI;

pub struct Orientation {
    pub mat2screen: Mat2,
    pub mat2coord: Mat2,
    pub start_angle: f64 // * 60deg
}

pub const POINTY_TOP: Orientation = Orientation {
    mat2screen: [[SQRT3, SQRT3/2.0], [0.0, 1.5]],
    mat2coord:  [[SQRT3/3.0, -1.0/3.0], [0.0, 2.0/3.0]],
    start_angle: 0.5
};

#[allow(dead_code)]
pub const FLAT_TOP: Orientation = Orientation {
    mat2screen: [[1.5, 0.0], [SQRT3/2.0, SQRT3]],
    mat2coord:  [[2.0/3.0, 0.0], [-1.0/3.0, SQRT3/3.0]],
    start_angle: 0.0
};

pub struct Layout {
    pub orientation: Orientation,
    pub radius: Point,
    pub origin: Point
}

impl Layout {
    pub fn new(d: Orientation, r: Point, o: Point) -> Layout {
        Layout {
            orientation: d,
            radius: r,
            origin: o
        }
    }

    /// 顶点相对于中心的距离向量。
    ///
    ///   4 *
    /// 3 *   * 5
    /// 2 *   * 0
    ///     * 1
    ///
    pub fn vertex_offset(&self, index: usize) -> Point {
        let radius = self.radius;
        let angle = PI * (self.orientation.start_angle + index as f64) / 3.0;
        [radius.x * angle.cos(), radius.y * angle.sin()].into()
    }

    pub fn move_to(self, o: Point) -> Layout {
    	Layout {
    		orientation: self.orientation,
    		radius: self.radius,
    		origin: o
    	}
    }

    pub fn scale(self, s: Point) -> Layout {
    	Layout {
    		orientation: self.orientation,
    		radius: [self.radius.x * s.x, self.radius.y * s.y].into(),
    		origin: self.origin
    	}
    }
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn vertex_offset() {
		let layout = Layout::new(POINTY_TOP, [1.0, 0.5].into(), [2.5, 3.0].into());
		let expected: Vec<Point> = vec![
			[SQRT3 / 2.0, 0.25].into(),
			[0.0, 0.5].into(),
			[-SQRT3 / 2.0, 0.25].into(),
			[-SQRT3 / 2.0, -0.25].into(),
			[0.0, -0.5].into(),
			[SQRT3 / 2.0, -0.25].into(),
		];
		let offsets = (0..6).map(|i| layout.vertex_offset(i)).collect::<Vec<Point>>();
		assert_eq!(expected, offsets);
	}
}