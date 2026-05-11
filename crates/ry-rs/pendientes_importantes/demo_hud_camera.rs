// demo_hud_camera.rs
// Demo: HUD + Cámara 2D + Health Bars + Debug Overlay
//
// Controles:
// WASD/Flechas: Mover cámara
// Q/E: Rotar cámara
// +/-: Zoom
// F: Toggle debug | R: Reset cámara
// ESC: Salir

use ry_gfx::backend_sdl2::Sdl2Backend;
use ry_gfx::camera::Camera2D;
use ry_gfx::ColorRydit;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

struct TestEntity {
    x: f32, y: f32,
    w: u32, h: u32,
    hp: f32, max_hp: f32,
    name: &'static str,
    color: Color,
    vx: f32, vy: f32,
}

fn crear_textura<'a>(
    font: &Option<ry_gfx::sdl2_ffi::FontFFI>,
    texto: &str,
    r: u8, g: u8, b: u8,
    tc: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
) -> Option<sdl2::render::Texture<'a>> {
    if let Some(f) = font {
        if let Ok(sp) = f.render_text_blended(texto, r, g, b) {
            unsafe {
                let s = sdl2::surface::Surface::from_ll(sp as *mut sdl2::sys::SDL_Surface);
                if let Ok(t) = tc.create_texture_from_surface(&s) {
                    return Some(std::mem::transmute(t));
                }
            }
        }
    }
    None
}

fn draw_health_bar_world(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    camera: &Camera2D,
    sw: i32, sh: i32,
    wx: f32, wy: f32,
    ew: f32, eh: f32,
    hp: f32, mhp: f32,
) {
    let (sx, sy) = camera.world_to_screen(wx, wy, sw, sh);
    let bw = (ew * camera.zoom).max(30.0);
    let bh = 6i32;
    let off = (eh * camera.zoom) + 10.0;
    let by = sy as i32 - off as i32;
    let ratio = (hp / mhp).clamp(0.0, 1.0);

    canvas.set_draw_color(Color::RGBA(20, 20, 20, 200));
    let _ = canvas.fill_rect(Rect::new(sx, by, bw as u32, bh as u32));

    let fc = if ratio > 0.5 { Color::RGB(50, 200, 50) }
        else if ratio > 0.25 { Color::RGB(220, 200, 50) }
        else { Color::RGB(220, 50, 50) };
    canvas.set_draw_color(fc);
    let fw = (bw * ratio).max(1.0);
    let _ = canvas.fill_rect(Rect::new(sx, by, fw as u32, bh as u32));

    canvas.set_draw_color(Color::RGB(80, 80, 80));
    let _ = canvas.draw_rect(Rect::new(sx, by, bw as u32, bh as u32));
}

fn draw_grid(c: &mut sdl2::render::Canvas<sdl2::video::Window>, cam: &Camera2D) {
    c.set_draw_color(Color::RGB(40, 40, 50));
    for x in (0..2400).step_by(100) {
        let (sx, st) = cam.world_to_screen(x as f32, 0.0, 1280, 720);
        let (_, sb) = cam.world_to_screen(x as f32, 1800.0, 1280, 720);
        let h = (sb - st).unsigned_abs();
        if h > 0 && h < 5000 { let _ = c.fill_rect(Rect::new(sx, st.min(sb), 1, h)); }
    }
    for y in (0..1800).step_by(100) {
        let (sl, sy) = cam.world_to_screen(0.0, y as f32, 1280, 720);
        let (sr, _) = cam.world_to_screen(2400.0, y as f32, 1280, 720);
        let w = (sr - sl).unsigned_abs();
        if w > 0 && w < 5000 { let _ = c.fill_rect(Rect::new(sl.min(sr), sy, w, 1)); }
    }
}

fn draw_minimap(c: &mut sdl2::render::Canvas<sdl2::video::Window>, _cam: &Camera2D, ents: &[TestEntity], sz: i32) {
    let x = 1280 - sz - 12;
    let y = 720 - sz - 12;
    c.set_draw_color(Color::RGBA(30, 30, 40, 200));
    let _ = c.fill_rect(Rect::new(x, y, sz as u32, sz as u32));
    c.set_draw_color(Color::RGB(80, 80, 100));
    let _ = c.draw_rect(Rect::new(x, y, sz as u32, sz as u32));
    c.set_draw_color(Color::RGB(50, 200, 50));
    let _ = c.fill_rect(Rect::new(x + sz/2 - 3, y + sz/2 - 3, 6, 6));
    for e in ents {
        let ex = x + ((e.x / 2400.0) * sz as f32) as i32;
        let ey = y + ((e.y / 1800.0) * sz as f32) as i32;
        c.set_draw_color(e.color);
        let _ = c.fill_rect(Rect::new(ex - 2, ey - 2, 4, 4));
    }
}

