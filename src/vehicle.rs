use std::f32::consts::PI;

use bevy::prelude::*;
use ops::atan2;
use ops::cos;
use ops::sin;

use crate::palette::ColorPalette;

#[derive(Component)]
#[require(Car)]
pub struct MountedCar;

#[derive(Component, Debug)]
#[require(Visibility, Transform)]
pub struct Car {
    speed: f32,
    steer_angle: f32,
    wheel_base: f32,
}

impl Car {
    const MAX_STEER_ANGLE: f32 = PI / 3.;
    fn clamp_steer_angle(&mut self) {
        self.steer_angle = self
            .steer_angle
            .clamp(-Car::MAX_STEER_ANGLE, Car::MAX_STEER_ANGLE);
    }
}

impl Default for Car {
    fn default() -> Self {
        Self {
            speed: 0.,
            steer_angle: 0.,
            wheel_base: 60.,
        }
    }
}

pub struct VehiclePlugin;

impl Plugin for VehiclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_car);
        app.add_systems(Update, control_car);
        app.add_systems(Update, move_car.before(control_car));
    }
}

fn spawn_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    palette: Res<ColorPalette>,
    mut vehicles: Query<Entity, Added<Car>>,
) {
    for entity in vehicles.iter_mut() {
        let mesh = Mesh2d(meshes.add(Rectangle::new(75., 50.)));
        let material = MeshMaterial2d(materials.add(ColorMaterial::from(palette.green)));
        commands.entity(entity).insert(mesh).insert(material);
    }
}

fn control_car(
    camera: Single<&mut Transform, (With<Camera2d>, Without<Car>)>,
    vehicle: Single<(&mut Car, &Transform), With<MountedCar>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (mut car, transform) = vehicle.into_inner();

    if keyboard_input.pressed(KeyCode::KeyW) {
        car.speed += 2.;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        car.speed -= 2.;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        car.steer_angle += f32::to_radians(1.);
        car.clamp_steer_angle();
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        car.steer_angle -= f32::to_radians(1.);
        car.clamp_steer_angle();
    }

    if keyboard_input.pressed(KeyCode::Space) {
        let mut camera = camera.into_inner();
        camera.translation.x = transform.translation.x;
        camera.translation.y = transform.translation.y;
    }
}

fn move_car(time: Res<Time>, mut vehicles: Query<(&Car, &mut Transform), With<Car>>) {
    for (car, mut transform) in vehicles.iter_mut() {
        let car_position = transform.translation.xy();
        let car_heading = transform.rotation.to_euler(EulerRot::XYZ).2;

        let relative_wheel = car.wheel_base / 2. * Vec2::new(cos(car_heading), sin(car_heading));

        let dt = time.delta_secs();
        let steer = car_heading + car.steer_angle;
        let back_delta = car.speed * dt * Vec2::new(cos(car_heading), sin(car_heading));
        let front_delta = car.speed * dt * Vec2::new(cos(steer), sin(steer));

        let front_wheel = car_position + relative_wheel + front_delta;
        let back_wheel = car_position - relative_wheel + back_delta;

        let new_car_position = (front_wheel + back_wheel) / 2.;
        transform.translation.x = new_car_position.x;
        transform.translation.y = new_car_position.y;

        let new_car_heading = atan2(front_wheel.y - back_wheel.y, front_wheel.x - back_wheel.x);
        transform.rotation = Quat::from_rotation_z(new_car_heading);
    }
}
