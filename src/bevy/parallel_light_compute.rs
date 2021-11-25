use bevy_ecs::prelude::*;
use bevy_tasks::TaskPool;
use cgmath::*;

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(World, TaskPool);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        world.spawn_batch((0..10000).map(|_| {
            (
                Matrix4::<f32>::from_angle_x(Rad(1.2)),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));

        Self(world, TaskPool::new())
    }

    pub fn run(&mut self) {
        let mut query = self.0.query::<(&mut Position, &mut Matrix4<f32>)>();

        query.par_for_each_mut(&mut self.0, &self.1, 1024, |(mut pos, mut mat)| {
            *mat = mat.invert().unwrap();
            pos.0 = mat.transform_vector(pos.0);
        });
    }
}
