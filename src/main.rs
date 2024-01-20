use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_editor_pls::prelude::*;
use core::f32;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EditorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_controls)
        .run();
}

pub fn keyboard_controls(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Camera)>,
) {
    if let Some((mut transform, _camera)) = query.iter_mut().next() {
        let speed = 10.;
        let forward = Vec3::new(1., 0., 0.);
        let left = Vec3::new(0., 0., -1.);
        let up = Vec3::new(0., 1., 0.);
        let mut pos = transform.translation;
        if input.pressed(KeyCode::W) {
            pos += speed * forward * time.delta_seconds();
        } else if input.pressed(KeyCode::S) {
            pos -= speed * forward * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            pos += speed * left * time.delta_seconds();
        } else if input.pressed(KeyCode::D) {
            pos -= speed * left * time.delta_seconds();
        }
        if input.pressed(KeyCode::Q) {
            pos += speed * up * time.delta_seconds();
        } else if input.pressed(KeyCode::E) {
            pos -= speed * up * time.delta_seconds();
        }

        transform.translation = pos;
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 10.))
            .looking_at(Vec3::ZERO, Vec3::ZERO),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 10.)),
        ..Default::default()
    });

    let real_hex = Hexagon::new(1.0, (0.0, 0.0));
    commands.spawn(PbrBundle {
        material: materials.add(Color::rgb_u8(0, 0, 255).into()),
        mesh: meshes.add(real_hex.get_mesh()),
        transform: Transform::from_xyz(0.0, 1.73, 0.0),
        ..Default::default()
    });

    let real_hex2 = Hexagon::new(1.0, (real_hex.width() * 0.5, (3. / 2. * real_hex.side)));
    commands.spawn(PbrBundle {
        material: materials.add(Color::rgb_u8(0, 255, 0).into()),
        mesh: meshes.add(real_hex2.get_mesh()),
        transform: Transform::from_xyz(0.0, 1.73, 0.0),
        ..Default::default()
    });

    let real_hex5 = Hexagon::new(
        1.0,
        (real_hex.width() * 0.5, (3. / 2. * real_hex.side) * -1.),
    );
    commands.spawn(PbrBundle {
        material: materials.add(Color::rgb_u8(0, 127, 0).into()),
        mesh: meshes.add(real_hex5.get_mesh()),
        transform: Transform::from_xyz(0.0, 1.73, 0.0),
        ..Default::default()
    });

    let real_hex7 = Hexagon::new(
        1.0,
        (
            (real_hex.width() * 0.5) * -1.,
            (3. / 2. * real_hex.side) * -1.,
        ),
    );
    commands.spawn(PbrBundle {
        material: materials.add(Color::rgb_u8(0, 255, 0).into()),
        mesh: meshes.add(real_hex7.get_mesh()),
        transform: Transform::from_xyz(0.0, 1.73, 0.0),
        ..Default::default()
    });

    let real_hex8 = Hexagon::new(
        1.0,
        ((real_hex.width() * 0.5) * -1., (3. / 2. * real_hex.side)),
    );
    commands.spawn(PbrBundle {
        material: materials.add(Color::rgb_u8(127, 127, 127).into()),
        mesh: meshes.add(real_hex8.get_mesh()),
        transform: Transform::from_xyz(0.0, 1.73, 0.0),
        ..Default::default()
    });

    let real_hex3 = Hexagon::new(1.0, ((real_hex.width() * 0.5) * 2., 0.0));
    commands.spawn(PbrBundle {
        material: materials.add(Color::rgb_u8(255, 0, 0).into()),
        mesh: meshes.add(real_hex3.get_mesh()),
        transform: Transform::from_xyz(0.0, 1.73, 0.0),
        ..Default::default()
    });

    // left hexagon
    let real_hex4 = Hexagon::new(1.0, (((real_hex.width() * 0.5) * 2.) * -1., 0.0));
    commands.spawn(PbrBundle {
        material: materials.add(Color::rgb_u8(127, 0, 0).into()),
        mesh: meshes.add(real_hex4.get_mesh()),
        transform: Transform::from_xyz(0.0, 1.73, 0.0),
        ..Default::default()
    });

    // commands.spawn(PbrBundle {
    //     material: materials.add(Color::rgb_u8(0, 255, 0).into()),
    //     mesh: meshes.add(hex.clone()),
    //     ..Default::default()
    // });
    let mut hex_grid: HexGrid<'_> = HexGrid::new();
    dbg!(&hex_grid);
    let cordinate1: Cordinate = Cordinate::new(0, 0);
    let cordinate_with_hexagon1 = CordinateWithHexagon::new(cordinate1, &real_hex5);
    hex_grid.add(&cordinate_with_hexagon1);
    dbg!(&hex_grid);
}

