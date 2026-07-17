mod framebuffer;
mod line;
mod bmp;

use framebuffer::Framebuffer;
use line::draw_line;

fn main() {
    let width = 800;
    let height = 600;
    let mut fb = Framebuffer::new(width, height, (255, 255, 255)); 

    let color = (200, 30, 30); 

    let cx = width / 2;
    let cy = height / 2;

    draw_line(&mut fb, cx, cy, cx + 250, cy - 100, color);
    draw_line(&mut fb, cx, cy, cx + 100, cy - 250, color);
    draw_line(&mut fb, cx, cy, cx - 100, cy - 250, color);
    draw_line(&mut fb, cx, cy, cx - 250, cy - 100, color);
    draw_line(&mut fb, cx, cy, cx - 250, cy + 100, color);
    draw_line(&mut fb, cx, cy, cx - 100, cy + 250, color);
    draw_line(&mut fb, cx, cy, cx + 100, cy + 250, color);
    draw_line(&mut fb, cx, cy, cx + 250, cy + 100, color);

    bmp::save_bmp(&fb, "out.bmp").expect("no se pudo guardar out.bmp");
    println!("Listo: out.bmp generado ({}x{})", width, height);
}