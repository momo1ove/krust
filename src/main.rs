use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use noise::{NoiseFn, Perlin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Light + camera
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Procedural texture (RGBA8 sRGB)
    let w: u32 = 256;
    let h: u32 = 256;
    let perlin = Perlin::new(1);

    let mut data = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let i = ((y * w + x) * 4) as usize;

            // Noise: [-1,1] -> [0,1]
            let n = perlin.get([x as f64 / 32.0, y as f64 / 32.0]);
            let n = ((n + 1.0) * 0.5) as f32;

            // Panel stripes (pure rule-based)
            let stripe = ((x / 16) % 2) as f32 * 0.08;

            let base = (0.25 + 0.55 * n + stripe).clamp(0.0, 1.0);
            let v = (base * 255.0) as u8;

            // Cool metal-gray
            data[i + 0] = (v as f32 * 0.90) as u8; // R
            data[i + 1] = (v as f32 * 0.95) as u8; // G
            data[i + 2] = v;                       // B
            data[i + 3] = 255;                     // A
        }
    }

    let image = Image::new_fill(
        Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );

    let tex = images.add(image);

    let mat = materials.add(StandardMaterial {
        base_color_texture: Some(tex),
        perceptual_roughness: 0.85,
        metallic: 0.2,
        ..default()
    });

    // Floor
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(10.0, 0.2, 10.0))),
        material: mat.clone(),
        transform: Transform::from_xyz(0.0, -0.1, 0.0),
        ..default()
    });

    // Back wall
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(10.0, 3.0, 0.2))),
        material: mat.clone(),
        transform: Transform::from_xyz(0.0, 1.5, -5.0),
        ..default()
    });

    // A couple of boxes
    for (x, z) in [(-2.0, -1.0), (2.0, -2.0), (0.0, -3.0)] {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
            material: mat.clone(),
            transform: Transform::from_xyz(x, 0.5, z),
            ..default()
        });
    }
}
