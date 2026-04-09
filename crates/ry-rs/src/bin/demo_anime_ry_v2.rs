// demo_anime_ry_v2.rs
// Snake controlable con WASD/Flechas + Manzanas + Bombas + HUD + Minimap
//
// Controles:
// WASD / Flechas: Mover snake
// R: Reiniciar
// F: Toggle debug overlay
// ESC: Salir

use ry_gfx::backend_sdl2::Sdl2Backend;
use ry_gfx::camera::Camera2D;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::VecDeque;

const MAP_W: f32 = 2400.0;
const MAP_H: f32 = 1800.0;
const SEG_SIZE: i32 = 24;
const APPLE_R: i32 = 10;
const BOMB_R: i32 = 12;
const APPLE_COUNT: usize = 15;
const BOMB_COUNT: usize = 20;
const ENTITY_COUNT: usize = 8;
const MOVE_MS: u64 = 100;

#[derive(Clone, Copy)]
struct V2 { x: f32, y: f32 }

struct Apple { pos: V2, collected: bool }
struct Bomb { pos: V2, exploded: bool }
struct MovingEnt { pos: V2, vel: V2, color: Color, sz: i32, name: &'static str }

struct Snake {
    segs: VecDeque<V2>,
    dir: V2,
    ndir: V2,
    alive: bool,
    len: usize,
}

impl Snake {
    fn new(x: f32, y: f32) -> Self {
        let mut s = VecDeque::new();
        for i in 0..3 { s.push_back(V2 { x: x - i as f32 * SEG_SIZE as f32, y }); }
        Self { segs: s, dir: V2 { x: 1.0, y: 0.0 }, ndir: V2 { x: 1.0, y: 0.0 }, alive: true, len: 3 }
    }
    fn set_dir(&mut self, dx: f32, dy: f32) {
        if dx != -self.dir.x || dy != -self.dir.y { self.ndir = V2 { x: dx, y: dy }; }
    }
    fn update(&mut self) {
        if !self.alive { return; }
        self.dir = self.ndir;
        let h = self.segs.front().unwrap();
        let nx = V2 { x: h.x + self.dir.x * SEG_SIZE as f32, y: h.y + self.dir.y * SEG_SIZE as f32 };
        if nx.x < 0.0 || nx.x >= MAP_W || nx.y < 0.0 || nx.y >= MAP_H { self.alive = false; return; }
        for seg in &self.segs {
            if (seg.x - nx.x).abs() < 4.0 && (seg.y - nx.y).abs() < 4.0 { self.alive = false; return; }
        }
        self.segs.push_front(nx);
        while self.segs.len() > self.len { self.segs.pop_back(); }
    }
    fn grow(&mut self) { self.len += 1; }
    fn head_pos(&self) -> Option<V2> { self.segs.front().copied() }
}

fn crear_textura<'a>(f: &Option<ry_gfx::sdl2_ffi::FontFFI>, t: &str, r: u8, g: u8, b: u8,
    tc: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Option<sdl2::render::Texture<'a>> {
    if let Some(font) = f {
        if let Ok(sp) = font.render_text_blended(t, r, g, b) {
            unsafe {
                let s = sdl2::surface::Surface::from_ll(sp as *mut sdl2::sys::SDL_Surface);
                if let Ok(tex) = tc.create_texture_from_surface(&s) { return Some(std::mem::transmute(tex)); }
            }
        }
    }
    None
}

fn rng(min: f32, max: f32) -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as f32;
    min + (seed.sin() * 10000.0).fract().abs() * (max - min)
}

