use cgmath::*;
use rayon::prelude::*;
use specs::{prelude::*, ParJoin};
use specs_derive::*;

#[derive(Copy, Clone, Component)]
#[storage(VecStorage)]
struct Transform(Matrix4<f32>);
#[derive(Copy, Clone, Component)]
#[storage(VecStorage)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone, Component)]
#[storage(VecStorage)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone, Component)]
#[storage(VecStorage)]
struct Velocity(Vector3<f32>);

struct ParallelLightComputeSystem;

impl<'a> System<'a> for ParallelLightComputeSystem {
    type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Transform>);

    fn run(&mut self, (mut pos_store, mut mat_store): Self::SystemData) {
        use cgmath::Transform;
        (&mut pos_store, &mut mat_store)
            .par_join()
            .for_each(|(pos, mat)| {
                mat.0 = mat.0.invert().unwrap();
                pos.0 = mat.0.transform_vector(pos.0);
            });
    }
}

pub struct Benchmark(World, ParallelLightComputeSystem);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        world.register::<Transform>();
        world.register::<Position>();
        world.register::<Rotation>();
        world.register::<Velocity>();
        (0..10000).for_each(|_| {
            world
                .create_entity()
                .with(Transform(Matrix4::<f32>::from_angle_x(Rad(1.2))))
                .with(Position(Vector3::unit_x()))
                .with(Rotation(Vector3::unit_x()))
                .with(Velocity(Vector3::unit_x()))
                .build();
        });

        Self(world, ParallelLightComputeSystem)
    }

    pub fn run(&mut self) {
        self.1.run_now(&self.0);
    }
}