fn main() -> Result<(), String> {
    println!("🛡️ RyDit - Demo HUD + Camara 2D");
    println!("WASD/Flechas: Mover | Q/E: Rotar | +/-: Zoom");
    println!("F: Debug | R: Reset | ESC: Salir\n");

    let mut backend = Sdl2Backend::new("Demo HUD + Camara 2D", 1280, 720)?;
    for p in &["/system/fonts/DroidSans.ttf", "/data/data/com.termux/files/usr/share/fonts/noto-sans/NotoSans-Regular.ttf"] {
        if std::path::Path::new(p).exists() {
            let _ = backend.load_font(p, 14);
            let _ = backend.load_font(p, 12);
            break;
        }
    }

    let mut camera = Camera2D::at(600.0, 450.0);
    camera.smooth = 0.08;
    camera.set_bounds(0.0, 0.0, 2400.0, 1800.0);

    let mut entities = vec![
        TestEntity { x: 300.0, y: 300.0, w: 40, h: 50, hp: 100.0, max_hp: 100.0, name: "Heroe", color: Color::RGB(50,200,50), vx: 30.0, vy: 20.0 },
        TestEntity { x: 700.0, y: 500.0, w: 35, h: 40, hp: 75.0, max_hp: 100.0, name: "Enemigo 1", color: Color::RGB(200,50,50), vx: -25.0, vy: 15.0 },
        TestEntity { x: 1100.0, y: 400.0, w: 50, h: 60, hp: 200.0, max_hp: 200.0, name: "Boss", color: Color::RGB(200,100,50), vx: 15.0, vy: -30.0 },
        TestEntity { x: 1500.0, y: 700.0, w: 30, h: 35, hp: 15.0, max_hp: 80.0, name: "NPC Herido", color: Color::RGB(100,100,200), vx: -20.0, vy: -10.0 },
        TestEntity { x: 1800.0, y: 900.0, w: 45, h: 55, hp: 150.0, max_hp: 150.0, name: "Guardian", color: Color::RGB(200,200,50), vx: 35.0, vy: -25.0 },
    ];

    let mut show_debug = true;
    let mut score: i64 = 12500;
    let start_time = std::time::Instant::now();
    let mut fc: u32 = 0;
    let mut fa: f32 = 0.0;
    let mut cfps: f32 = 60.0;
    let mut lfu = std::time::Instant::now();

    let tc = &backend.canvas.texture_creator();
    let mut txt_debug: Option<sdl2::render::Texture<'static>> = None;
    let mut txt_stats: Option<sdl2::render::Texture<'static>> = None;
    let mut txt_names: [Option<sdl2::render::Texture<'static>>; 5] = Default::default();
    let mut lcf: u64 = 0;
    let mut running = true;

    'run: loop {
        let fs = std::time::Instant::now();
        for ev in backend.event_pump.poll_iter() {
            match ev {
                Event::Quit{..} | Event::KeyDown{keycode: Some(Keycode::Escape),..} => { running=false; break 'run; }
                Event::KeyDown{keycode: Some(k), repeat: false,..} => match k {
                    Keycode::F => show_debug = !show_debug,
                    Keycode::R => { camera = Camera2D::at(600.0, 450.0); camera.smooth = 0.08; camera.set_bounds(0.0, 0.0, 2400.0, 1800.0); }
                    _ => {}
                }
                _ => {}
            }
        }

        let ks = sdl2::keyboard::KeyboardState::new(&backend.event_pump);
        let spd = 300.0;
        if ks.is_scancode_pressed(sdl2::keyboard::Scancode::W) || ks.is_scancode_pressed(sdl2::keyboard::Scancode::Up) { camera.target_y = Some(camera.y - spd); }
        if ks.is_scancode_pressed(sdl2::keyboard::Scancode::S) || ks.is_scancode_pressed(sdl2::keyboard::Scancode::Down) { camera.target_y = Some(camera.y + spd); }
        if ks.is_scancode_pressed(sdl2::keyboard::Scancode::A) || ks.is_scancode_pressed(sdl2::keyboard::Scancode::Left) { camera.target_x = Some(camera.x - spd); }
        if ks.is_scancode_pressed(sdl2::keyboard::Scancode::D) || ks.is_scancode_pressed(sdl2::keyboard::Scancode::Right) { camera.target_x = Some(camera.x + spd); }
        if ks.is_scancode_pressed(sdl2::keyboard::Scancode::Q) { camera.rotation -= 60.0; }
        if ks.is_scancode_pressed(sdl2::keyboard::Scancode::E) { camera.rotation += 60.0; }
        if ks.is_scancode_pressed(sdl2::keyboard::Scancode::KpPlus) || ks.is_scancode_pressed(sdl2::keyboard::Scancode::Equals) { camera.zoom = (camera.zoom + 0.5).min(5.0); }
        if ks.is_scancode_pressed(sdl2::keyboard::Scancode::KpMinus) || ks.is_scancode_pressed(sdl2::keyboard::Scancode::Minus) { camera.zoom = (camera.zoom - 0.5).max(0.2); }
        if let (Some(tx), Some(ty)) = (camera.target_x, camera.target_y) {
            camera.follow_smooth(tx, ty, camera.smooth);
        }

        for e in &mut entities {
            e.x += e.vx; e.y += e.vy;
            if e.x < 50.0 || e.x > 2350.0 { e.vx *= -1.0; e.x = e.x.clamp(50.0, 2350.0); }
            if e.y < 50.0 || e.y > 1750.0 { e.vy *= -1.0; e.y = e.y.clamp(50.0, 1750.0); }
            let t = start_time.elapsed().as_secs_f32();
            e.hp = (e.max_hp * (0.3 + 0.7 * (0.5 + 0.5 * (t * 0.5 + e.x * 0.001).sin()))).clamp(0.0, e.max_hp);
        }
        score += 1;

        backend.canvas.set_draw_color(Color::RGB(18, 18, 24));
        backend.canvas.clear();
        draw_grid(&mut backend.canvas, &camera);

        for e in &entities {
            let (sx, sy) = camera.world_to_screen(e.x, e.y, 1280, 720);
            let sw = (e.w as f32 * camera.zoom) as u32;
            let sh = (e.h as f32 * camera.zoom) as u32;
            backend.canvas.set_draw_color(e.color);
            let _ = backend.canvas.fill_rect(Rect::new(sx, sy, sw.max(4), sh.max(4)));
        }

        for (i, e) in entities.iter().enumerate() {
            draw_health_bar_world(&mut backend.canvas, &camera, 1280, 720, e.x, e.y, e.w as f32, e.h as f32, e.hp, e.max_hp);
            let (sx, sy) = camera.world_to_screen(e.x, e.y, 1280, 720);
            let off = (e.h as f32 * camera.zoom) + 10.0;
            let ny = sy as i32 - off as i32 - 16;
            if lcf % 30 == 0 {
                txt_names[i] = crear_textura(&backend.font, e.name, 230, 230, 240, tc).map(|t| unsafe { std::mem::transmute(t) });
            }
            if let Some(ref tex) = txt_names[i] {
                let qw = tex.query().width;
                backend.canvas.copy(tex, None, Rect::new(sx - (qw as i32)/2, ny, qw, 14));
            }
        }

        if show_debug {
            if lcf % 30 == 0 {
                let el = start_time.elapsed().as_secs_f32();
                let txt = format!("FPS: {:.0} | Cam: ({:.0},{:.0}) Zoom: {:.2}x Rot: {:.0}° | Ent: {} | {:.0}s",
                    cfps, camera.x, camera.y, camera.zoom, camera.rotation, entities.len(), el);
                txt_debug = crear_textura(&backend.font, &txt, 200, 200, 220, tc).map(|t| unsafe { std::mem::transmute(t) });
            }
            if let Some(ref tex) = txt_debug {
                let qw = tex.query().width;
                backend.canvas.set_draw_color(Color::RGBA(0, 0, 0, 180));
                let _ = backend.canvas.fill_rect(Rect::new(10, 10, qw + 16, 22));
                backend.canvas.copy(tex, None, Rect::new(14, 12, qw, 16));
            }
        }

        if lcf % 30 == 0 {
            let el = start_time.elapsed().as_secs_f32();
            let m = (el / 60.0) as i32;
            let s = (el % 60.0) as i32;
            let txt = format!("Score: {} | {:02}:{:02} | Nivel 1", score, m, s);
            txt_stats = crear_textura(&backend.font, &txt, 255, 220, 100, tc).map(|t| unsafe { std::mem::transmute(t) });
        }
        if let Some(ref tex) = txt_stats {
            let qw = tex.query().width;
            backend.canvas.set_draw_color(Color::RGBA(0, 0, 0, 140));
            let _ = backend.canvas.fill_rect(Rect::new(1280 - (qw as i32) - 22, 10, qw + 16, 22));
            backend.canvas.copy(tex, None, Rect::new(1280 - (qw as i32) - 18, 12, qw, 16));
        }

        draw_minimap(&mut backend.canvas, &camera, &entities, 150);
        backend.canvas.present();

        fc += 1; fa += fs.elapsed().as_secs_f32();
        if lfu.elapsed().as_secs_f32() >= 0.5 {
            cfps = fc as f32 / fa; fc = 0; fa = 0.0; lfu = std::time::Instant::now();
        }
        lcf += 1;
        let fe = fs.elapsed();
        if fe < std::time::Duration::from_millis(16) { std::thread::sleep(std::time::Duration::from_millis(16) - fe); }
    }
    println!("\n✅ Demo cerrado");
    Ok(())
}
