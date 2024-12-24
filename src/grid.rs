use bevy::asset::RenderAssetUsages;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;
use bevy::window::PrimaryWindow;

use crate::palette::ColorPalette;

#[derive(Component)]
#[require(Visibility, Transform)]
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
    grid: Single<(Entity, &InfiniteGrid, &mut Transform), Added<InfiniteGrid>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let window = window.into_inner();
    let (width, height) = window.size().into();

    let (entity, settings, mut transform) = grid.into_inner();
    info!("Spawned a grid on {width}x{height} window");

    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default());
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    let spacing = settings.spacing;
    let grid_size = (width.max(height) / settings.spacing).ceil() as i32 + 1;

    transform.translation.z = -1.;

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

fn move_grid(
    camera: Single<&Transform, (With<Camera2d>, Changed<Transform>, Without<InfiniteGrid>)>,
    grid: Single<(&mut Transform, &InfiniteGrid), With<InfiniteGrid>>,
) {
    let camera = camera.into_inner();
    let (mut transform, settings) = grid.into_inner();

    let spacing = settings.spacing;
    let (x, y) = (camera.translation.x, camera.translation.y);

    transform.translation.x = (x / spacing).round() * spacing;
    transform.translation.y = (y / spacing).round() * spacing;
}

fn camera_panning(
    camera: Single<&mut Transform, With<Camera2d>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    if !mouse_button_input.pressed(MouseButton::Middle) {
        return;
    }

    let mut camera = camera.into_inner();
    for event in mouse_motion.read() {
        camera.translation.x -= event.delta.x;
        camera.translation.y += event.delta.y;
    }
}
