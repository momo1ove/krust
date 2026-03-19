use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use noise::{NoiseFn, Perlin};

// FPS Camera plugin
#[derive(Component)]
struct FpsCamera {
    yaw: f32,
    pitch: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (fps_movement, fps_look, lock_cursor))
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Procedural texture (RGBA8 sRGB)
    let w: u32 = 256;
    let h: u32 = 256;
    let perlin = Perlin::new(1);

    let mut data = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let i = ((y * w + x) * 4) as usize;
            let n = perlin.get([x as f64 / 32.0, y as f64 / 32.0]);
            let n = ((n + 1.0) * 0.5) as f32;
            let stripe = ((x / 16) % 2) as f32 * 0.08;
            let base = (0.25 + 0.55 * n + stripe).clamp(0.0, 1.0);
            let v = (base * 255.0) as u8;
            data[i + 0] = (v as f32 * 0.90) as u8;
            data[i + 1] = (v as f32 * 0.95) as u8;
            data[i + 2] = v;
            data[i + 3] = 255;
        }
    }

    let image = Image::new_fill(
        Extent3d { width: w, height: h, depth_or_array_layers: 1 },
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

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.4, 0.4, 0.5),
        brightness: 0.5,
    });

    // Point light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 4.0, 0.0),
        ..default()
    });

    // FPS Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.7, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FpsCamera { yaw: 0.0, pitch: 0.0 },
    ));

    // Floor (20x20)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(20.0, 0.2, 20.0))),
        material: mat.clone(),
        transform: Transform::from_xyz(0.0, -0.1, 0.0),
        ..default()
    });

    // Walls (room)
    for (sx, sz, sw, sd) in [
        (0.0, -10.0, 20.0, 0.2),  // back
        (0.0, 10.0, 20.0, 0.2),   // front
        (-10.0, 0.0, 0.2, 20.0),  // left
        (10.0, 0.0, 0.2, 20.0),   // right
    ] {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(sw, 3.0, sd))),
            material: mat.clone(),
            transform: Transform::from_xyz(sx, 1.5, sz),
            ..default()
        });
    }

    // Random boxes to shoot at
    let mut rng = rand::thread_rng();
    for _ in 0..8 {
        let x = rand::Rng::gen_range(&mut rng, -8.0..8.0);
        let z = rand::Rng::gen_range(&mut rng, -8.0..8.0);
        let scale = rand::Rng::gen_range(&mut rng, 0.5..1.5);
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(scale, scale, scale))),
            material: mat.clone(),
            transform: Transform::from_xyz(x, scale / 2.0, z),
            ..default()
        });
    }
}

fn lock_cursor(mut windows: Query<&mut Window>) {
    for mut window in &mut windows {
        window.cursor.visible = false;
        window.cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
    }
}

fn fps_look(
    mut events: EventReader<bevy::input::mouse::MouseMotion>,
    mut cameras: Query<(&mut Transform, &mut FpsCamera)>,
) {
    let mut look_delta = Vec2::ZERO;
    for event in events.read() {
        look_delta += event.delta;
    }

    if look_delta == Vec2::ZERO {
        return;
    }

    let sensitivity = 0.002;
    for (mut transform, mut camera) in cameras.iter_mut() {
        camera.yaw -= look_delta.x * sensitivity;
        camera.pitch -= look_delta.y * sensitivity;
        camera.pitch = camera.pitch.clamp(-1.5, 1.5);

        transform.rotation = Quat::from_rotation_y(camera.yaw)
            * Quat::from_rotation_x(camera.pitch);
    }
}

fn fps_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut cameras: Query<(&mut Transform, &FpsCamera)>,
) {
    let speed = 5.0;
    let movement = time.delta_seconds() * speed;

    let forward = Vec3::new(0.0, 0.0, -1.0);
    let right = Vec3::new(1.0, 0.0, 0.0);

    let mut input_dir = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        input_dir += forward;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        input_dir -= forward;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        input_dir -= right;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        input_dir += right;
    }

    if input_dir == Vec3::ZERO {
        return;
    }

    for (mut transform, camera) in cameras.iter_mut() {
        let yaw = Quat::from_rotation_y(camera.yaw);
        let move_dir = yaw * input_dir.normalize_or_zero();
        
        transform.translation += move_dir * movement;
        
        // Keep on ground
        transform.translation.y = 1.7;
    }
}
