use crate::components::color::RGBA;
use crate::components::font::FontSettings;
use crate::components::font::FontTrait;
use crate::components::font::FontType;
use crate::components::polygons::Mesh;
use crate::components::polygons::Polygon;
use crate::components::polygons::Quad;
use crate::components::polygons::Triangle;
use crate::components::vectors::Vector3D;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::Font;
use speedy2d::font::FormattedTextBlock;
use speedy2d::font::TextLayout;
use speedy2d::font::TextOptions;

use speedy2d::window::WindowHelper;
use speedy2d::Graphics2D;
use std::rc::Rc;

#[derive(Debug)]
struct TriangleDraw {
    points: [(f64, f64); 3],
    color: RGBA,
    id: u32,
}

impl TriangleDraw {
    pub fn new(points: [(f64, f64); 3], color: RGBA) -> TriangleDraw {
        let id = 3;
        TriangleDraw { points, color, id }
    }
}

#[derive(Debug)]
struct QuadDraw {
    points: [(f64, f64); 4],
    color: RGBA,
    id: u32,
}

impl QuadDraw {
    pub fn new(points: [(f64, f64); 4], color: RGBA) -> QuadDraw {
        let id = 3;
        QuadDraw { points, color, id }
    }
}

#[derive(Debug)]
struct LineDraw {
    points: ((f64, f64), (f64, f64)),
    color: RGBA,
    thickness: f64,
    id: u32,
}

impl LineDraw {
    pub fn new(points: ((f64, f64), (f64, f64)), color: RGBA, thickness: f64) -> LineDraw {
        let id = 2;
        LineDraw {
            points,
            color,
            id,
            thickness,
        }
    }
}

#[derive(Debug)]
struct TextDraw {
    position: (f64, f64),
    text: String,
    font_settings: FontSettings,
    id: u32,
}

impl TextDraw {
    pub fn new(position: (f64, f64), text: String, font_settings: FontSettings) -> TextDraw {
        let id = 1;
        TextDraw {
            position,
            text,
            font_settings,
            id,
        }
    }
}

#[derive(Debug)]
struct FillDraw {
    color: RGBA,
    id: u32,
}

impl FillDraw {
    pub fn new(color: RGBA) -> FillDraw {
        let id = 4;
        FillDraw { color, id }
    }
}

impl Draw for TriangleDraw {
    fn draw(&self, graphics: &mut Graphics2D) {
        let p1: (f64, f64) = self.points[0];
        let p2: (f64, f64) = self.points[1];
        let p3: (f64, f64) = self.points[2];

        let v1: Vector2<f32> = Vector2::new(p1.0 as f32, p1.1 as f32);
        let v2: Vector2<f32> = Vector2::new(p2.0 as f32, p2.1 as f32);
        let v3: Vector2<f32> = Vector2::new(p3.0 as f32, p3.1 as f32);

        let vertex_positions: [Vector2<f32>; 3] = [v1, v2, v3];
        let color: Color = self.color.to_sp2d_color();
        graphics.draw_triangle(vertex_positions, color);
    }
    fn id(&self) -> u32 {
        self.id
    }
}

impl Draw for QuadDraw {
    fn draw(&self, graphics: &mut Graphics2D) {
        let p1: (f64, f64) = self.points[0];
        let p2: (f64, f64) = self.points[1];
        let p3: (f64, f64) = self.points[2];
        let p4: (f64, f64) = self.points[2];

        let v1: Vector2<f32> = Vector2::new(p1.0 as f32, p1.1 as f32);
        let v2: Vector2<f32> = Vector2::new(p2.0 as f32, p2.1 as f32);
        let v3: Vector2<f32> = Vector2::new(p3.0 as f32, p3.1 as f32);
        let v4: Vector2<f32> = Vector2::new(p4.0 as f32, p4.1 as f32);

        let vertex_positions: [Vector2<f32>; 4] = [v1, v2, v3, v4];
        let color: Color = self.color.to_sp2d_color();
        graphics.draw_quad(vertex_positions, color);
    }
    fn id(&self) -> u32 {
        self.id
    }
}

impl Draw for LineDraw {
    fn draw(&self, graphics: &mut Graphics2D) {
        let p1: (f64, f64) = self.points.0;
        let p2: (f64, f64) = self.points.1;

        let v1: Vector2<f32> = Vector2::new(p1.0 as f32, p1.1 as f32);
        let v2: Vector2<f32> = Vector2::new(p2.0 as f32, p2.1 as f32);

        let color: Color = self.color.to_sp2d_color();
        let thickness: f32 = self.thickness as f32;
        graphics.draw_line(v1, v2, thickness, color);
    }
    fn id(&self) -> u32 {
        self.id
    }
}

