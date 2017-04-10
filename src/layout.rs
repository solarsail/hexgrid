use geometry::{Mat2, Point, PointPair, SQRT3};
use std::f64::consts::PI;
use coordinates::{Coordinates, EdgeCoordinates};

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
    pub fn new<T: Into<Point>>(d: Orientation, r: T, o: T) -> Layout {
        Layout {
            orientation: d,
            radius: r.into(),
            origin: o.into()
        }
    }

    /// 顶点相对于中心的距离向量。
    //
    //     * 2
    // 3 *   * 1
    // 4 *   * 0
    //   5 *
    //
    pub fn vertex_offset(&self, index: usize) -> Point {
        let radius = self.radius;
        let angle = PI * (self.orientation.start_angle - index as f64) / 3.0;
        [radius.x * angle.cos(), radius.y * angle.sin()].into()
    }

    pub fn coord_at<T: Into<Point>>(&self, p: T) -> Coordinates {
        let p: Point = p.into();
        let mat = self.orientation.mat2coord;
        let pt: Point = [(p.x - self.origin.x) / self.radius.x, (p.y - self.origin.y) / self.radius.y].into();
        let q = mat[0][0] * pt.x + mat[0][1] * pt.y;
        let r = mat[1][0] * pt.x + mat[1][1] * pt.y;
        Coordinates::round(q, r)
    }

    pub fn center_of_hex(&self, c: Coordinates)  -> Point {
        let mat = self.orientation.mat2screen;
        let x = (mat[0][0] * c.q as f64 + mat[0][1] * c.r as f64) * self.radius.x;
        let y = (mat[1][0] * c.q as f64 + mat[1][1] * c.r as f64) * self.radius.y;
        [x + self.origin.x, y + self.origin.y].into()
    }

    pub fn vertices_of_hex(&self, c: Coordinates) -> Vec<Point> {
        let center = self.center_of_hex(c);
        (0..6).map(|i| self.vertex_offset(i) + center).collect::<Vec<Point>>()
    }

    pub fn vertices_of_edge(&self, e: EdgeCoordinates) -> PointPair {
        let center = self.center_of_hex(e.coord);
        [center + self.vertex_offset(e.index), center + self.vertex_offset(e.index+1)].into()
    }

    pub fn all_edges_of_hex(&self, c: Coordinates) -> Vec<PointPair> {
        let mut edges: Vec<PointPair> = vec![[[0.0, 0.0], [0.0, 0.0]].into(); 6];
        let center = self.center_of_hex(c);
        for i in 0..6 {
            let offset = self.vertex_offset(i);
            let p = center + offset;
            edges[i].a = p;
            edges[(i+5) % 6].b = p;
        }
        edges
    }

    pub fn bounding_box_of(&self, c: Coordinates) -> [f64; 4] {
        let center = self.center_of_hex(c);
        let x1 = center.x + self.vertex_offset(3).x;
        let x2 = center.x + self.vertex_offset(0).x;
        let y1 = center.y + self.vertex_offset(4).y;
        let y2 = center.y + self.vertex_offset(1).y;
        [x1, y1, x2-x1, y2-y1]
    }

    pub fn move_to<T: Into<Point>>(self, o: T) -> Layout {
        Layout {
            orientation: self.orientation,
            radius: self.radius,
            origin: o.into()
        }
    }

    pub fn scale<T: Into<Point>>(self, s: T) -> Layout {
        let s: Point = s.into();
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
        let layout = Layout::new(POINTY_TOP, [1.0, 0.5], [2.5, 3.0]);
        let expected: Vec<Point> = vec![
        [SQRT3 / 2.0, 0.25].into(),
        [SQRT3 / 2.0, -0.25].into(),
        [0.0, -0.5].into(),
        [-SQRT3 / 2.0, -0.25].into(),
        [-SQRT3 / 2.0, 0.25].into(),
        [0.0, 0.5].into(),
        ];
        let offsets = (0..6).map(|i| layout.vertex_offset(i)).collect::<Vec<Point>>();
        assert_eq!(expected, offsets);
    }

    #[test]
    fn coord_at() {
        let layout = Layout::new(POINTY_TOP, [10.0, 5.0], [2.5, 3.0]);
        let expected = Coordinates::at(1, 2);
        let value = layout.coord_at([44.5, 17.0]);
        assert_eq!(expected, value);
    }

    #[test]
    fn center_of_hex() {
        let layout = Layout::new(POINTY_TOP, [10.0, 5.0], [2.5, 3.0]);
        let expected: Point = [20.0 * SQRT3 + 2.5, 18.0].into();
        let value = layout.center_of_hex(Coordinates::at(1, 2));
        assert_eq!(expected, value);
    }
}