#[derive(Debug, Copy, Clone)]
struct Cordinate {
    x: i32,
    y: i32,
}

impl Cordinate {
    pub fn new(x: i32, y: i32) -> Cordinate {
        Cordinate { x, y }
    }
}
#[derive(Debug, Clone, Copy)]
struct CordinateWithHexagon<'a> {
    cordinate: Cordinate,
    hexagon: &'a Hexagon,
}

impl<'a> CordinateWithHexagon<'a> {
    pub fn new(cordinate: Cordinate, hexagon: &'a Hexagon) -> CordinateWithHexagon<'a> {
        CordinateWithHexagon { cordinate, hexagon }
    }
}
#[derive(Debug, Clone)]
pub struct HexGrid<'a> {
    list_of_hexagon: Vec<&'a CordinateWithHexagon<'a>>,
}

impl<'a> HexGrid<'a> {
    pub fn new() -> HexGrid<'a> {
        let list_of_hexagon = vec![];
        HexGrid { list_of_hexagon }
    }
    fn add(&mut self, cordinate: &'a CordinateWithHexagon<'a>) {
        self.list_of_hexagon.push(cordinate);
    }
}

impl<'a> Default for HexGrid<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hexagon {
    side: f32,
    center: (f32, f32),
}

impl Hexagon {
    pub fn new(side: f32, center: (f32, f32)) -> Hexagon {
        Hexagon { side, center }
    }

    pub fn perimeter(self) -> f32 {
        self.side * 6.0
    }

    pub fn radius(self) -> f32 {
        self.side / (2.0 * (std::f32::consts::PI / 6.0).tan())
    }

    pub fn height(self) -> f32 {
        f32::sqrt(3.0) * self.radius()
    }

    pub fn width(self) -> f32 {
        2.0 * self.radius()
    }

    pub fn area(self) -> f32 {
        self.perimeter() * self.radius() / 2.0
    }

    pub fn corners(self) -> Vec<(f32, f32)> {
        let mut corners: Vec<(f32, f32)> = vec![];
        for i in 0..6 {
            let angle_deg = 60.0 * (i as f32) - 30.0;
            let angle_rad = std::f32::consts::PI / 180.0 * angle_deg;
            corners.push((
                self.center.0 + self.side * angle_rad.cos(),
                self.center.1 + self.side * angle_rad.sin(),
            ));
        }

        corners
    }

    fn get_mesh(&self) -> Mesh {
        let center = ([self.center.0, self.center.1, 0.], [0., 0., 1.], [0., 0.]);
        let corners = self.corners();
        let mut vertices: Vec<([f32; 3], [f32; 3], [f32; 2])> = vec![center];
        for corner in &corners {
            let corner_vertices = ([corner.0, corner.1, 0.], [0., 0., 1.], [0., 0.]);
            vertices.push(corner_vertices);
        }

        let mut positions = Vec::with_capacity(6);
        let mut normals = Vec::with_capacity(6);
        let mut uvs = Vec::with_capacity(6);

        for (position, normal, uv) in &vertices {
            positions.push(*position);
            normals.push(*normal);
            uvs.push(*uv);
        }

        let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1]);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(indices));
        mesh
    }
}

pub fn normalize_angle(angle: f32) -> f32 {
    (angle + 360.0) % 360.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_hexagon_area() {
        let center = (0.0, 0.0);

        let angle_degree: f32 = normalize_angle(-30.0);
        let angle_radians = angle_degree.to_radians();
        println!("x: {:?}", 0.0 + 1.0 * angle_radians.cos());
        println!("y: {:?}", 0.0 + 1.0 * angle_radians.sin());

        let angle_degree = normalize_angle(30.0);
        let angle_radians = angle_degree.to_radians();
        println!("{:?}", 0.0 + 1.0 * angle_radians.cos());
        println!("{:?}", 0.0 + 1.0 * angle_radians.sin());

        let hexagon: Hexagon = Hexagon::new(1.0, center);
        assert_eq!(hexagon.area(), 2.5981);
    }
}
