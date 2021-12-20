// This is pretty much all stolen from linebender/druid.
// They have the license on their github repo.
// I didn't include it here because it might change-
// just go ask them what the license says and don't bother me.

use crate::{box_constraints::BoxConstraints, Data, Size, Widget};

use super::WidgetPod;

mod axis;
mod child;
mod cross_axis_alignment;
mod flex_params;
mod main_axis_alignment;
mod spacing;
use axis::*;
use child::*;
use cross_axis_alignment::*;
use flex_params::*;
use main_axis_alignment::*;
use spacing::*;

pub struct Flex<T> {
	direction: Axis,
	cross_alignment: CrossAxisAlignment,
	main_alignment: MainAxisAlignment,
	fill_major_axis: bool,
	children: Vec<Child<T>>,
}

impl<T: Data> Flex<T> {
	/// Create a new Flex oriented along the provided axis.
	pub fn for_axis(axis: Axis) -> Self {
		Flex {
			direction: axis,
			children: Vec::new(),
			cross_alignment: CrossAxisAlignment::Center,
			main_alignment: MainAxisAlignment::Start,
			fill_major_axis: false,
		}
	}

	/// Create a new horizontal stack.
	///
	/// The child widgets are laid out horizontally, from left to right.
	///
	pub fn row() -> Self {
		Self::for_axis(Axis::Horizontal)
	}

	/// Create a new vertical stack.
	///
	/// The child widgets are laid out vertically, from top to bottom.
	pub fn column() -> Self {
		Self::for_axis(Axis::Vertical)
	}

	/// Builder-style method for specifying the childrens' [`CrossAxisAlignment`].
	///
	/// [`CrossAxisAlignment`]: enum.CrossAxisAlignment.html
	pub fn cross_axis_alignment(mut self, alignment: CrossAxisAlignment) -> Self {
		self.cross_alignment = alignment;
		self
	}

	/// Builder-style method for specifying the childrens' [`MainAxisAlignment`].
	///
	/// [`MainAxisAlignment`]: enum.MainAxisAlignment.html
	pub fn main_axis_alignment(mut self, alignment: MainAxisAlignment) -> Self {
		self.main_alignment = alignment;
		self
	}

	/// Builder-style method for setting whether the container must expand
	/// to fill the available space on its main axis.
	///
	/// If any children have flex then this container will expand to fill all
	/// available space on its main axis; But if no children are flex,
	/// this flag determines whether or not the container should shrink to fit,
	/// or must expand to fill.
	///
	/// If it expands, and there is extra space left over, that space is
	/// distributed in accordance with the [`MainAxisAlignment`].
	///
	/// The default value is `false`.
	///
	/// [`MainAxisAlignment`]: enum.MainAxisAlignment.html
	pub fn must_fill_main_axis(mut self, fill: bool) -> Self {
		self.fill_major_axis = fill;
		self
	}

	/// Builder-style variant of `add_child`.
	///
	/// Convenient for assembling a group of widgets in a single expression.
	pub fn with_child(mut self, child: impl Widget<T> + 'static) -> Self {
		self.add_child(child);
		self
	}

	/// Builder-style method to add a flexible child to the container.
	///
	/// This method is used when you need more control over the behaviour
	/// of the widget you are adding. In the general case, this likely
	/// means giving that child a 'flex factor', but it could also mean
	/// giving the child a custom [`CrossAxisAlignment`], or a combination
	/// of the two.
	///
	/// This function takes a child widget and [`FlexParams`]; importantly
	/// you can pass in a float as your [`FlexParams`] in most cases.
	///
	/// For the non-builder varient, see [`add_flex_child`].
	///
	/// # Examples
	///
	/// ```
	/// use druid::widget::{Flex, FlexParams, Label, Slider, CrossAxisAlignment};
	///
	/// let my_row = Flex::row()
	///     .with_flex_child(Slider::new(), 1.0)
	///     .with_flex_child(Slider::new(), FlexParams::new(1.0, CrossAxisAlignment::End));
	/// ```
	///
	/// [`FlexParams`]: struct.FlexParams.html
	/// [`add_flex_child`]: #method.add_flex_child
	/// [`CrossAxisAlignment`]: enum.CrossAxisAlignment.html
	pub fn with_flex_child(
		mut self,
		child: impl Widget<T> + 'static,
		params: impl Into<FlexParams>,
	) -> Self {
		self.add_flex_child(child, params);
		self
	}

	/// Builder-style method to add a spacer widget with a standard size.
	///
	/// The actual value of this spacer depends on whether this container is
	/// a row or column, as well as theme settings.
	pub fn with_default_spacer(mut self) -> Self {
		self.add_default_spacer();
		self
	}

