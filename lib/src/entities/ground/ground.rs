extern crate ncollide;

use collision::collision_handling::{CollisionHandler, CollisionLayer};
use components::collider::Collider;
use components::rect_drawable::RectDrawable;
use components::transform::Transform;
use entities::game_entity::GameEntity;
use nalgebra::{Isometry2, Vector2};
use ncollide::shape::{Cuboid2, ShapeHandle2};
use ncollide::world::{CollisionObjectHandle, GeometricQueryType};
use specs::{Entity, World};
use types::Color;

pub struct GroundEntity(Entity);

pub struct Ground {
	position: Vector2<f32>,
	size: Vector2<f32>,
	color: Color,
}

impl Default for Ground {
	fn default() -> Self {
		Ground {
			position: Vector2::new(0.0, 0.0),
			size: Vector2::new(32.0, 32.0),
			color: Color::new(1.0, 0.0, 0.0, 1.0),
		}
	}
}

impl Ground {
	pub fn new(position: Vector2<f32>, size: Vector2<f32>, color: Color) -> Self {
		Ground {
			position,
			size,
			color,
		}
	}

	pub fn new_explicit(x: f32, y: f32, w: f32, h: f32, col_r: f32, col_g: f32, col_b: f32,
		col_a: f32) -> Ground {
		Ground {
			position: Vector2::new(x, y),
			size: Vector2::new(w, h),
			color: Color::new(col_r, col_g, col_b, col_a),
		}
	}
}

impl GameEntity for Ground {
	type Entity = GroundEntity;

	fn add_to_world(self, world: &mut World) -> Self::Entity {
		let isometry: Isometry2<f32> = Isometry2::new(self.position, 0.);
		let shape: ShapeHandle2<f32> = ShapeHandle2::new(Cuboid2::new(self.size));

		let collision_object_handle: Option<CollisionObjectHandle> = {
			let mut collision_handler = world.write_resource::<CollisionHandler>();
			let collision_group = collision_handler.get_collision_group(CollisionLayer::Normal);

			let coh = collision_handler.world.add(
				isometry,
				shape,
				collision_group,
				GeometricQueryType::Contacts(0., 0.),
				(),
			);

			Some(coh)
		};

		let entity: Entity = world
			.create_entity()
			.with(Transform::new(
				Vector2::new(self.position.x, self.position.y),
				self.size,
			))
			.with(RectDrawable::new(self.color))
			.with(Collider::new(collision_object_handle.unwrap()))
			.build();

		GroundEntity(entity)
	}
}