impl Draw for TextDraw {
    fn draw(&self, graphics: &mut Graphics2D) {
        let text: &String = &self.text;
        let font_settings: &FontSettings = &self.font_settings;
        let position_f64: (f64, f64) = self.position;
        let position_f32: (f32, f32) = (position_f64.0 as f32, position_f64.1 as f32);
        let font_type: &FontType = &font_settings.font_type;
        let font_color: &RGBA = &font_settings.font_color;

        let text_options: TextOptions = TextOptions::new();
        let size: f32 = font_settings.font_size as f32;
        let font: &Font = font_type.get_sp2d_font();
        let color: Color = font_color.to_sp2d_color();
        let text_block: Rc<FormattedTextBlock> = font.layout_text(text, size, text_options);
        graphics.draw_text(position_f32, color, &text_block);
    }
    fn id(&self) -> u32 {
        self.id
    }
}

impl Draw for FillDraw {
    fn draw(&self, graphics: &mut Graphics2D) {
        let color: Color = self.color.to_sp2d_color();
        graphics.clear_screen(color);
    }
    fn id(&self) -> u32 {
        self.id
    }
}

trait Draw {
    fn draw(&self, graphics: &mut Graphics2D);
    fn id(&self) -> u32;
}

#[derive(Debug)]
enum DrawType {
    TriangleDraw(TriangleDraw),
    QuadDraw(QuadDraw),
    LineDraw(LineDraw),
    TextDraw(TextDraw),
    FillDraw(FillDraw),
}

impl Draw for DrawType {
    fn draw(&self, graphics: &mut Graphics2D) {
        match self {
            DrawType::TriangleDraw(s) => s.draw(graphics),
            DrawType::QuadDraw(s) => s.draw(graphics),
            DrawType::LineDraw(s) => s.draw(graphics),
            DrawType::TextDraw(s) => s.draw(graphics),
            DrawType::FillDraw(s) => s.draw(graphics),
        }
    }

    fn id(&self) -> u32 {
        match self {
            DrawType::TriangleDraw(s) => s.id(),
            DrawType::QuadDraw(s) => s.id(),
            DrawType::LineDraw(s) => s.id(),
            DrawType::TextDraw(s) => s.id(),
            DrawType::FillDraw(s) => s.id(),
        }
    }
}

pub struct CursorGrab {
    pub is_grabbed: bool,
    pub previous_state: bool,
    pub first_pass: bool,
}

impl CursorGrab {
    pub fn new() -> CursorGrab {
        let is_grabbed = false;
        let previous_state = false;
        let first_pass = true;
        CursorGrab {
            is_grabbed,
            previous_state,
            first_pass,
        }
    }

    pub fn set_cursor_grab(&mut self, state: bool) {
        self.is_grabbed = state;
    }

    pub fn apply_cursor_grab(&mut self, helper: &mut WindowHelper) {
        if self.is_grabbed != self.previous_state {
            let grab = helper.set_cursor_grab(self.is_grabbed);
            self.first_pass = false;
            if grab.is_err() {
                self.is_grabbed = false;
                self.first_pass = true;
            }
        }
        self.previous_state = self.is_grabbed;
    }
}

pub struct Graphics {
    width: u32,
    height: u32,
    bg_color: RGBA,
    cursor_grab: CursorGrab,
    buffer: Vec<DrawType>,
    buffer_execute: bool,
}

impl Graphics {
    pub fn new(width: u32, height: u32) -> Self {
        let bg_color: RGBA = RGBA::from_rgb(0.05, 0.05, 0.05);
        let cursor_grab: CursorGrab = CursorGrab::new();
        let buffer: Vec<DrawType> = vec![];
        let buffer_execute: bool = false;

        Graphics {
            width,
            height,
            bg_color,
            cursor_grab,
            buffer,
            buffer_execute,
        }
    }

    pub fn get_cursor_grab(&self) -> &CursorGrab {
        &self.cursor_grab
    }

    pub fn set_cursor_grab(&mut self, grab: bool) {
        self.cursor_grab.set_cursor_grab(grab);
    }

    pub fn execute_helper_functions(&mut self, helper: &mut WindowHelper) {
        self.cursor_grab.apply_cursor_grab(helper);
    }

    pub fn execute_buffer(&mut self, graphics: &mut Graphics2D) {
        let center = Vector3D::new(self.width as f64 / 2.0, self.height as f64 / 2.0, 0.0);
        self.add_crosshair(center, 20.0);
        self.sort_buffer();
        for buffer_type in &self.buffer {
            buffer_type.draw(graphics);
        }

        self.buffer.clear();
        self.buffer_execute = false;
    }

