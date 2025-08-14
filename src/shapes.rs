// This module has methods for drawing shapes
use crate::RendiumDrawHandle;
use crate::types::{Color, Vector2};

pub trait DrawShape {
    fn draw_rect(&mut self, pos: Vector2, width: i32, height: i32, col: Color);
    fn draw_rect_lines(
        &mut self,
        pos: Vector2,
        width: i32,
        height: i32,
        thickness: f32,
        col: Color,
    );
    fn draw_circle(&mut self, pos: Vector2, radius: i32, col: Color, segments: usize);
    fn draw_triangle(&mut self, p1: Vector2, p2: Vector2, p3: Vector2, col: Color);
    fn draw_line(&mut self, from: Vector2, to: Vector2, thickness: i32, col: Color);
}

impl DrawShape for RendiumDrawHandle {
    fn draw_rect(&mut self, pos: Vector2, width: i32, height: i32, col: Color) {
        let base = self.vertices.len() as u16;
        let x = pos.0 as i32;
        let y = pos.1 as i32;

        self.add_vertex([x as f32, y as f32, 0.0], col);
        self.add_vertex([x as f32, (y + height) as f32, 0.0], col);
        self.add_vertex([(x + width) as f32, y as f32, 0.0], col);
        self.add_vertex([(x + width) as f32, (y + height) as f32, 0.0], col);

        self.add_index(base);
        self.add_index(base + 1);
        self.add_index(base + 2);

        self.add_index(base + 2);
        self.add_index(base + 1);
        self.add_index(base + 3);
    }

    fn draw_rect_lines(
        &mut self,
        pos: Vector2,
        width: i32,
        height: i32,
        thickness: f32,
        col: Color,
    ) {
        let x = pos.0;
        let y = pos.1;
        self.draw_rect(pos.clone(), width, thickness as i32, col);
        self.draw_rect(
            (x, y + height as f32 - thickness).into(),
            width,
            thickness as i32,
            col,
        );
        self.draw_rect(pos, thickness as i32, height, col);
        self.draw_rect(
            (x + width as f32 - thickness, y).into(),
            thickness as i32,
            height,
            col,
        );
    }

    fn draw_circle(&mut self, pos: Vector2, radius: i32, col: Color, segments: usize) {
        if segments < 3 {
            return;
        }

        let base = self.vertices.len() as u16;
        let cx = pos.0;
        let cy = pos.1;
        let radius = radius as f32;

        // center
        self.add_vertex([cx, cy, 0.0], col);

        for i in 0..segments {
            let theta = -(i as f32) / segments as f32 * std::f32::consts::TAU;
            let x = cx + radius * theta.cos();
            let y = cy + radius * theta.sin();
            self.add_vertex([x, y, 0.0], col);
        }

        for i in 0..segments {
            let next = if i + 1 == segments { 0 } else { i + 1 };
            self.add_index(base);
            self.add_index(base + i as u16 + 1);
            self.add_index(base + next as u16 + 1);
        }
    }

    fn draw_triangle(&mut self, p1: Vector2, p2: Vector2, p3: Vector2, col: Color) {
        let x1 = p1.0;
        let y1 = p1.1;
        let x2 = p2.0;
        let y2 = p2.0;
        let x3 = p3.1;
        let y3 = p3.1;

        let base = self.vertices.len() as u16;

        self.add_vertex([x1 as f32, y1 as f32, 0.0], col);
        self.add_vertex([x2 as f32, y2 as f32, 0.0], col);
        self.add_vertex([x3 as f32, y3 as f32, 0.0], col);

        self.add_index(base);
        self.add_index(base + 1);
        self.add_index(base + 2);
    }

    fn draw_line(&mut self, from: Vector2, to: Vector2, thickness: i32, col: Color) {
        let x1 = from.0;
        let y1 = from.1;
        let x2 = to.0;
        let y2 = to.1;

        let dx = (x2 - x1) as f32;
        let dy = (y2 - y1) as f32;

        let length = (dx * dx + dy * dy).sqrt();
        if length == 0.0 {
            return;
        }

        let dir_x = dx / length;
        let dir_y = dy / length;

        let nx = -dir_y;
        let ny = dir_x;

        let half_thickness = thickness as f32 / 2.0;

        let offset_x = nx * half_thickness;
        let offset_y = ny * half_thickness;

        let p1 = [x1 as f32 + offset_x, y1 as f32 + offset_y, 0.0];
        let p2 = [x1 as f32 - offset_x, y1 as f32 - offset_y, 0.0];
        let p3 = [x2 as f32 + offset_x, y2 as f32 + offset_y, 0.0];
        let p4 = [x2 as f32 - offset_x, y2 as f32 - offset_y, 0.0];

        let base = self.vertices.len() as u16;

        self.add_vertex(p1, col);
        self.add_vertex(p2, col);
        self.add_vertex(p3, col);
        self.add_vertex(p4, col);

        self.add_index(base);
        self.add_index(base + 2);
        self.add_index(base + 1);

        self.add_index(base + 2);
        self.add_index(base + 3);
        self.add_index(base + 1);
    }
}
