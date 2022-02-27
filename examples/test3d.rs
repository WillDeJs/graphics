use graphics::canvas::Canvas;
use graphics::color::Color;
use graphics::render::*;

use graphics::math::matrix::Mat4x4;
use graphics::math::FVec3D;
use graphics::utils::d3::*;

fn main() {
    let drawing_canvas = Draw3D::new(800, 600, "3D Render".into());
    drawing_canvas.render();
}

pub struct Draw3D {
    width: u32,
    height: u32,
    title: String,
    projection: Mat4x4<f32>,
    mesh: Mesh3D,
    theta: f32,
    camera: FVec3D,
    look_dir: FVec3D,
    yaw: f32,
}

impl Draw3D {
    pub fn new(width: u32, height: u32, title: String) -> Self {
        Self {
            width,
            height,
            title,
            mesh: Mesh3D::default(),
            theta: std::f32::consts::PI,
            projection: Mat4x4::identity(),
            camera: FVec3D::new(0.0, 1.0, -3.0),
            look_dir: FVec3D::new(0.0, 0.0, 0.0),
            yaw: 0.0,
        }
    }
}

impl Render2D for Draw3D {
    fn height(&mut self) -> u32 {
        self.height
    }
    fn width(&mut self) -> u32 {
        self.width
    }

    fn title(&mut self) -> String {
        self.title.clone()
    }

    fn setup(&mut self, canvas: &mut Canvas) -> bool {
        let object = Object3D::from_file("./assets/teapot.obj").unwrap();

        self.mesh = object.mesh;

        let near = 0.001;
        let far = 1000.0;
        let fov: f32 = 90.0;
        let aspect_ratio = (canvas.height() as f32) / (canvas.width() as f32);
        let fov_radians = 1.0_f32 / (fov * 0.5 / 180.0 * std::f32::consts::PI).tan();

        self.projection = Mat4x4::projected(aspect_ratio, fov_radians, far, near);

        true
    }

