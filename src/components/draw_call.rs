use super::polygons::Mesh;
use super::vectors::Vector3D;
use crate::abstracts::body::Body;
use crate::abstracts::body::BodyType;
use crate::components::backface_culling::BackfaceCulling;
use crate::components::color::RGBA;
use crate::components::frametime::FrameTimeHandler;
use crate::components::graphics::Graphics;
use crate::components::polygons::Polygon;
use crate::components::shaders::Light;
use crate::components::shaders::Shaders;
use crate::components::simulation::Simulation;
use crate::components::z_buffer::ZBufferSort;
use crate::Camera;
use rayon::prelude::*;

pub struct DrawCall {
    pub graphics: Graphics,
    pub frame_timing: FrameTimeHandler,
    pub simulation: Simulation,
    pub light: Light,
    shaders: Shaders,
    backface_culling: BackfaceCulling,
    z_buffer_sort: ZBufferSort,
}

impl DrawCall {
    pub fn new(graphics: Graphics, simulation: Simulation) -> DrawCall {
        let frame_timing: FrameTimeHandler = FrameTimeHandler::new(30);
        let light: Light = Light::get_light();
        let shaders: Shaders = Shaders::new();
        let backface_culling: BackfaceCulling = BackfaceCulling::new();
        let z_buffer_sort: ZBufferSort = ZBufferSort::new();
        DrawCall {
            graphics,
            frame_timing,
            simulation,
            light,
            shaders,
            backface_culling,
            z_buffer_sort,
        }
    }

    fn get_camera_light(&self) -> Light {
        let camera: &Camera = &self.simulation.camera;
        let camera_position: Vector3D = camera.camera_position;
        let camera_target: Vector3D = camera.camera_target;
        let light_camera: Light = Light::get_light_from_position(camera_position, camera_target);
        light_camera
    }

    fn get_lights(&self, meshes: &Vec<Mesh>) -> Vec<Light> {
        let mut lights: Vec<Light> = vec![];
        for mesh in meshes {
            let mesh_light: &Option<Light> = &mesh.light;
            if mesh_light.is_some() {
                lights.push(mesh_light.unwrap());
            }
        }
        // let camera_light: Light = self.get_camera_light();
        // lights.push(camera_light);
        lights
    }

    fn get_meshes(&mut self, objects: Vec<BodyType>) -> Vec<Mesh> {
        let meshes: Vec<Mesh> = objects.par_iter().map(|body| body.mesh().clone()).collect();
        meshes
    }

    fn draw_convex_hulls(&mut self, meshes: Vec<Mesh>) {
        let camera: &mut Camera = &mut self.simulation.camera;
        let color: RGBA = RGBA::from_rgb(0.6, 1.0, 0.6);
        let thickness = 1.0;

        for mesh in meshes {
            for i in 0..mesh.convex_hull.len() {
                let v1: Vector3D = mesh.convex_hull[i];
                let v2: Vector3D = mesh.convex_hull[(i + 1) % mesh.convex_hull.len()];

                let line: Option<(Vector3D, Vector3D)> = camera.transform_line(v1, v2);
                if line.is_some() {
                    let (v1, v2): (Vector3D, Vector3D) = line.unwrap();
                    self.graphics.draw_line(v1, v2, color, thickness);
                }
            }
        }
    }

    fn combine_meshes(&mut self, meshes: Vec<Mesh>) -> Mesh {
        let total_polygons: usize = meshes.iter().map(|mesh| mesh.polygons.len()).sum();
        self.simulation.polygon_count = total_polygons;
        let mut polygons: Vec<Polygon> = Vec::with_capacity(total_polygons);

        for mesh in meshes {
            polygons.extend(mesh.polygons);
        }

        Mesh::new(polygons)
    }

    fn cull_backfaces_meshes(&self, meshes: Vec<Mesh>) {}

    fn apply_lighting_meshes(&self, meshes: Vec<Mesh>, lights: Vec<Light>) {}

    fn cull_backfaces_mesh(&self, mut mesh: Mesh) -> Mesh {
        let camera: &Camera = &self.simulation.camera;
        let camera_position: Vector3D = camera.camera_position;
        mesh = self.backface_culling.cull_backfaces(mesh, &camera_position);
        mesh
    }

    fn apply_lighting_mesh(&self, mut mesh: Mesh, lights: Vec<Light>) -> Mesh {
        let camera: &Camera = &self.simulation.camera;
        let camera_position: Vector3D = camera.camera_position;
        for light in lights {
            mesh = self
                .shaders
                .apply_pbr_lighting(mesh, &light, &camera_position);
        }
        mesh
    }

    fn apply_projection(&mut self, mesh: Mesh) -> Mesh {
        let camera: &mut Camera = &mut self.simulation.camera;
        let mesh: Mesh = camera.apply_projection_polygons(mesh);
        mesh
    }

    fn apply_z_buffer_sort(&self, mut mesh: Mesh) -> Mesh {
        let camera = &self.simulation.camera;
        let camera_position = camera.camera_position;
        mesh = self
            .z_buffer_sort
            .get_sorted_polygons(mesh, camera_position);
        mesh
    }

    fn get_lights_1(&self) -> Vec<Light> {
        let lights: Vec<Light> = vec![self.get_camera_light(), self.light.clone()];
        lights
    }

    fn get_lights_2(&self) -> Vec<Light> {
        let lights: Vec<Light> = vec![self.get_camera_light()];
        lights
    }

    fn get_lights_3(&self) -> Vec<Light> {
        let lights: Vec<Light> = vec![self.light.clone()];
        lights
    }

    pub fn draw(&mut self, objects: Vec<BodyType>) {
        let meshes: Vec<Mesh> = self.get_meshes(objects);
        // let lights = self.get_lights(&meshes);

        if self.simulation.draw_mesh {
            self.draw_convex_hulls(meshes.clone());
        }

        if self.simulation.draw_polygons {
            let mut mesh: Mesh = self.combine_meshes(meshes);

            let lights: Vec<Light> = self.get_lights_2();

            mesh = self.cull_backfaces_mesh(mesh);
            mesh = self.apply_z_buffer_sort(mesh);
            mesh = self.apply_lighting_mesh(mesh, lights);
            mesh = self.apply_projection(mesh);

            self.graphics.draw_polygons(mesh);
        }
        self.graphics.update();
    }
}
