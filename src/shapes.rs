// This module has methods for drawing shapes
use crate::{Color, RendiumDrawHandle};

pub trait DrawShape {
    fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, col: Color);
    fn draw_rect_lines(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        thickness: i32,
        col: Color,
    );
    fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, col: Color, segments: usize);
    fn draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, col: Color);
    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, thickness: i32, col: Color);
}

impl DrawShape for RendiumDrawHandle {
    fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, col: Color) {
        let base = self.vertices.len() as u16;

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
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        thickness: i32,
        col: Color,
    ) {
        self.draw_rect(x, y, width, thickness, col);
        self.draw_rect(x, y + height - thickness, width, thickness, col);
        self.draw_rect(x, y, thickness, height, col);
        self.draw_rect(x + width - thickness, y, thickness, height, col);
    }

    fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, col: Color, segments: usize) {
        if segments < 3 {
            return;
        }

        let base = self.vertices.len() as u16;
        let cx = cx as f32;
        let cy = cy as f32;
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

    fn draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, col: Color) {
        let base = self.vertices.len() as u16;

        self.add_vertex([x1 as f32, y1 as f32, 0.0], col);
        self.add_vertex([x2 as f32, y2 as f32, 0.0], col);
        self.add_vertex([x3 as f32, y3 as f32, 0.0], col);

        self.add_index(base);
        self.add_index(base + 1);
        self.add_index(base + 2);
    }

    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, thickness: i32, col: Color) {
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
