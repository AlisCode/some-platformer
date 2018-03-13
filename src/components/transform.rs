use nalgebra::Point2;
use specs::{Component, VecStorage};

// Component that handles the position/size/rotation of a game entity
pub struct Transform {
	pub position: Point2<f32>,
	pub size: Point2<i32>,
	pub rotation: f32,
}

impl Transform {
	pub fn new(position: Point2<f32>, size: Point2<i32>, rotation: f32) -> Self {
		Transform {
			position,
			size,
			rotation,
		}
	}

	pub fn new_empty() -> Self {
		Transform {
			position: Point2::new(0., 0.),
			size: Point2::new(0, 0),
			rotation: 0.,
		}
	}
}

impl Component for Transform {
	type Storage = VecStorage<Self>;
}