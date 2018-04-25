use collision::collision_handling::{CollisionHandler, CollisionLayer};
use components::collider::Collider;
use components::moving::{GravityAffected, Moving};
use components::rect_drawable::RectDrawable;
use components::transform::Transform;
use entities::game_entity::GameEntity;
use nalgebra::geometry::Isometry2;
use nalgebra::Vector2;
use ncollide::shape::{Cuboid2, ShapeHandle2};
use ncollide::world::{CollisionObjectHandle, GeometricQueryType};
use specs::{Entity, World};
use types::Color;

pub struct PlayerEntity(Entity);

pub struct Player {
	position: Vector2<f32>,
	size: Vector2<f32>,
	color: Color,
}

impl Default for Player {
	fn default() -> Self {
		Player {
			position: Vector2::new(200.0, 100.0),
			size: Vector2::new(32.0, 32.0),
			color: Color::new(0.0, 1.0, 0.0, 1.0),
		}
	}
}

impl Player {
	pub fn new(position: Vector2<f32>, size: Vector2<f32>, color: Color) -> Self {
		Player {
			position,
			size,
			color,
		}
	}
}

impl GameEntity for Player {
	type Entity = PlayerEntity;

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
			.with(Moving::new())
			.with(GravityAffected::new())
			.with(Collider::new(collision_object_handle.unwrap()))
			.build();

		PlayerEntity(entity)
	}
}