    pub fn get_buffer_state(&self) -> bool {
        self.buffer_execute
    }

    pub fn update(&mut self) {
        self.buffer_execute = true;
    }

    pub fn set_screensize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_background_color(&mut self, color: RGBA) {
        self.bg_color = color;
    }

    fn sort_buffer(&mut self) {
        self.buffer.sort_by(|a, b| a.id().cmp(&b.id()).reverse());
    }

    fn push_to_buffer(&mut self, draw_type: DrawType) {
        self.buffer.push(draw_type);
    }

    fn set_title(&self, title: String) {}

    fn get_screensize(&self) {}

    fn get_width(&self) {}

    fn get_height(&self) {}

    fn get_pointer_xy(&self) {}

    pub fn add_crosshair(&mut self, center: Vector3D, size: f64) {
        let half_size = size / 2.0;
        let left = Vector3D::new(center.x - half_size, center.y, center.z);
        let right = Vector3D::new(center.x + half_size, center.y, center.z);
        let top = Vector3D::new(center.x, center.y - half_size, center.z);
        let bottom = Vector3D::new(center.x, center.y + half_size, center.z);

        let color = RGBA::from_rgb(1.0, 1.0, 1.0);

        self.draw_line(left, right, color, 1.0);
        self.draw_line(top, bottom, color, 1.0);
    }

    pub fn draw_polygons(&mut self, mesh: Mesh) {
        for polygon in mesh.polygons {
            match polygon {
                Polygon::Triangle(triangle) => {
                    self.draw_triangle(triangle);
                }
                Polygon::Quad(quad) => {
                    self.draw_quad(quad);
                }
            }
        }
    }

    pub fn draw_triangle(&mut self, triangle: Triangle) {
        let vertices: [Vector3D; 3] = triangle.vertices;
        let shader: RGBA = triangle.shader;
        let color: RGBA = triangle.color;
        let color: RGBA = color.multiply(&shader);

        let v1: Vector3D = vertices[0];
        let v2: Vector3D = vertices[1];
        let v3: Vector3D = vertices[2];

        let p1: (f64, f64) = (v1.to_tuple().0, v1.to_tuple().1);
        let p2: (f64, f64) = (v2.to_tuple().0, v2.to_tuple().1);
        let p3: (f64, f64) = (v3.to_tuple().0, v3.to_tuple().1);

        let points: [(f64, f64); 3] = [p1, p2, p3];
        let triangle_draw: TriangleDraw = TriangleDraw::new(points, color);
        let draw_type: DrawType = DrawType::TriangleDraw(triangle_draw);
        self.push_to_buffer(draw_type);
    }

    pub fn draw_quad(&mut self, quad: Quad) {
        let vertices: [Vector3D; 4] = quad.vertices;
        let shader: RGBA = quad.shader;
        let color: RGBA = quad.color;
        let color: RGBA = color.multiply(&shader);

        let v1: Vector3D = vertices[0];
        let v2: Vector3D = vertices[1];
        let v3: Vector3D = vertices[2];
        let v4: Vector3D = vertices[3];

        let p1: (f64, f64) = (v1.to_tuple().0, v1.to_tuple().1);
        let p2: (f64, f64) = (v2.to_tuple().0, v2.to_tuple().1);
        let p3: (f64, f64) = (v3.to_tuple().0, v3.to_tuple().1);
        let p4: (f64, f64) = (v4.to_tuple().0, v4.to_tuple().1);

        let points: [(f64, f64); 4] = [p1, p2, p3, p4];
        let quad_draw: QuadDraw = QuadDraw::new(points, color);
        let draw_type: DrawType = DrawType::QuadDraw(quad_draw);
        self.push_to_buffer(draw_type);
    }

    pub fn draw_line(&mut self, v1: Vector3D, v2: Vector3D, color: RGBA, thickness: f64) {
        let p1: (f64, f64) = (v1.x, v1.y);
        let p2: (f64, f64) = (v2.x, v2.y);
        let line_draw: LineDraw = LineDraw::new((p1, p2), color, thickness);
        let draw_type: DrawType = DrawType::LineDraw(line_draw);
        self.push_to_buffer(draw_type);
    }

    pub fn draw_text(&mut self, point: (f64, f64), text: String, font_settings: FontSettings) {
        let text_draw: TextDraw = TextDraw::new(point, text, font_settings);
        let draw_type: DrawType = DrawType::TextDraw(text_draw);
        self.push_to_buffer(draw_type);
    }

    pub fn clear_screen(&mut self) {
        let fill_draw: FillDraw = FillDraw::new(self.bg_color.clone());
        let draw_type: DrawType = DrawType::FillDraw(fill_draw);
        self.push_to_buffer(draw_type);
    }
}
