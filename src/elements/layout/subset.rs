use crate::{ Drawable, DrawableData, DrawBuffer };

pub struct SubSet {
	data_set:DrawableData
}
impl SubSet {

	/// Create a new SubSet.
	pub fn new(x:usize, y:usize, w:usize, h:usize, children:Vec<&dyn Drawable>) -> SubSet {
		SubSet {
			data_set: DrawableData::new::<usize>(vec![("x", x), ("y", y), ("w", w), ("h", h)], children)
		}
	}
}
impl Drawable for SubSet {

	/// Get the name of the components.
	fn name(&self) -> String {
		String::from("subset")
	}

	/// Create a boxed version of the instance.
	fn boxify(&self) -> Box<dyn Drawable> {
		Box::new(SubSet {
			data_set: self.data_set.clone()
		})
	}

	/// Get the instances relative position as a xywh array.
	fn position(&self) -> [usize; 4] {
		[
			0,
			0,
			self.data_set().get_setting_value_or::<usize>("w", 0),
			self.data_set().get_setting_value_or::<usize>("h", 0)
		]
	}
	
	/// Draw the core of this specific instance type.
	fn draw(&mut self) -> DrawBuffer {
		self.draw_children().take(
			self.data_set().get_setting_value_or::<usize>("x", 0),
			self.data_set().get_setting_value_or::<usize>("y", 0),
			self.data_set().get_setting_value_or::<usize>("w", 0),
			self.data_set().get_setting_value_or::<usize>("h", 0)
		)
	}

	

	/* DATA SET METHODS */

	/// Get a reference to the instances Data object.
	fn data_set(&self) -> &DrawableData {
		&self.data_set
	}

	/// Get a mutable reference to the instances Data object.
	fn data_set_mut(&mut self) -> &mut DrawableData {
		&mut self.data_set
	}
}