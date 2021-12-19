use crate::widget::{Widget, WidgetPod};

use super::cross_axis_alignment::CrossAxisAlignment;

pub enum Child<T> {
    Fixed {
        widget: WidgetPod<T, Box<dyn Widget<T>>>,
        alignment: Option<CrossAxisAlignment>,
    },
    Flex {
        widget: WidgetPod<T, Box<dyn Widget<T>>>,
        alignment: Option<CrossAxisAlignment>,
        flex: f64,
    },
    FixedSpacer(
        // KeyOrValue<f64>,
        usize,
        f64,
    ),
    FlexedSpacer(f64, f64),
}

impl<T> Child<T> {
    fn widget_mut(&mut self) -> Option<&mut WidgetPod<T, Box<dyn Widget<T>>>> {
        match self {
            Child::Fixed { widget, .. } | Child::Flex { widget, .. } => Some(widget),
            _ => None,
        }
    }
    fn widget(&self) -> Option<&WidgetPod<T, Box<dyn Widget<T>>>> {
        match self {
            Child::Fixed { widget, .. } | Child::Flex { widget, .. } => Some(widget),
            _ => None,
        }
    }
}
