use bevy::asset::RenderAssetUsages;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;
use bevy::window::PrimaryWindow;

use crate::palette::ColorPalette;

#[derive(Component)]
#[require(InheritedVisibility, Visibility, GlobalTransform, Transform)]
pub struct InfiniteGrid {
    color: Color,
    spacing: f32,
}

impl Default for InfiniteGrid {
    fn default() -> Self {
        Self {
            color: ColorPalette::KIZU.fg.darker(0.5),
            spacing: 100.,
        }
    }
}

pub struct InfiniteGridPlugin;

impl Plugin for InfiniteGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_grid);
        app.add_systems(Update, move_grid.before(spawn_grid));
        app.add_systems(FixedUpdate, camera_panning);
    }
}

fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grids: Query<(Entity, &InfiniteGrid), Added<InfiniteGrid>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };

    let (width, height) = window.size().into();

    for (entity, settings) in grids.iter() {
        info!("Spawned a grid on {width}x{height} window");
        let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default());
        let mut positions = Vec::new();
        let mut indices = Vec::new();

        let spacing = settings.spacing;
        let grid_size = (width.max(height) / settings.spacing).ceil() as i32 + 1;

        // Horizontal lines
        for i in 0..=grid_size {
            let i = i as f32;
            let grid_size = grid_size as f32;
            let offset = i * spacing - (grid_size * spacing / 2.0);
            positions.push([offset, -grid_size * spacing / 2.0, 0.0]);
            positions.push([offset, grid_size * spacing / 2.0, 0.0]);
        }

        // Vertical lines
        for i in 0..=grid_size {
            let offset = i as f32 * spacing - (grid_size as f32 * spacing / 2.0);
            positions.push([-grid_size as f32 * spacing / 2.0, offset, 0.0]);
            positions.push([grid_size as f32 * spacing / 2.0, offset, 0.0]);
        }

        for i in 0..positions.len() as u32 / 2 {
            indices.push(i * 2);
            indices.push(i * 2 + 1);
        }

        info!(
            "Grid mesh: {} vertices, {} lines",
            positions.len(),
            indices.len() / 2
        );

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![[0.0, 0.0, 1.0]; positions.len()],
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; positions.len()]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_indices(Indices::U32(indices));

        commands
            .spawn((
                Mesh2d(meshes.add(mesh)),
                MeshMaterial2d(materials.add(ColorMaterial::from(settings.color))),
                Transform::default(),
            ))
            .set_parent(entity);
    }
}

fn move_grid(
    mut param_set: ParamSet<(
        Query<&Transform, (With<Camera2d>, Changed<Transform>)>,
        Query<(&mut Transform, &InfiniteGrid), With<InfiniteGrid>>,
    )>,
) {
    let Ok(camera_transform) = param_set.p0().get_single().cloned() else {
        return;
    };

    for (mut transform, settings) in param_set.p1().iter_mut() {
        let spacing = settings.spacing;
        let (x, y) = (
            camera_transform.translation.x,
            camera_transform.translation.y,
        );

        transform.translation.x = (x / spacing).round() * spacing;
        transform.translation.y = (y / spacing).round() * spacing;
    }
}

fn camera_panning(
    mut cameras: Query<&mut Transform, With<Camera2d>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    if !mouse_button_input.pressed(MouseButton::Middle) {
        return;
    }

    for event in mouse_motion.read() {
        for mut transform in cameras.iter_mut() {
            transform.translation.x -= event.delta.x;
            transform.translation.y += event.delta.y;
        }
    }
}
