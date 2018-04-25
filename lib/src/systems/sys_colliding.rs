use collision::collision_handling::CollisionHandler;
use components::collider::Collider;
use components::transform::Transform;
use specs::{FetchMut, Join, ReadStorage, System};

pub struct SysCollide {}

// A system updating the ncollide position of an item that has both moving and transform components,
// thus allowing for collision detection with ncollide
impl<'a> System<'a> for SysCollide {
	type SystemData = (
		ReadStorage<'a, Transform>,
		ReadStorage<'a, Collider>,
		FetchMut<'a, CollisionHandler>,
	);

	// Updates the CollisionHandler based on the transform's position
	fn run(&mut self, (transform, collider, mut collision_handler): Self::SystemData) {
		// Updates the positions of items based on their transform
		for (tr, col) in (&transform, &collider).join() {
			collision_handler.world.set_position(
				col.collision_object_handle,
				tr.isometry,
			);
		}

		// Computes the collisions
		collision_handler.world.update();

		// Handle collisions ?
		info!("{}",
			  collision_handler.world.contacts().into_iter().count() > 0
		);
	}
}