    fn update(&mut self, canvas: &mut Canvas, input: &WinitInputHelper, delta_t: f32) -> bool {
        canvas.fill(Color::BLACK);

        let rotation_matrix_z: Mat4x4<f32> = Mat4x4::<f32>::rotate_z(self.theta);
        let rotation_matrix_x: Mat4x4<f32> = Mat4x4::<f32>::rotate_x(self.theta / 2.0);

        let mat_translation: Mat4x4<f32> = Mat4x4::<f32>::translate(0.0, 0.0, 1.0);
        // let mut world_matrix = Mat4x4::<f32>::identity();

        // Upon up and down press change Y axis
        if input.key_pressed(VirtualKeyCode::Down) {
            self.camera.y -= 8.0 * delta_t;
        }
        if input.key_pressed(VirtualKeyCode::Up) {
            self.camera.y += 8.0 * delta_t;
        }
        // Upon left and right press change X axis
        if input.key_pressed(VirtualKeyCode::Right) {
            self.camera.x += 8.0 * delta_t;
        }
        if input.key_pressed(VirtualKeyCode::Left) {
            self.camera.x -= 8.0 * delta_t;
        }

        let forward = self.look_dir * 8.0 * delta_t;
        // Use WASD as rotating keys
        if input.key_pressed(VirtualKeyCode::W) {
            self.camera = self.camera + forward;
        }
        if input.key_pressed(VirtualKeyCode::S) {
            self.camera = self.camera - forward;
        }
        if input.key_pressed(VirtualKeyCode::A) {
            self.yaw -= 2.0 * delta_t;
        }
        if input.key_pressed(VirtualKeyCode::D) {
            self.yaw += 2.0 * delta_t;
        }

        let mut world_matrix = rotation_matrix_z * rotation_matrix_x;
        world_matrix = world_matrix * mat_translation;

        let up_vector = FVec3D::new(0.0, 1.0, 0.0);
        let mut target = FVec3D::new(0.0, 0.0, 1.0);
        let camera_rotation = Mat4x4::<f32>::rotate_y(self.yaw);
        self.look_dir = camera_rotation.vector_multiply(target);
        target = self.camera + self.look_dir;

        let camera_matrix = Mat4x4::<f32>::point_at(self.camera, target, up_vector);

        let mat_view = camera_matrix.inverse();

        let mut tris_to_raster = Vec::<Triangle3D>::with_capacity(self.mesh.tris.len());
        // let rotation_matrix = rotation_matrix_x * rotation_matrix_z;
        for triangle in &self.mesh.tris {
            let mut transformed = Triangle3D::default();
            let mut projected = Triangle3D::default();
            let mut viewed = Triangle3D::default();

            // transform triangle
            transformed.vertices[0] = world_matrix.vector_multiply(triangle.vertices[0]);
            transformed.vertices[1] = world_matrix.vector_multiply(triangle.vertices[1]);
            transformed.vertices[2] = world_matrix.vector_multiply(triangle.vertices[2]);

            let line_one = transformed.vertices[1] - transformed.vertices[0];
            let line_two = transformed.vertices[2] - transformed.vertices[0];

            let normal = FVec3D::cross(line_one, line_two).unit_vector();

            // any of the points in the triangle is on the plane
            // lets calculate the field of view by getting its distance from the camera.
            let point_of_view = (transformed.vertices[0] - self.camera).unit_vector();
            if FVec3D::dot(point_of_view, normal) < 0.0 {
                // lest add a light direction to do some shading
                let light = FVec3D::new(0.0, 0.0, -1.0);

                let depth = FVec3D::dot(normal, light.unit_vector());

                let color = Color::rgb(170, 248, 11) * depth;

                // convert world space to view space
                viewed.vertices[0] = mat_view.vector_multiply(transformed.vertices[0]);
                viewed.vertices[1] = mat_view.vector_multiply(transformed.vertices[1]);
                viewed.vertices[2] = mat_view.vector_multiply(transformed.vertices[2]);

                projected.vertices[0] = self.projection.vector_multiply(viewed.vertices[0]);
                projected.vertices[1] = self.projection.vector_multiply(viewed.vertices[1]);
                projected.vertices[2] = self.projection.vector_multiply(viewed.vertices[2]);
                // scale into view
                projected.vertices[0] += FVec3D::new(1.0, 1.0, 0.0);
                projected.vertices[1] += FVec3D::new(1.0, 1.0, 0.0);
                projected.vertices[2] += FVec3D::new(1.0, 1.0, 0.0);
                // go to screen middle
                projected.vertices[0].x *= 0.5 * (canvas.width() as f32);
                projected.vertices[1].x *= 0.5 * (canvas.width() as f32);
                projected.vertices[2].x *= 0.5 * (canvas.width() as f32);
                projected.vertices[0].y *= 0.5 * (canvas.height() as f32 - 1.08);
                projected.vertices[1].y *= 0.5 * (canvas.height() as f32 - 1.08);
                projected.vertices[2].y *= 0.5 * (canvas.height() as f32 - 1.08);

                tris_to_raster.push(Triangle3D {
                    vertices: projected.vertices,
                    color,
                })
            }
        }
        tris_to_raster.sort_by(|a, b| {
            // sort by z mid point (distance from screen)
            let zsum1 = (a.vertices[0].z + a.vertices[1].z + a.vertices[2].z) / 3.0;
            let zsum2 = (b.vertices[0].z + b.vertices[1].z + b.vertices[2].z) / 3.0;

            zsum2
                .partial_cmp(&zsum1)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for triangle in tris_to_raster {
            canvas.fill_triangle(
                triangle.vertices[0].to_i32().into(),
                triangle.vertices[1].to_i32().into(),
                triangle.vertices[2].to_i32().into(),
                triangle.color,
            );
            // canvas.triangle(
            //     triangle.vertices[0].to_i32().into(),
            //     triangle.vertices[1].to_i32().into(),
            //     triangle.vertices[2].to_i32().into(),
            //     Color::GRAY,
            // );
        }
        true
    }
}
