use super::cross_axis_alignment::CrossAxisAlignment;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FlexParams {
    pub(super) flex: f64,
    pub(super) alignment: Option<CrossAxisAlignment>,
}

impl FlexParams {
    /// Create custom `FlexParams` with a specific `flex_factor` and an optional
    /// [`CrossAxisAlignment`].
    ///
    /// You likely only need to create these manually if you need to specify
    /// a custom alignment; if you only need to use a custom `flex_factor` you
    /// can pass an `f64` to any of the functions that take `FlexParams`.
    ///
    /// By default, the widget uses the alignment of its parent [`Flex`] container.
    pub fn new(flex: f64, alignment: impl Into<Option<CrossAxisAlignment>>) -> Self {
        #[cfg(debug)]
        if flex <= 0.0 {
            panic!("Flex value should be > 0.0. Flex given was: {}", flex);
        }

        let flex = flex.max(0.0);

        FlexParams {
            flex,
            alignment: alignment.into(),
        }
    }
}

impl From<f64> for FlexParams {
    fn from(flex: f64) -> FlexParams {
        FlexParams::new(flex, None)
    }
}
