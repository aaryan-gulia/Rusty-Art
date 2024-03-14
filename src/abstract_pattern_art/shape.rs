use bevy::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};
use rand::random;
use rand::Rng;


pub const NUM_SHAPES:u32 = 100;
pub enum shape_type{
    Circle,
    Square,
    Triangle
}
#[derive(Component)]
pub struct Shape{
    shape:shape_type,
    size_min:u32,
    size_max:u32,
    random_position:Vec2,
    random_color:Color,
    random_rotation:Quat,
    z_position:f32
}

pub fn art_generation_system(
    mut commands:Commands
){
    let mut rng = rand::thread_rng();
    let num_shapes = NUM_SHAPES;
    for i in 0..num_shapes{
        let shape = shape_type::Circle;
        let size_min = 1;
        let size_max = 10;
        let random_position = Vec2::new(rng.gen_range(-300.0..300.0), rng.gen_range(-300.0..300.0));
        let random_color = Color::rgb(random::<f32>(), random::<f32>(), random::<f32>());
        let random_rotation = Quat::from_rotation_z(random::<f32>());
        let z_position = i as f32;
        commands.spawn(Shape{
            shape,
            size_min,
            size_max,
            random_position,
            random_color,
            random_rotation,
            z_position
        });
    }
}

pub fn create_shape_system(
    mut commands:Commands,
    mut materials:ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query:Query<(Entity, &Shape)>,
){
    for (entity, shape) in query.iter(){
        let mut shape_material = materials.add(shape.random_color.clone());
        let mut shape_transform = Transform::from_translation(shape.random_position.clone().extend(shape.z_position));
        shape_transform.rotate(shape.random_rotation.clone());
        shape_transform.scale = Vec3::splat(shape.size_min as f32);
        commands.spawn(MaterialMesh2dBundle{
            mesh:Mesh2dHandle(meshes.add(Circle::new(10.0))),
            material:shape_material,
            transform:shape_transform,
            ..Default::default()
        });
    }
}