	/// Builder-style method for adding a fixed-size spacer to the container.
	///
	/// If you are laying out standard controls in this container, you should
	/// generally prefer to use [`add_default_spacer`].
	///
	/// [`add_default_spacer`]: #method.add_default_spacer
	pub fn with_spacer(mut self, len: usize) -> Self {
		self.add_spacer(len);
		self
	}

	/// Builder-style method for adding a `flex` spacer to the container.
	pub fn with_flex_spacer(mut self, flex: f64) -> Self {
		self.add_flex_spacer(flex);
		self
	}

	/// Set the childrens' [`CrossAxisAlignment`].
	///
	/// [`CrossAxisAlignment`]: enum.CrossAxisAlignment.html
	pub fn set_cross_axis_alignment(&mut self, alignment: CrossAxisAlignment) {
		self.cross_alignment = alignment;
	}

	/// Set the childrens' [`MainAxisAlignment`].
	///
	/// [`MainAxisAlignment`]: enum.MainAxisAlignment.html
	pub fn set_main_axis_alignment(&mut self, alignment: MainAxisAlignment) {
		self.main_alignment = alignment;
	}

	/// Set whether the container must expand to fill the available space on
	/// its main axis.
	pub fn set_must_fill_main_axis(&mut self, fill: bool) {
		self.fill_major_axis = fill;
	}

	/// Add a non-flex child widget.
	///
	/// See also [`with_child`].
	///
	/// [`with_child`]: Flex::with_child
	pub fn add_child(&mut self, child: impl Widget<T> + 'static) {
		let child = Child::Fixed {
			widget: WidgetPod::new(Box::new(child)),
			alignment: None,
		};
		self.children.push(child);
	}

	/// Add a flexible child widget.
	///
	/// This method is used when you need more control over the behaviour
	/// of the widget you are adding. In the general case, this likely
	/// means giving that child a 'flex factor', but it could also mean
	/// giving the child a custom [`CrossAxisAlignment`], or a combination
	/// of the two.
	///
	/// This function takes a child widget and [`FlexParams`]; importantly
	/// you can pass in a float as your [`FlexParams`] in most cases.
	///
	/// For the builder-style varient, see [`with_flex_child`].
	///
	/// # Examples
	///
	/// ```
	/// use druid::widget::{Flex, FlexParams, Label, Slider, CrossAxisAlignment};
	///
	/// let mut my_row = Flex::row();
	/// my_row.add_flex_child(Slider::new(), 1.0);
	/// my_row.add_flex_child(Slider::new(), FlexParams::new(1.0, CrossAxisAlignment::End));
	/// ```
	///
	/// [`with_flex_child`]: Flex::with_flex_child
	pub fn add_flex_child(
		&mut self,
		child: impl Widget<T> + 'static,
		params: impl Into<FlexParams>,
	) {
		let params = params.into();
		let child = if params.flex > 0.0 {
			Child::Flex {
				widget: WidgetPod::new(Box::new(child)),
				alignment: params.alignment,
				flex: params.flex,
			}
		} else {
			// tracing::warn!("Flex value should be > 0.0. To add a non-flex child use the add_child or with_child methods.\nSee the docs for more information: https://docs.rs/druid/0.7.0/druid/widget/struct.Flex.html");
			Child::Fixed {
				widget: WidgetPod::new(Box::new(child)),
				alignment: None,
			}
		};
		self.children.push(child);
	}

	/// Add a spacer widget with a standard size.
	///
	/// The actual value of this spacer depends on whether this container is
	/// a row or column, as well as theme settings.
	pub fn add_default_spacer(&mut self) {
		let key = match self.direction {
			Axis::Vertical => crate::theme::WIDGET_PADDING_VERTICAL,
			Axis::Horizontal => crate::theme::WIDGET_PADDING_HORIZONTAL,
		};
		self.add_spacer(key);
	}

	/// Add an empty spacer widget with the given size.
	///
	/// If you are laying out standard controls in this container, you should
	/// generally prefer to use [`add_default_spacer`].
	///
	/// [`add_default_spacer`]: Flex::add_default_spacer
	pub fn add_spacer(&mut self, len: usize) {
		let new_child = Child::FixedSpacer(len, 0);
		self.children.push(new_child);
	}

	/// Add an empty spacer widget with a specific `flex` factor.
	pub fn add_flex_spacer(&mut self, flex: f64) {
		let flex = if flex >= 0.0 {
			flex
		} else {
			debug_assert!(
				flex >= 0.0,
				"flex value for space should be greater than equal to 0, received: {}",
				flex
			);
			// tracing::warn!("Provided flex value was less than 0: {}", flex);
			0.0
		};
		let new_child = Child::FlexedSpacer(flex, 0.0);
		self.children.push(new_child);
	}
}

