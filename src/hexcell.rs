use coordinates::Coordinates;
use layout::Layout;
use geometry::{Point, PointPair};

#[derive(Debug)]
struct HexCell {
    coord: Coordinates,
}

impl HexCell {
	pub fn at(c: Coordinates) -> HexCell {
		HexCell {
			coord: c,
		}
	}

	pub fn from_pixel(p: Point, layout: &Layout) -> HexCell {
        let mat = layout.orientation.mat2coord;
        let origin = layout.origin;
        let radius = layout.radius;
        let pt: Point = [(p.x - origin.x) / radius.x, (p.y - origin.y) / radius.y].into();
        let q = mat[0][0] * pt.x + mat[0][1] * pt.y;
        let r = mat[1][0] * pt.x + mat[1][1] * pt.y;
        HexCell::at(Coordinates::round(q, r))
    }

	pub fn neighbour(&self, d: usize) -> Result<HexCell, String> {
		self.coord.neighbour(d).map(|c| HexCell { coord: c })
	}

	pub fn center(&self, layout: &Layout) -> Point {
        let mat = layout.orientation.mat2screen;
        let radius = layout.radius;
        let origin = layout.origin;
        let x = (mat[0][0] * self.coord.q as f64 + mat[0][1] * self.coord.r as f64) * radius.x;
        let y = (mat[1][0] * self.coord.q as f64 + mat[1][1] * self.coord.r as f64) * radius.y;
        [x + origin.x, y + origin.y].into()
    }

    pub fn vertices(&self, layout: &Layout) -> [Point; 6] {
        let mut vertices = [[0.0, 0.0].into(); 6];
        let center = self.center(layout);
        for i in 0..6 {
            let offset = layout.vertex_offset(i);
            vertices[i] = center + offset;
        }
        vertices
    }

    pub fn edge_towards(&self, t: usize, layout: &Layout) -> Option<PointPair> {
        if t < 3 {
	        let center = self.center(layout);
    	    let v1 = center + layout.vertex_offset(t);
        	let v2 = center + layout.vertex_offset(t+1);
        	Some([v1, v2].into())
        } else {
        	None
        }
    }

    pub fn edges_vertices(&self, layout: &Layout) -> [PointPair; 6] {
        let mut edges: [PointPair; 6] = [[0.0, 0.0, 0.0, 0.0].into(); 6];
        let center = self.center(layout);
        for i in 0..6 {
            let offset = layout.vertex_offset(i);
            let p = center + offset;
            edges[i].a = p;
            edges[(i+5) % 6].b = p;
        }
        edges
    }
}