fn main() -> Result<(), String> {
    println!("🐍 RyDit - Snake Anime v2");
    println!("WASD/Flechas: Mover | R: Reiniciar | F: Debug | ESC: Salir\n");

    let mut backend = Sdl2Backend::new("Snake Anime v2", 1280, 720)?;
    for p in &["/system/fonts/DroidSans.ttf", "/data/data/com.termux/files/usr/share/fonts/noto-sans/NotoSans-Regular.ttf"] {
        if std::path::Path::new(p).exists() { let _ = backend.load_font(p, 14); let _ = backend.load_font(p, 12); break; }
    }

    let mut camera = Camera2D::at(600.0, 450.0);
    camera.smooth = 0.12;
    camera.set_bounds(0.0, 0.0, MAP_W, MAP_H);

    let ecolors = [(Color::RGB(200,50,50),"Enemigo Rojo"),(Color::RGB(50,200,50),"Enemigo Verde"),
        (Color::RGB(50,50,200),"Enemigo Azul"),(Color::RGB(200,200,50),"Enemigo Amarillo"),
        (Color::RGB(200,50,200),"Enemigo Rosa"),(Color::RGB(50,200,200),"Enemigo Cyan"),
        (Color::RGB(200,100,50),"Enemigo Naranja"),(Color::RGB(100,50,200),"Enemigo Violeta")];
    let mut entities: Vec<MovingEnt> = Vec::new();
    for i in 0..ENTITY_COUNT {
        entities.push(MovingEnt { pos: V2 { x: rng(100.0, MAP_W-100.0), y: rng(100.0, MAP_H-100.0) },
            vel: V2 { x: rng(-40.0, 40.0), y: rng(-40.0, 40.0) }, color: ecolors[i].0,
            sz: 20 + (i % 3) as i32 * 5, name: ecolors[i].1 });
    }

    let mut apples: Vec<Apple> = (0..APPLE_COUNT).map(|_| Apple { pos: V2 { x: rng(50.0, MAP_W-50.0), y: rng(50.0, MAP_H-50.0) }, collected: false }).collect();
    let mut bombs: Vec<Bomb> = (0..BOMB_COUNT).map(|_| Bomb { pos: V2 { x: rng(100.0, MAP_W-100.0), y: rng(100.0, MAP_H-100.0) }, exploded: false }).collect();

    let mut snake = Snake::new(300.0, 300.0);
    let mut score: i64 = 0;
    let start = std::time::Instant::now();
    let mut last_move = std::time::Instant::now();
    let mut show_debug = false;
    let tc = &backend.canvas.texture_creator();
    let mut txt_debug: Option<sdl2::render::Texture<'static>> = None;
    let mut txt_stats: Option<sdl2::render::Texture<'static>> = None;
    let mut txt_go: Option<sdl2::render::Texture<'static>> = None;
    let mut lcf: u64 = 0;
    let mut running = true;

    'run: loop {
        let fs = std::time::Instant::now();
        let ks = sdl2::keyboard::KeyboardState::new(&backend.event_pump);
        for ev in backend.event_pump.poll_iter() {
            match ev {
                Event::Quit{..}|Event::KeyDown{keycode:Some(Keycode::Escape),..} => { running=false; break 'run; }
                Event::KeyDown{keycode:Some(Keycode::R),..} => {
                    snake=Snake::new(300.0,300.0); score=0; last_move=std::time::Instant::now();
                    apples.iter_mut().for_each(|a|a.collected=false);
                    bombs.iter_mut().for_each(|b|b.exploded=false); txt_go=None;
                }
                Event::KeyDown{keycode:Some(Keycode::F),..} => show_debug=!show_debug,
                Event::KeyDown{keycode:Some(k),..} if snake.alive => match k {
                    Keycode::W|Keycode::Up=>snake.set_dir(0.0,-1.0), Keycode::S|Keycode::Down=>snake.set_dir(0.0,1.0),
                    Keycode::A|Keycode::Left=>snake.set_dir(-1.0,0.0), Keycode::D|Keycode::Right=>snake.set_dir(1.0,0.0),
                    _=>{} },
                _=>{}
            }
        }

        if snake.alive && last_move.elapsed().as_millis() >= MOVE_MS as u128 {
            snake.update();
            last_move = std::time::Instant::now();
            if snake.alive {
                if let Some(hp) = snake.head_pos() {
                    for apple in &mut apples {
                        if !apple.collected && (hp.x-apple.pos.x).abs()<(SEG_SIZE/2+APPLE_R) as f32 && (hp.y-apple.pos.y).abs()<(SEG_SIZE/2+APPLE_R) as f32 {
                            apple.collected=true; snake.grow(); score+=10;
                        }
                    }
                    for bomb in &mut bombs {
                        if !bomb.exploded && (hp.x-bomb.pos.x).abs()<(SEG_SIZE/2+BOMB_R) as f32 && (hp.y-bomb.pos.y).abs()<(SEG_SIZE/2+BOMB_R) as f32 {
                            bomb.exploded=true; snake.alive=false;
                        }
                    }
                    for ent in &entities {
                        if (hp.x-ent.pos.x).abs()<(SEG_SIZE/2+ent.sz/2) as f32 && (hp.y-ent.pos.y).abs()<(SEG_SIZE/2+ent.sz/2) as f32 {
                            snake.alive=false;
                        }
                    }
                }
            }
        }

        for ent in &mut entities {
            ent.pos.x+=ent.vel.x*0.016; ent.pos.y+=ent.vel.y*0.016;
            if ent.pos.x<20.0||ent.pos.x>MAP_W-20.0 { ent.vel.x*=-1.0; ent.pos.x=ent.pos.x.clamp(20.0,MAP_W-20.0); }
            if ent.pos.y<20.0||ent.pos.y>MAP_H-20.0 { ent.vel.y*=-1.0; ent.pos.y=ent.pos.y.clamp(20.0,MAP_H-20.0); }
        }

        if let Some(hp) = snake.head_pos() {
            camera.target_x=Some(hp.x); camera.target_y=Some(hp.y);
            camera.follow_smooth(hp.x, hp.y, camera.smooth);
        }

        lcf+=1;
        if lcf%20==0 {
            let el=start.elapsed().as_secs_f32(); let m=(el/60.0) as i32; let s=(el%60.0) as i32;
            let ac=apples.iter().filter(|a|a.collected).count();
            txt_stats=crear_textura(&backend.font,&format!("🍎 {} | 💣 {} | Score: {} | {:02}:{:02}",ac,BOMB_COUNT-bombs.iter().filter(|b|b.exploded).count(),score,m,s),255,220,100,tc).map(|t|unsafe{std::mem::transmute(t)});
            if !snake.alive&&txt_go.is_none() { txt_go=crear_textura(&backend.font,"💥 GAME OVER — R: Reiniciar | ESC: Salir",255,80,80,tc).map(|t|unsafe{std::mem::transmute(t)}); }
            if show_debug { if let Some(hp)=snake.head_pos() { txt_debug=crear_textura(&backend.font,&format!("Snake: {} seg | Pos: ({:.0},{:.0}) | Cam: ({:.0},{:.0})",snake.len,hp.x,hp.y,camera.x,camera.y),200,200,220,tc).map(|t|unsafe{std::mem::transmute(t)}); } }
        }

        backend.canvas.set_draw_color(Color::RGB(18,18,24)); backend.canvas.clear();
        draw_grid(&mut backend.canvas,&camera);

        let (bx,by)=camera.world_to_screen(0.0,0.0,1280,720); let (bx2,by2)=camera.world_to_screen(MAP_W,MAP_H,1280,720);
        backend.canvas.set_draw_color(Color::RGB(100,100,120));
        let _=backend.canvas.draw_rect(Rect::new(bx,by,(bx2-bx).unsigned_abs(),(by2-by).unsigned_abs()));

        for apple in &apples { if apple.collected{continue;} let (sx,sy)=camera.world_to_screen(apple.pos.x,apple.pos.y,1280,720);
            backend.canvas.set_draw_color(Color::RGB(255,255,50)); let _=backend.canvas.fill_rect(Rect::new(sx-APPLE_R/2,sy-APPLE_R/2,APPLE_R as u32,APPLE_R as u32));
            backend.canvas.set_draw_color(Color::RGB(255,255,200)); let _=backend.canvas.fill_rect(Rect::new(sx-3,sy-3,6,6)); }

        for bomb in &bombs { let (sx,sy)=camera.world_to_screen(bomb.pos.x,bomb.pos.y,1280,720);
            if bomb.exploded { backend.canvas.set_draw_color(Color::RGBA(255,100,50,100)); let _=backend.canvas.fill_rect(Rect::new(sx-16,sy-16,32,32)); }
            else { backend.canvas.set_draw_color(Color::RGB(50,50,50)); let _=backend.canvas.fill_rect(Rect::new(sx-BOMB_R,sy-BOMB_R,BOMB_R as u32*2,BOMB_R as u32*2));
                backend.canvas.set_draw_color(Color::RGB(255,150,50)); let _=backend.canvas.fill_rect(Rect::new(sx-2,sy-BOMB_R-6,4,6));
                backend.canvas.set_draw_color(Color::RGB(255,255,255)); let _=backend.canvas.fill_rect(Rect::new(sx-5,sy-3,4,4)); let _=backend.canvas.fill_rect(Rect::new(sx+2,sy-3,4,4));
                backend.canvas.set_draw_color(Color::RGB(0,0,0)); let _=backend.canvas.fill_rect(Rect::new(sx-4,sy-2,2,2)); let _=backend.canvas.fill_rect(Rect::new(sx+3,sy-2,2,2)); } }

        for ent in &entities { let (sx,sy)=camera.world_to_screen(ent.pos.x,ent.pos.y,1280,720);
            backend.canvas.set_draw_color(ent.color); let _=backend.canvas.fill_rect(Rect::new(sx-ent.sz/2,sy-ent.sz/2,ent.sz as u32,ent.sz as u32));
            backend.canvas.set_draw_color(Color::WHITE); let _=backend.canvas.fill_rect(Rect::new(sx-2,sy-ent.sz/2-4,4,4)); }

        for (i,seg) in snake.segs.iter().enumerate() { let (sx,sy)=camera.world_to_screen(seg.x,seg.y,1280,720);
            let c=if i==0{Color::RGB(100,255,100)}else{let g=200-(i*5).min(100);Color::RGB(50,g as u8,50)};
            backend.canvas.set_draw_color(c); let _=backend.canvas.fill_rect(Rect::new(sx-SEG_SIZE/2+1,sy-SEG_SIZE/2+1,SEG_SIZE as u32-2,SEG_SIZE as u32-2));
            if i==0 { backend.canvas.set_draw_color(Color::WHITE);
                let (ox,oy)=match(snake.dir.x as i32,snake.dir.y as i32){(1,0)=>(4,-4),(-1,0)=>(-4,-4),(0,1)=>(0,4),(0,-1)=>(0,-8),_=>(4,-4)};
                let _=backend.canvas.fill_rect(Rect::new(sx+ox-3,sy+oy,3,3)); let _=backend.canvas.fill_rect(Rect::new(sx+ox+2,sy+oy,3,3)); } }

        if let Some(ref tex)=txt_stats { let qw=tex.query().width; backend.canvas.set_draw_color(Color::RGBA(0,0,0,140));
            let _=backend.canvas.fill_rect(Rect::new(1280-qw as i32-12,10,qw+16,24)); let _=backend.canvas.copy(tex,None,Rect::new(1280-qw as i32-8,12,qw,18)); }
        if show_debug { if let Some(ref tex)=txt_debug { let qw=tex.query().width; backend.canvas.set_draw_color(Color::RGBA(0,0,0,180));
            let _=backend.canvas.fill_rect(Rect::new(10,10,qw+16,22)); let _=backend.canvas.copy(tex,None,Rect::new(14,12,qw,16)); } }
        if let Some(ref tex)=txt_go { let qw=tex.query().width; let qh=tex.query().height;
            backend.canvas.set_draw_color(Color::RGBA(0,0,0,180));
            let _=backend.canvas.fill_rect(Rect::new(640-qw as i32/2-20,360-qh as i32/2-10,qw+40,qh+20));
            let _=backend.canvas.copy(tex,None,Rect::new(640-qw as i32/2,360-qh as i32/2,qw,qh)); }

        draw_minimap(&mut backend.canvas,&snake,&entities,&apples,&bombs);
        backend.canvas.present();
        let fe=fs.elapsed(); if fe<std::time::Duration::from_millis(16){std::thread::sleep(std::time::Duration::from_millis(16)-fe);}
    }
    println!("\n✅ Snake Anime v2 cerrado"); Ok(())
}

