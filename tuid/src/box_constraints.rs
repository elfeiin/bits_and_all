use crate::size::Size;

#[derive(Clone, Copy, Debug)]
pub struct BoxConstraints {
    min: Size,
    max: Size,
}

impl BoxConstraints {
    /// An unbounded box constraints object.
    ///
    /// Can be satisfied by any nonnegative size.
    pub const BIG: BoxConstraints = BoxConstraints {
        min: Size::ZERO,
        max: Size::new(usize::MAX as usize, usize::MAX as usize),
    };

    /// Create a new box constraints object.
    ///
    /// Create constraints based on minimum and maximum size.
    ///
    /// The given sizes are also [rounded away from zero],
    /// so that the layout is aligned to integers.
    ///
    /// [rounded away from zero]: struct.Size.html#method.expand
    pub fn new(min: Size, max: Size) -> BoxConstraints {
        BoxConstraints { min, max }
    }

    /// Create a "tight" box constraints object.
    ///
    /// A "tight" constraint can only be satisfied by a single size.
    ///
    /// The given size is also [rounded away from zero],
    /// so that the layout is aligned to integers.
    ///
    /// [rounded away from zero]: struct.Size.html#method.expand
    pub fn tight(size: Size) -> BoxConstraints {
        let size = size;
        BoxConstraints {
            min: size,
            max: size,
        }
    }

    /// Create a "loose" version of the constraints.
    ///
    /// Make a version with zero minimum size, but the same maximum size.
    pub fn loosen(&self) -> BoxConstraints {
        BoxConstraints {
            min: Size::ZERO,
            max: self.max,
        }
    }

    /// Clamp a given size so that it fits within the constraints.
    ///
    /// The given size is also [rounded away from zero],
    /// so that the layout is aligned to integers.
    ///
    /// [rounded away from zero]: struct.Size.html#method.expand
    pub fn constrain(&self, size: impl Into<Size>) -> Size {
        size.into().clamp(self.min, self.max)
    }

    /// Returns the max size of these constraints.
    pub fn max(&self) -> Size {
        self.max
    }

    /// Returns the min size of these constraints.
    pub fn min(&self) -> Size {
        self.min
    }

    /// Whether there is an upper bound on the width.
    pub fn is_width_bounded(&self) -> bool {
        true
    }

    /// Whether there is an upper bound on the height.
    pub fn is_height_bounded(&self) -> bool {
        true
    }

    /// Shrink min and max constraints by size
    ///
    /// The given size is also [rounded away from zero],
    /// so that the layout is aligned to integers.
    ///
    /// [rounded away from zero]: struct.Size.html#method.expand
    pub fn shrink(&self, diff: impl Into<Size>) -> BoxConstraints {
        let diff = diff.into();
        let min = Size::new(
            (self.min().width - diff.width).max(0),
            (self.min().height - diff.height).max(0),
        );
        let max = Size::new(
            (self.max().width - diff.width).max(0),
            (self.max().height - diff.height).max(0),
        );

        BoxConstraints::new(min, max)
    }

    /// Test whether these constraints contain the given `Size`.
    pub fn contains(&self, size: impl Into<Size>) -> bool {
        let size = size.into();
        (self.min.width <= size.width && size.width <= self.max.width)
            && (self.min.height <= size.height && size.height <= self.max.height)
    }

    // pub fn constrain_aspect_ratio(&self, aspect_ratio: usize, width: usize) -> Size {
    //     // Minimizing/maximizing based on aspect ratio seems complicated, but in reality everything
    //     // is linear, so the amount of work to do is low.
    //     let ideal_size = Size {
    //         width,
    //         height: width * aspect_ratio,
    //     };

    //     // Firstly check if we can simply return the exact requested
    //     if self.contains(ideal_size) {
    //         return ideal_size;
    //     }

    //     // Then we check if any `Size`s with our desired aspect ratio are inside the constraints.
    //     // TODO this currently outputs garbage when things are < 0.
    //     let min_w_min_h = self.min.height / self.min.width;
    //     let max_w_min_h = self.min.height / self.max.width;
    //     let min_w_max_h = self.max.height / self.min.width;
    //     let max_w_max_h = self.max.height / self.max.width;

    //     // When the aspect ratio line crosses the constraints, the closest point must be one of the
    //     // two points where the aspect ratio enters/exits.

    //     // When the aspect ratio line doesn't intersect the box of possible sizes, the closest
    //     // point must be either (max width, min height) or (max height, min width). So all we have
    //     // to do is check which one of these has the closest aspect ratio.

    //     // Check each possible intersection (or not) of the aspect ratio line with the constraints
    //     if aspect_ratio > min_w_max_h {
    //         // outside max height min width
    //         Size {
    //             width: self.min.width,
    //             height: self.max.height,
    //         }
    //     } else if aspect_ratio < max_w_min_h {
    //         // outside min height max width
    //         Size {
    //             width: self.max.width,
    //             height: self.min.height,
    //         }
    //     } else if aspect_ratio > min_w_min_h {
    //         // hits the constraints on the min width line
    //         if width < self.min.width {
    //             // we take the point on the min width
    //             Size {
    //                 width: self.min.width,
    //                 height: self.min.width * aspect_ratio,
    //             }
    //         } else if aspect_ratio < max_w_max_h {
    //             // exits through max.width
    //             Size {
    //                 width: self.max.width,
    //                 height: self.max.width * aspect_ratio,
    //             }
    //         } else {
    //             // exits through max.height
    //             Size {
    //                 width: self.max.height * aspect_ratio.recip(),
    //                 height: self.max.height,
    //             }
    //         }
    //     } else {
    //         // final case is where we hit constraints on the min height line
    //         if width < self.min.width {
    //             // take the point on the min height
    //             Size {
    //                 width: self.min.height * aspect_ratio.recip(),
    //                 height: self.min.height,
    //             }
    //         } else if aspect_ratio > max_w_max_h {
    //             // exit thru max height
    //             Size {
    //                 width: self.max.height * aspect_ratio.recip(),
    //                 height: self.max.height,
    //             }
    //         } else {
    //             // exit thru max width
    //             Size {
    //                 width: self.max.width,
    //                 height: self.max.width * aspect_ratio,
    //             }
    //         }
    //     }
    // }
}

impl From<(u16, u16)> for BoxConstraints {
    fn from(s: (u16, u16)) -> Self {
        Self {
            min: Size::ZERO,
            max: s.into(),
        }
    }
}
