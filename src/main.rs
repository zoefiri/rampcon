use bevy::{
    math::{vec3, vec3a, Vec3, Vec3A},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_egui::EguiPlugin;
use colorgen::palette_models::setup_model_resources;
use expr::parse::setup_expr_list;
use std::f32::consts::PI;
use ui::egui::ui_example_system;

mod colorgen;
mod expr;
mod ui;

enum CollissionState {
    Intersecting,
    Encroaching,
    Uncontacting,
}

#[derive(Component)]
struct MeshComponent {
    pub mesh: Mesh,
    pub velocity: Vec3,
    pub transform: Vec3,
    pub calc_v: fn(f32, Vec3A) -> bevy::math::Vec3A,
}

#[derive(Component)]
struct FloorMesh;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut window_query: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let display_font: Handle<Font> = asset_server.load("GermaniaOne-Regular.ttf");

    // main UI node
    commands
        // spawn root UI node
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(25.0),
                height: Val::Percent(75.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        })
        // title and rest of UI
        .with_children(|parent| {
            parent.spawn(ui::title::text_box()).with_children(|parent| {
                parent.spawn(ui::title::text(&display_font));
            });

            // test field
            parent
                .spawn(ui::field::InputField::default_field_components(
                    bevy::math::vec2(1200.0, 100.0),
                ))
                .with_children(|parent| {
                    parent.spawn(ui::field::InputField::field_display_child(display_font));
                });

            // + btn for fields
            parent.spawn(ui::button::new_btn());
        });

    let dbg_material = materials.add(StandardMaterial {
        // base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let user_meshes: [Mesh; 1] = [shape::Cube::default().into()];
    // let num_shapes = shapes.len();

    for (_i, mesh) in user_meshes.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(mesh.clone()),
                material: dbg_material.clone(),
                transform: Transform::from_xyz(1.0, 0.5, 1.0)
                    .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            MeshComponent {
                mesh,
                velocity: vec3(0.0, 0.0, 0.0),
                calc_v: |time: f32, v_factor: Vec3A| vec3a(0.0, (v_factor.y * time).sin(), 0.0),
                transform: vec3(1.0, -1.0, 1.0),
            },
        ));
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    let ground_plane_mesh: Mesh = shape::Plane::from_size(50.0).into();
    ground_plane_mesh.compute_aabb();

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(ground_plane_mesh.clone()),
            material: materials.add(Color::SILVER.into()),
            ..default()
        },
        MeshComponent {
            mesh: ground_plane_mesh,
            velocity: vec3(0.0, 2.0, 0.0),
            calc_v: |_, _| vec3a(0.0, 1.0, 0.0),
            transform: vec3(0.0, 0.0, 0.0),
        },
        FloorMesh,
    ));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6.0, 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });

    // UI focus resource
    commands.insert_resource(ui::field::Focused(None));

    // enable IME
    window_query.single_mut().ime_enabled = true;
}

// iterate each mesh comopnent and alter its velocity, before doing so determine if it encroaches
// any other meshes, accumulate the encroachment vectors, and use (encroachment/limit) as the
// domain of the velocity function.
fn process_physics(mut mesh_query: Query<(&mut MeshComponent, &Transform)>, time: Res<Time>) {
    let limit_base = vec3a(1.0, 1.5, 1.0);

    let mut mesh_iter_comb = mesh_query.iter_combinations_mut();
    while let Some([(mut mesh_a, transform_a), (mut mesh_b, transform_b)]) =
        mesh_iter_comb.fetch_next()
    {
        let mut aabb_a: bevy::render::primitives::Aabb = mesh_a.mesh.compute_aabb().unwrap();
        let mut aabb_b: bevy::render::primitives::Aabb = mesh_b.mesh.compute_aabb().unwrap();
        aabb_a.center += <bevy::prelude::Vec3 as Into<Vec3A>>::into(transform_a.translation);
        aabb_b.center += <bevy::prelude::Vec3 as Into<Vec3A>>::into(transform_b.translation);

        let (encroachment, collission_state) = aabb_encroaching_vec(aabb_a, aabb_b, limit_base);
        let mut reflect = 1.0;
        let v_factor = match collission_state {
            CollissionState::Uncontacting => vec3a(1.0, 1.0, 1.0),
            CollissionState::Encroaching => {
                let original_velocity_dot = mesh_a.velocity.dot(mesh_b.velocity);
                // println!(
                //     "encroaching!!! vdot is: {}, enchraoch.len: {}, limit.len: {}",
                //     original_velocity_dot,
                //     encroachment.length(),
                //     limit_base.length()
                // );
                if original_velocity_dot <= 0.0 && encroachment.length() > limit_base.length() / 2.0
                {
                    println!("encroach reflect conditions met...");
                    reflect = -1.0;
                }
                encroachment / limit_base
            } // converges to 0.00...1
            CollissionState::Intersecting => {
                let original_velocity_dot = mesh_a.velocity.dot(mesh_b.velocity);
                // println!("intersecting!!! vdot is: {}", original_velocity_dot);
                if original_velocity_dot <= 0.0 {
                    vec3a(0.0, 0.0, 0.0)
                } else {
                    reflect = -1.0;
                    vec3a(1.5, 1.5, 1.5)
                }
            }
        };

        mesh_a.velocity = (mesh_a.calc_v)(time.elapsed_seconds(), v_factor).into();
        mesh_b.velocity = (mesh_b.calc_v)(time.elapsed_seconds(), v_factor).into();
        mesh_a.velocity *= reflect;
        mesh_b.velocity *= reflect;

        // println!(
        //     "mesh velocities: a:{}, b:{}, v_factor:{}",
        //     mesh_a.velocity, mesh_b.velocity, v_factor
        // );
    }
}

