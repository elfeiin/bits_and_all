use crate::{rect::Rect, size::Size, vec2::Vec2, Data, Point, box_constraints::BoxConstraints};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis {
    /// The x axis
    Horizontal,
    /// The y axis
    Vertical,
}

impl Axis {
    /// Get the axis perpendicular to this one.
    pub fn cross(self) -> Axis {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal,
        }
    }

    /// Extract from the argument the magnitude along this axis
    pub fn major(self, coords: Size) -> usize {
        match self {
            Axis::Horizontal => coords.width,
            Axis::Vertical => coords.height,
        }
    }

    /// Extract from the argument the magnitude along the perpendicular axis
    pub fn minor(self, coords: Size) -> usize {
        self.cross().major(coords)
    }

    /// Extract the extent of the argument in this axis as a pair.
    pub fn major_span(self, rect: Rect) -> (usize, usize) {
        match self {
            Axis::Horizontal => (rect.x0, rect.x1),
            Axis::Vertical => (rect.y0, rect.y1),
        }
    }

    /// Extract the extent of the argument in the minor axis as a pair.
    pub fn minor_span(self, rect: Rect) -> (usize, usize) {
        self.cross().major_span(rect)
    }

    /// Extract the coordinate locating the argument with respect to this axis.
    pub fn major_pos(self, pos: Point) -> usize {
        match self {
            Axis::Horizontal => pos.x,
            Axis::Vertical => pos.y,
        }
    }

    /// Extract the coordinate locating the argument with respect to this axis.
    pub fn major_vec(self, vec: Vec2) -> usize {
        match self {
            Axis::Horizontal => vec.x,
            Axis::Vertical => vec.y,
        }
    }

    /// Extract the coordinate locating the argument with respect to the perpendicular axis.
    pub fn minor_pos(self, pos: Point) -> usize {
        self.cross().major_pos(pos)
    }

    /// Extract the coordinate locating the argument with respect to the perpendicular axis.
    pub fn minor_vec(self, vec: Vec2) -> usize {
        self.cross().major_vec(vec)
    }

    /// Arrange the major and minor measurements with respect to this axis such that it forms
    /// an (x, y) pair.
    pub fn pack(self, major: usize, minor: usize) -> (usize, usize) {
        match self {
            Axis::Horizontal => (major, minor),
            Axis::Vertical => (minor, major),
        }
    }

    /// Generate constraints with new values on the major axis.
    pub(crate) fn constraints(
        self,
        bc: &BoxConstraints,
        min_major: usize,
        major: usize,
    ) -> BoxConstraints {
        match self {
            Axis::Horizontal => BoxConstraints::new(
                Size::new(min_major, bc.min().height),
                Size::new(major, bc.max().height),
            ),
            Axis::Vertical => BoxConstraints::new(
                Size::new(bc.min().width, min_major),
                Size::new(bc.max().width, major),
            ),
        }
    }
}

impl Data for Axis {}
