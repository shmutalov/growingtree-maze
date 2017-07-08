extern crate rand;
extern crate cgmath;
extern crate three;

pub mod growing_tree_maze;

use rand::{thread_rng, Rng};
use growing_tree_maze::{GrowingTreeMaze, CellType};
use std::f32::consts::PI;
use std::env;

use cgmath::prelude::*;
use cgmath::{Rad, Deg};

const GROUND_COLOR: three::Color = 0xD4A190;
const CUBE_COLOR: three::Color = 0x00FF00;
const BLOCK_SIZE: f32 = 32.0;

fn main() {
    let args: Vec<String> = env::args().collect();
    let width: usize = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(40)
    } else {
        40
    };
    let height: usize = if args.len() > 2 {
        args[2].parse::<usize>().unwrap_or(20)
    } else {
        20
    };

    let mut maze = GrowingTreeMaze::new(width, height);

    let mut rng = thread_rng();
    let x_start = rng.gen_range::<usize>(0, width - 1);
    let y_start = rng.gen_range::<usize>(0, height - 1);

    maze.generate(x_start, y_start, 0_f64);
    maze.print();

    let mut win = three::Window::new("Three-rs maze example by Sherzod Mutalov", "data/shaders");
    let mut cam = win.factory.perspective_camera(75.0, 1.0, 1000.0);
    cam.set_position([0.0, 64.0, 0.0]);
    let mut controls = three::OrbitControls::new(&cam, [0.0, 2.0, -5.0], [0.0, 0.0, 0.0]);

    let mut dir_light = win.factory.directional_light(0xffffff, 0.9);
    dir_light.look_at([15.0, 35.0, 35.0], [0.0, 0.0, 2.0], None);
    win.scene.add(&dir_light);

    let ground_geometry = three::Geometry::new_plane(BLOCK_SIZE * width as f32,
                                                     BLOCK_SIZE * height as f32);
    let ground_material = three::Material::MeshLambert {
        color: GROUND_COLOR,
        flat: false,
    };
    let mut ground_mesh = win.factory.mesh(ground_geometry.clone(), ground_material.clone());

    let angle = Deg(-90.0);
    let q = cgmath::Quaternion::from_angle_x(angle);
    ground_mesh.set_orientation([q.v.x, q.v.y, q.v.z, q.s]);
    ground_mesh.set_position([0.0, 0.0, 0.0]);

    win.scene.add(&ground_mesh);

    let cube_geometry = three::Geometry::new_box(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE);
    let cube_material = three::Material::MeshLambert {
        color: CUBE_COLOR,
        flat: false,
    };
    let cube_mesh = win.factory.mesh(cube_geometry.clone(), cube_material.clone());

    let mut maze_group = win.factory.group();

    let mut cube_meshes = Vec::with_capacity(height*width);
    for z in 0..height {
        for x in 0..width {
            if *maze.get_cell(x, z) != CellType::Wall {
                continue;
            }

            let mut cell_mesh = win.factory.mesh_instance(&cube_mesh, cube_material.clone());
            cell_mesh.set_position([BLOCK_SIZE * x as f32, 0.0, BLOCK_SIZE * z as f32]);
            cube_meshes.push(cell_mesh);
            win.scene.add(&cube_meshes[cube_meshes.len()-1]);
        }
    }

    while win.update() && !three::KEY_ESCAPE.is_hit(&win.input) {
        controls.update(&win.input);

        win.render(&cam);
    }
}