fn draw_grid(c:&mut sdl2::render::Canvas<sdl2::video::Window>,cam:&Camera2D){
    c.set_draw_color(Color::RGB(40,40,50));
    for x in (0..2400).step_by(100){let(sx,st)=cam.world_to_screen(x as f32,0.0,1280,720);let(_,sb)=cam.world_to_screen(x as f32,1800.0,1280,720);
        let h=(sb-st).unsigned_abs();if h>0&&h<5000{let _=c.fill_rect(Rect::new(sx,st.min(sb),1,h));}}
    for y in (0..1800).step_by(100){let(sl,sy)=cam.world_to_screen(0.0,y as f32,1280,720);let(sr,_)=cam.world_to_screen(2400.0,y as f32,1280,720);
        let w=(sr-sl).unsigned_abs();if w>0&&w<5000{let _=c.fill_rect(Rect::new(sl.min(sr),sy,w,1));}}}

fn draw_minimap(c:&mut sdl2::render::Canvas<sdl2::video::Window>,snake:&Snake,ents:&[MovingEnt],apples:&[Apple],bombs:&[Bomb]){
    let sz=150;let x=1280-sz-12;let y=720-sz-12;
    c.set_draw_color(Color::RGBA(30,30,40,200));let _=c.fill_rect(Rect::new(x,y,sz as u32,sz as u32));
    c.set_draw_color(Color::RGB(80,80,100));let _=c.draw_rect(Rect::new(x,y,sz as u32,sz as u32));
    let mx=|wx:f32|x+((wx/MAP_W)*sz as f32) as i32; let my=|wy:f32|y+((wy/MAP_H)*sz as f32) as i32;
    for a in apples{if a.collected{continue;}c.set_draw_color(Color::RGB(255,255,50));let _=c.fill_rect(Rect::new(mx(a.pos.x)-1,my(a.pos.y)-1,3,3));}
    for b in bombs{if b.exploded{continue;}c.set_draw_color(Color::RGB(255,50,50));let _=c.fill_rect(Rect::new(mx(b.pos.x)-2,my(b.pos.y)-2,4,4));}
    for e in ents{c.set_draw_color(e.color);let _=c.fill_rect(Rect::new(mx(e.pos.x)-2,my(e.pos.y)-2,4,4));}
    for(i,seg) in snake.segs.iter().enumerate(){let c2=if i==0{Color::RGB(100,255,100)}else{Color::RGB(50,200,50)};
        c.set_draw_color(c2);let _=c.fill_rect(Rect::new(mx(seg.x)-2,my(seg.y)-2,4,4));}}
