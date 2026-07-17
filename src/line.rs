use crate::framebuffer::Framebuffer;

pub fn draw_line(fb: &mut Framebuffer, x0: i32, y0: i32, x1: i32, y1: i32, color: (u8, u8, u8)) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx: i32 = if x0 < x1 { 1 } else { -1 };
    let sy: i32 = if y0 < y1 { 1 } else { -1 };

    let mut x = x0;
    let mut y = y0;

    if dx >= dy {
        let mut err = dx / 2; 
        loop {
            fb.set_pixel(x, y, color);
            if x == x1 {
                break;
            }
            x += sx;
            err -= dy;
            if err < 0 {
                y += sy;
                err += dx;
            }
        }
    } else {
        let mut err = dy / 2;
        loop {
            fb.set_pixel(x, y, color);
            if y == y1 {
                break;
            }
            y += sy;
            err -= dx;
            if err < 0 {
                x += sx;
                err += dy;
            }
        }
    }
}