fn apply_physics(
    mut mesh_query: Query<
        (&mut Transform, &mut MeshComponent),
        (Without<FloorMesh>, Without<FloorMesh>),
    >,
    time: Res<Time>,
) {
    for (mut transform, mut mesh_component) in &mut mesh_query {
        transform.translation += time.delta_seconds() * mesh_component.velocity;
        mesh_component.transform = transform.translation;
        // println!(
        //     "transformed: t.{} * v.{} = {}",
        //     time.delta_seconds(),
        //     mesh_component.velocity,
        //     transform.translation
        // );
    }
}

// aabb_encroaching_vec: once the two aabb encroach each other on an axis within the distance
// `limit` return the distance vector for how far it has encroached. Intersection is occurring if
// all elements of the return vector are inter
//
//
// m    extant A                 extant B
// e      '|'                      '|'
// s      ,|,,,,                   ,|,,,
// h  ,,,|,,,,,,|                 |,,,,,|,,,,,
//   |   x------|-----------------|------x    | mesh B
// A  ''''''''''                   '''''''''''
//  4 intersect calc. by (12 len. - 6 - 10)
//            | -4 <= 0 so intersects,      8    gap of 3 (18 len - 10 - 5) <= 8 limit so they
//            | by 4.                       |    "intersect" by a degree of 5 (limit - gap).
//            |                    lim=8    |    `limit - (len - a.ext - b.ext)`
//          ,,|,                        ,''':'''\
//         |,,,,|,,,,,,,,,              |    ,,,,|,,,,,,,,,
//    ,,,,,|,,,,|         |     ,,,,,,,,|   |    |         |
//   |   x-|====|----x    |    |   x----|---|----|-----x   |
//    ''''1|3'5'789AB     |     ''''12345678|ABCDEF123     |
//         2'4'6''''''''''                   ''''''''''''''
//
fn aabb_encroaching_vec(
    a: bevy::render::primitives::Aabb,
    b: bevy::render::primitives::Aabb,
    limit: Vec3A,
) -> (Vec3A, CollissionState) {
    // calculate "gap" between intersects
    let gap_vec = a.center.distance(b.center) - a.half_extents - b.half_extents;
    // print!("gap: {} :::: \na.center:{} |///| b.center:{} <><> distance:{} :::: \na.extn:{} |///| b.extn:{}\n\n", gap_vec, a.center, b.center, a.center.distance(b.center), a.half_extents, b.half_extents);
    if gap_vec.max_element() <= 0.0 {
        // println!("intersecting! abs gap: {} :::: ", gap_vec.abs());
        return (gap_vec.abs(), CollissionState::Intersecting);
    } else {
        let encroach_vec = limit - gap_vec;
        if encroach_vec.min_element() > 0.0 {
            // println!("encroaching! encroach vec: {} :::: ", encroach_vec);
            return (encroach_vec, CollissionState::Encroaching);
        } else {
            // println!("not intersecting! gap vec: {} :::: ", gap_vec);
            return (gap_vec, CollissionState::Uncontacting);
        }
    }
}

fn aabb_intersects(a: bevy::render::primitives::Aabb, b: bevy::render::primitives::Aabb) -> bool {
    let p = a.center - b.center;
    p.x.abs() <= a.half_extents.x + b.half_extents.x
        && p.y.abs() <= a.half_extents.y + b.half_extents.y
        && p.z.abs() <= a.half_extents.z + b.half_extents.z
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin)

        .add_systems(Startup, (setup, setup_model_resources))
        .add_systems(Startup, (setup_expr_list))

        .add_systems(Update, ui_example_system)
        .add_systems(Update, (process_physics, apply_physics))
        .add_systems(
            Update,
            (
                ui::field::input_mouse_refocus.pipe(error_handler),
                ui::field::handle_text_input,
            ),
        )
        .run();
}

fn error_handler(In(result): In<anyhow::Result<()>>) {
    if let Err(err) = result {
        println!("encountered an error {:?}", err);
    }
}
