pub trait Renderer {
    fn clear(&mut self, r: u8, g: u8, b: u8);
    fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, r: u8, g: u8, b: u8, fill: bool);
    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, r: u8, g: u8, b: u8);
    fn present(&mut self);
}
