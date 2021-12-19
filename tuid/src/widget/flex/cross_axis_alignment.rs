use crate::Data;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CrossAxisAlignment {
    /// Top or leading.
    ///
    /// In a vertical container, widgets are top aligned. In a horiziontal
    /// container, their leading edges are aligned.
    Start,
    /// Widgets are centered in the container.
    Center,
    /// Bottom or trailing.
    ///
    /// In a vertical container, widgets are bottom aligned. In a horiziontal
    /// container, their trailing edges are aligned.
    End,
    /// Align on the baseline.
    ///
    /// In a horizontal container, widgets are aligned along the calculated
    /// baseline. In a vertical container, this is equivalent to `End`.
    ///
    /// The calculated baseline is the maximum baseline offset of the children.
    Baseline,
    /// Fill the available space.
    ///
    /// The size on this axis is the size of the largest widget;
    /// other widgets must fill that space.
    Fill,
}

impl CrossAxisAlignment {
    /// Given the difference between the size of the container and the size
    /// of the child (on their minor axis) return the necessary offset for
    /// this alignment.
    fn align(self, val: f64) -> f64 {
        match self {
            CrossAxisAlignment::Start => 0.0,
            // in vertical layout, baseline is equivalent to center
            CrossAxisAlignment::Center | CrossAxisAlignment::Baseline => (val / 2.0).round(),
            CrossAxisAlignment::End => val,
            CrossAxisAlignment::Fill => 0.0,
        }
    }
}

impl Data for CrossAxisAlignment {}