impl<T: Data> Widget<T> for Flex<T> {
	fn event(&mut self, data: ) {
	    for child in self.children.iter_mut().filter_map(|x| x.widget_mut()) {
	        child.event(ctx, event, data, env);
	    }
	}

	// #[instrument(name = "Flex", level = "trace", skip(self, ctx, event, data, env))]
	// fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
	//     for child in self.children.iter_mut().filter_map(|x| x.widget_mut()) {
	//         child.lifecycle(ctx, event, data, env);
	//     }
	// }

	fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
	    for child in self.children.iter_mut() {
	        match child {
	            Child::Fixed { widget, .. } | Child::Flex { widget, .. } => {
	                widget.update(ctx, data, env)
	            }
	            Child::FixedSpacer(key_or_val, _) if ctx.env_key_changed(key_or_val) => {
	                ctx.request_layout()
	            }
	            _ => {}
	        }
	    }
	}

	fn layout(&mut self, bc: &BoxConstraints) -> Size {
		// bc.debug_check("Flex");
		// we loosen our constraints when passing to children.
		let loosened_bc = bc.loosen();

		// minor-axis values for all children
		let mut minor = self.direction.minor(bc.min());
		// these two are calculated but only used if we're baseline aligned
		let mut max_above_baseline = 0;
		let mut max_below_baseline = 0;
		let mut any_use_baseline = self.cross_alignment == CrossAxisAlignment::Baseline;

		// Measure non-flex children.
		let mut major_non_flex = 0;
		let mut flex_sum = 0.0;
		for child in &mut self.children {
			match child {
				Child::Fixed { widget, alignment } => {
					any_use_baseline &= *alignment == Some(CrossAxisAlignment::Baseline);

					let child_bc = self.direction.constraints(&loosened_bc, 0, usize::MAX);
					let child_size = widget.layout(&child_bc);
					let baseline_offset = 0;

					major_non_flex += self.direction.major(child_size);
					minor = minor.max(self.direction.minor(child_size));
					max_above_baseline = max_above_baseline.max(child_size.height - baseline_offset);
					max_below_baseline = max_below_baseline.max(baseline_offset);
				}
				Child::FixedSpacer(kv, calculated_siz) => {
					*calculated_siz = *kv;
					major_non_flex += *calculated_siz;
				}
				Child::Flex { flex, .. } | Child::FlexedSpacer(flex, _) => flex_sum += *flex,
			}
		}

		let total_major = self.direction.major(bc.max());
		let remaining = (total_major - major_non_flex).max(0.0);
		let mut remainder: f64 = 0.0;

		let mut major_flex: f64 = 0.0;
		let px_per_flex = remaining / flex_sum;
		// Measure flex children.
		for child in &mut self.children {
			match child {
				Child::Flex { widget, flex, .. } => {
					let desired_major = (*flex) * px_per_flex + remainder;
					let actual_major = desired_major.round();
					remainder = desired_major - actual_major;

					let child_bc = self.direction.constraints(&loosened_bc, 0.0, actual_major);
					let child_size = widget.layout(ctx, &child_bc, data, env);
					let baseline_offset = widget.baseline_offset();

					major_flex += self.direction.major(child_size).expand();
					minor = minor.max(self.direction.minor(child_size).expand());
					max_above_baseline = max_above_baseline.max(child_size.height - baseline_offset);
					max_below_baseline = max_below_baseline.max(baseline_offset);
				}
				Child::FlexedSpacer(flex, calculated_size) => {
					let desired_major = (*flex) * px_per_flex + remainder;
					*calculated_size = desired_major.round();
					remainder = desired_major - *calculated_size;
					major_flex += *calculated_size;
				}
				_ => {}
			}
		}

		// figure out if we have extra space on major axis, and if so how to use it
		let extra = if self.fill_major_axis {
			(remaining - major_flex).max(0.0)
		} else {
			// if we are *not* expected to fill our available space this usually
			// means we don't have any extra, unless dictated by our constraints.
			(self.direction.major(bc.min()) - (major_non_flex + major_flex)).max(0.0)
		};

		let mut spacing = Spacing::new(self.main_alignment, extra, self.children.len());

		// the actual size needed to tightly fit the children on the minor axis.
		// Unlike the 'minor' var, this ignores the incoming constraints.
		let minor_dim = match self.direction {
			Axis::Horizontal if any_use_baseline => max_below_baseline + max_above_baseline,
			_ => minor,
		};

		let extra_height = minor - minor_dim.min(minor);

		let mut major = spacing.next().unwrap_or(0.);
		let mut child_paint_rect = Rect::ZERO;

		for child in &mut self.children {
			match child {
				Child::Fixed { widget, alignment }
				| Child::Flex {
					widget, alignment, ..
				} => {
					let child_size = widget.layout_rect().size();
					let alignment = alignment.unwrap_or(self.cross_alignment);
					let child_minor_offset = match alignment {
						// This will ignore baseline alignment if it is overridden on children,
						// but is not the default for the container. Is this okay?
						CrossAxisAlignment::Baseline if matches!(self.direction, Axis::Horizontal) => {
							let child_baseline = widget.baseline_offset();
							let child_above_baseline = child_size.height - child_baseline;
							extra_height + (max_above_baseline - child_above_baseline)
						}
						CrossAxisAlignment::Fill => {
							let fill_size: Size = self
								.direction
								.pack(self.direction.major(child_size), minor_dim)
								.into();
							let child_bc = BoxConstraints::tight(fill_size);
							widget.layout(ctx, &child_bc, data, env);
							0.0
						}
						_ => {
							let extra_minor = minor_dim - self.direction.minor(child_size);
							alignment.align(extra_minor)
						}
					};

					let child_pos: Point = self.direction.pack(major, child_minor_offset).into();
					widget.set_origin(ctx, data, env, child_pos);
					child_paint_rect = child_paint_rect.union(widget.paint_rect());
					major += self.direction.major(child_size).expand();
					major += spacing.next().unwrap_or(0.);
				}
				Child::FlexedSpacer(_, calculated_size) | Child::FixedSpacer(_, calculated_size) => {
					major += *calculated_size;
				}
			}
		}

		if flex_sum > 0.0 && total_major.is_infinite() {
			tracing::warn!("A child of Flex is flex, but Flex is unbounded.")
		}

		if flex_sum > 0.0 {
			major = total_major;
		}

		let my_size: Size = self.direction.pack(major, minor_dim).into();

		// if we don't have to fill the main axis, we loosen that axis before constraining
		let my_size = if !self.fill_major_axis {
			let max_major = self.direction.major(bc.max());
			self
				.direction
				.constraints(bc, 0.0, max_major)
				.constrain(my_size)
		} else {
			bc.constrain(my_size)
		};

		let my_bounds = Rect::ZERO.with_size(my_size);
		let insets = child_paint_rect - my_bounds;
		ctx.set_paint_insets(insets);

		let baseline_offset = match self.direction {
			Axis::Horizontal => max_below_baseline,
			Axis::Vertical => (&self.children)
				.last()
				.map(|last| {
					let child = last.widget();
					if let Some(widget) = child {
						let child_bl = widget.baseline_offset();
						let child_max_y = widget.layout_rect().max_y();
						let extra_bottom_padding = my_size.height - child_max_y;
						child_bl + extra_bottom_padding
					} else {
						0.0
					}
				})
				.unwrap_or(0.0),
		};

		ctx.set_baseline_offset(baseline_offset);
		trace!(
			"Computed layout: size={}, baseline_offset={}",
			my_size,
			baseline_offset
		);
		my_size
	}

	// #[instrument(name = "Flex", level = "trace", skip(self, ctx, data, env))]
	fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
		for child in self.children.iter_mut().filter_map(|x| x.widget_mut()) {
			child.paint(ctx, data, env);
		}

		// paint the baseline if we're debugging layout
		if env.get(Env::DEBUG_PAINT) && ctx.widget_state.baseline_offset != 0.0 {
			let color = env.get_debug_color(ctx.widget_id().to_raw());
			let my_baseline = ctx.size().height - ctx.widget_state.baseline_offset;
			let line = crate::kurbo::Line::new((0.0, my_baseline), (ctx.size().width, my_baseline));
			let stroke_style = crate::piet::StrokeStyle::new().dash_pattern(&[4.0, 4.0]);
			ctx.stroke_styled(line, &color, 1.0, &stroke_style);
		}
	}

	// fn debug_state(&self, data: &T) -> DebugState {
	//     let children_state = self
	//         .children
	//         .iter()
	//         .map(|child| {
	//             let child_widget_pod = child.widget()?;
	//             Some(child_widget_pod.widget().debug_state(data))
	//         })
	//         .flatten()
	//         .collect();
	//     DebugState {
	//         display_name: self.short_type_name().to_string(),
	//         children: children_state,
	//         ..Default::default()
	//     }
	// }
}
