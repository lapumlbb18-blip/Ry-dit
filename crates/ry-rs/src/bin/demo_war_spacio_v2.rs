// demo_war_spacio_v2.rs
// 🚀 War Spacio v2 — SDL2 + Audio Procedural + Ventana de Gracia
//
// cargo run --bin demo_war_spacio_v2 --release
//
// Controles:
//   WASD/Flechas: Mover | ESPACIO: Disparar | R: Reiniciar | ESC: Salir
//   M: Toggle Música | +/-: Volumen

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Instant;

use ry_gfx::audio_mixer::{AudioBus, AudioMixer};
use ry_gfx::backend_sdl2::Sdl2Backend;
use ry_gfx::gpu_particles::ParticleSystem;
use ry_gfx::sdl2_helpers::*;
use events_ry::{InputEvent, InputManager, Key};

const W: u32 = 900;
const H: u32 = 600;
const PSPEED: f32 = 300.0;
const BSPEED: f32 = 500.0;
const COLS: usize = 8;
const ROWS: usize = 4;
const SP: f32 = 70.0;
const GRACE_MS: u64 = 150;

#[derive(Clone)]
struct Player { x: f32, y: f32, lives: u32, score: u32 }
#[derive(Clone)]
struct Enemy { x: f32, y: f32, vx: f32, vy: f32, alive: bool, mass: f32, row: usize, dt_: f32, diving: bool }
#[derive(Clone)]
struct Bullet { x: f32, y: f32, vy: f32, active: bool }

fn rc(a:f32,b:f32,c:f32,d:f32,e:f32,f:f32,g:f32,h:f32) -> bool {
    a<e+g && a+c>e && b<f+h && b+d>f
}

fn rf() -> f32 {
    use std::time::{SystemTime,UNIX_EPOCH};
    ((SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as f32).sin()*10000.0).fract()
}

fn spawn_enemies() -> Vec<Enemy> {
    let mut v = Vec::new();
    let sx = (W as f32 - COLS as f32 * SP)/2.0 + 20.0;
    for r in 0..ROWS { for c in 0..COLS {
        v.push(Enemy { x:sx+c as f32*SP, y:40.0+r as f32*50.0, vx:(rf()-0.5)*30.0, vy:(rf()-0.5)*20.0,
            alive:true, mass:20.0+r as f32*10.0, row:r, dt_:0.0, diving:false });
    }}
    v
}

/// Generar WAV procedural
fn gen_wav(path: &str, freq: f32, dur: f32, wave_type: &str) {
    use std::io::Write;
    let sr = 22050u32;
    let ns = (sr as f32 * dur) as usize;
    let mut data = Vec::with_capacity(ns * 2);
    for i in 0..ns {
        let t = i as f32 / sr as f32;
        let sample = match wave_type {
            "shoot" => {
                let env = (-t * 25.0).exp();
                (freq * t * 2.0 * std::f32::consts::PI).sin() * env * 0.5
            }
            "explosion" => {
                let n = rf() * 2.0 - 1.0;
                let env = if t < 0.02 { t / 0.02 } else { (-(t - 0.02) * 5.0).exp() };
                n * env * 0.5
            }
            "ambient" => {
                // Bucle de fondo: drones bajos modulados
                let f1 = freq;
                let f2 = freq * 1.5;
                let modulator = (0.5 * t * 2.0 * std::f32::consts::PI).sin();
                let sample = ((f1 + modulator * 20.0) * t * 2.0 * std::f32::consts::PI).sin() * 0.15
                    + (f2 * t * 2.0 * std::f32::consts::PI).sin() * 0.1;
                // Fade in/out para looping suave
                let fade = if t < 0.1 { t / 0.1 } else if t > dur - 0.1 { (dur - t) / 0.1 } else { 1.0 };
                sample * fade
            }
            "music" => {
                // Secuencia melódica tipo Megaman
                let notes = [261.63, 329.63, 392.0, 523.25, 392.0, 329.63, 261.63, 196.0];
                let note_dur = dur / notes.len() as f32;
                let idx = ((t / note_dur) as usize).min(notes.len() - 1);
                let f = notes[idx];
                let lt = t - idx as f32 * note_dur;
                let env = if lt < 0.02 { lt / 0.02 } else if lt > note_dur - 0.05 { (note_dur - lt) / 0.05 } else { 1.0 };
                (f * t * 2.0 * std::f32::consts::PI).sin() * env * 0.2
            }
            _ => 0.0,
        };
        let s16 = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
        data.extend_from_slice(&s16.to_le_bytes());
    }
    let ds = data.len() as u32;
    let fs = 36 + ds;
    if let Ok(mut f) = std::fs::File::create(path) {
        let _ = f.write_all(b"RIFF");
        let _ = f.write_all(&fs.to_le_bytes());
        let _ = f.write_all(b"WAVEfmt ");
        let _ = f.write_all(&16u32.to_le_bytes());
        let _ = f.write_all(&1u16.to_le_bytes());
        let _ = f.write_all(&1u16.to_le_bytes());
        let _ = f.write_all(&sr.to_le_bytes());
        let _ = f.write_all(&(sr * 2).to_le_bytes());
        let _ = f.write_all(&2u16.to_le_bytes());
        let _ = f.write_all(&16u16.to_le_bytes());
        let _ = f.write_all(b"data");
        let _ = f.write_all(&ds.to_le_bytes());
        let _ = f.write_all(&data);
    }
}

fn ensure_pulseaudio() {
    // Como Chrome/Firefox en Termux-X11: iniciar PulseAudio internamente
    use std::process::Command;

    // Si ya está corriendo, no hacer nada
    let check = Command::new("pulseaudio").arg("--check").status();
    if let Ok(s) = check { if s.success() { return; } }

    // Iniciar PulseAudio en background
    println!("🔊 Iniciando PulseAudio...");
    let _ = Command::new("pulseaudio")
        .args(&["--start", "--exit-idle-time=-1", "--daemonize=yes"])
        .status();

    // Esperar a que el socket aparezca
    for _ in 0..20 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let check = Command::new("pulseaudio").arg("--check").status();
        if let Ok(s) = check { if s.success() { println!("✅ PulseAudio listo"); return; } }
    }
    eprintln!("⚠️  PulseAudio no inició — audio puede fallar");
}

fn main() -> Result<(), String> {
    // Iniciar PulseAudio como lo hacen Chrome/Firefox en Termux-X11
    ensure_pulseaudio();

    println!("🚀 War Spacio v2 — Audio Procedural 8-bit");
    println!("   WASD: Mover | SPACE: Disparar | R: Reiniciar | ESC: Salir");

    let mut backend = Sdl2Backend::new("🚀 War Spacio v2 | Ry-Dit", W as i32, H as i32)?;
    let mut input = InputManager::new();
    input.input_state_mut().set_grace_ms(GRACE_MS);

    // ====== AUDIO ======
    let td = std::env::temp_dir().join("rydit_audio");
    let _ = std::fs::create_dir_all(&td);

    let shoot_wav = td.join("shoot.wav");
    let explosion_wav = td.join("explosion.wav");
    let ambient_wav = td.join("ambient.wav");
    let music_wav = td.join("music.wav");

    println!("🔊 Generando audio 8-bit...");
    gen_wav(shoot_wav.to_str().unwrap(), 120.0, 0.15, "shoot");
    gen_wav(explosion_wav.to_str().unwrap(), 80.0, 0.3, "explosion");
    gen_wav(ambient_wav.to_str().unwrap(), 65.0, 4.0, "ambient");
    gen_wav(music_wav.to_str().unwrap(), 220.0, 8.0, "music");

    let mut mixer = match AudioMixer::new() {
        Ok(mut m) => {
            let _ = m.load_sound("shoot", shoot_wav.to_str().unwrap());
            let _ = m.load_sound("explosion", explosion_wav.to_str().unwrap());
            let _ = m.load_sound("ambient", ambient_wav.to_str().unwrap());
            println!("✅ Audio listo (shoot 120Hz bajo + explosion + ambient loop)");
            Some(std::cell::RefCell::new(m))
        }
        Err(e) => {
            eprintln!("⚠️  Audio falló: {} — modo sin sonido", e);
            None
        }
    };

    // Iniciar ambient loop
    if let Some(ref mx) = mixer {
        let mut mx = mx.borrow_mut();
        let _ = mx.load_music(ambient_wav.to_str().unwrap());
        mx.set_bus_volume(AudioBus::Ambiente, 0.4);
        mx.play_music(-1);
        println!("🌊 Ambient loop activo");
    }

    let mut ps = ParticleSystem::new(); ps.global_gravity = 100.0;
    let mut p = Player { x:W as f32/2.0-15.0, y:H as f32-60.0, lives:3, score:0 };
    let mut enemies = spawn_enemies();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut hits = 0u32;
    let mut over = false;
    let mut frame = 0u64;
    let mut last_time = Instant::now();

    'run: loop {
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        backend.actualizar_input_unificado(&mut input);
        
        for event in input.poll_events() {
            match event {
                InputEvent::WindowCloseRequested => break 'run,
                InputEvent::KeyPressed { key: Key::Escape } => break 'run,
                InputEvent::KeyPressed { key: Key::R } => {
                    enemies = spawn_enemies(); bullets.clear(); ps.clear();
                    p.lives = 3; p.score = 0; hits = 0; over = false;
                }
                InputEvent::KeyPressed { key: Key::Space } if !over => {
                    bullets.push(Bullet{x:p.x+13.0,y:p.y,vy:-BSPEED,active:true});
                    if let Some(ref mx) = mixer { mx.borrow().play_sound("shoot"); }
                }
                InputEvent::KeyPressed { key: Key::M } => {
                    if let Some(ref mx) = mixer {
                        let mut mx = mx.borrow_mut();
                        if mx.is_music_playing() { mx.stop_music(); } else { mx.play_music(-1); }
                    }
                }
                _ => {}
            }
        }

        if over {
            backend.set_draw_color(Color::RGB(5,5,15)); backend.clear();
            backend.set_draw_color(Color::RED); let _=backend.fill_rect(Rect::new(W as i32/2-100,H as i32/2-30,200,60));
            backend.set_draw_color(Color::WHITE); let _=backend.fill_rect(Rect::new(W as i32/2-80,H as i32/2-10,160,40));
            backend.present();
            continue;
        }

        // Movimiento con ventana de gracia
        if input.is_key_down(Key::W) || input.is_key_down(Key::Up) { p.y -= PSPEED * dt; }
        if input.is_key_down(Key::S) || input.is_key_down(Key::Down) { p.y += PSPEED * dt; }
        if input.is_key_down(Key::A) || input.is_key_down(Key::Left) { p.x -= PSPEED * dt; }
        if input.is_key_down(Key::D) || input.is_key_down(Key::Right) { p.x += PSPEED * dt; }
        p.x=p.x.max(0.0).min(W as f32-30.0);p.y=p.y.max(0.0).min(H as f32-30.0);

        // Gravedad entre enemigos
        let en_clone: Vec<Enemy> = enemies.clone();
        for i in 0..en_clone.len() {
            if !en_clone[i].alive { continue; }
            for j in (i+1)..en_clone.len() {
                if !en_clone[j].alive { continue; }
                let dx=en_clone[j].x-en_clone[i].x; let dy=en_clone[j].y-en_clone[i].y;
                let d2=dx*dx+dy*dy; let d=d2.sqrt(); if d<10.0{continue;}
                let g=50.0; let f=g*en_clone[i].mass*en_clone[j].mass/d2;
                let ax=f*dx/(d*en_clone[i].mass); let ay=f*dy/(d*en_clone[i].mass);
                enemies[i].vx+=ax*dt; enemies[i].vy+=ay*dt;
                enemies[j].vx-=ax*dt; enemies[j].vy-=ay*dt;
            }
        }

        let mut killed = false;
        for e in enemies.iter_mut() {
            if !e.alive{continue;}
            e.x+=e.vx*dt;e.y+=e.vy*dt;
            e.vy+=(frame as f32*0.02).sin()*10.0*dt;
            if e.x<0.0||e.x>W as f32-30.0{e.vx=-e.vx;e.x=e.x.max(0.0).min(W as f32-30.0);}
            e.dt_+=dt;
            if !e.diving&&e.dt_>5.0+rf()*5.0{e.diving=true;e.vy=150.0;e.vx=(rf()-0.5)*200.0;e.dt_=0.0;}
            if e.diving&&e.y>H as f32-100.0{e.diving=false;e.vy=-50.0;}
        }

        for b in bullets.iter_mut() {
            if !b.active{continue;}
            b.y+=b.vy*dt;
            if b.y < -10.0||b.y>H as f32+10.0{b.active=false;continue;}
            for e in enemies.iter_mut() {
                if !e.alive{continue;}
                if rc(b.x,b.y,4.0,10.0,e.x,e.y,30.0,20.0) {
                    b.active=false;e.alive=false;p.score+=100*(e.row as u32+1);hits+=1;
                    killed = true;
                    break;
                }
            }
        }
        bullets.retain(|b|b.active);

        if killed {
            if let Some(ref mx) = mixer { mx.borrow().play_sound("explosion"); }
        }

        if enemies.iter().all(|e|!e.alive){enemies=spawn_enemies();}

        ps.update(dt);

        // ====== RENDER ======
        backend.set_draw_color(Color::RGB(5,5,15)); backend.clear();

        // Estrellas
        backend.set_draw_color(Color::RGB(80,80,120));
        for i in 0..80u32{
            let _=backend.fill_rect(Rect::new(((i*7919+13)%W) as i32,((i*6271+37)%H) as i32, 1, 1));
        }

        // Enemigos (ARRIBA)
        for e in enemies.iter() {
            if !e.alive{continue;}
            let spd=(e.vx*e.vx+e.vy*e.vy).sqrt();
            let c=velocity_color_sdl2(spd,300.0);
            backend.set_draw_color(Color::RGBA(c.r/3,c.g/3,c.b/3,60));
            let _=backend.fill_rect(Rect::new(e.x as i32-4,e.y as i32-4,38,28));
            backend.set_draw_color(c);
            let _=backend.fill_rect(Rect::new(e.x as i32,e.y as i32,30,20));
            backend.set_draw_color(Color::RGB(255,255,255));
            let _=backend.fill_rect(Rect::new(e.x as i32+6,e.y as i32+6,4,4));
            let _=backend.fill_rect(Rect::new(e.x as i32+20,e.y as i32+6,4,4));
        }

        // Balas
        backend.set_draw_color(Color::RGB(255,255,100));
        for b in bullets.iter(){if b.active{let _=backend.fill_rect(Rect::new(b.x as i32,b.y as i32,4,10));}}

        // Jugador (ABAJO)
        backend.set_draw_color(Color::RGB(0,200,255));
        let _=backend.fill_rect(Rect::new(p.x as i32+12,p.y as i32,6,30));
        let _=backend.fill_rect(Rect::new(p.x as i32+2,p.y as i32+15,26,15));
        backend.set_draw_color(Color::RGB(0,150,200));
        let _=backend.fill_rect(Rect::new(p.x as i32,p.y as i32+22,6,8));
        let _=backend.fill_rect(Rect::new(p.x as i32+24,p.y as i32+22,6,8));

        // Partículas
        for (_, emitter) in ps.emitters.iter() {
            for pt in emitter.particles.iter() {
                let c = pt.color;
                backend.set_draw_color(Color::RGBA(c.r, c.g, c.b, c.a));
                let _ = backend.fill_rect(Rect::new(pt.x as i32, pt.y as i32, pt.size as u32, pt.size as u32));
            }
        }

        // ====== HUD ======
        backend.set_draw_color(Color::RGBA(0,0,0,180));
        let _=backend.fill_rect(Rect::new(0,0,W,36));

        // Score
        backend.set_draw_color(Color::WHITE);
        let _=backend.fill_rect(Rect::new(10,10,120,12));
        // Lives
        backend.set_draw_color(Color::RGB(255,80,80));
        for l in 0..p.lives { let _=backend.fill_rect(Rect::new(150 + l as i32 * 20, 10, 14, 12)); }
        // Hits
        backend.set_draw_color(Color::RGB(255,255,100));
        let _=backend.fill_rect(Rect::new(250,10,(hits * 10).min(300) as u32, 12));

        // Audio indicator
        let audio_active = mixer.is_some();
        if audio_active {
            backend.set_draw_color(Color::RGB(0,255,100));
            let _=backend.fill_rect(Rect::new(500,10,6,12));
        } else {
            backend.set_draw_color(Color::RGB(255,50,50));
            let _=backend.fill_rect(Rect::new(500,10,6,12));
        }

        backend.present();frame+=1;
    }

    println!("\n✅ War Spacio v2 cerrado — Score: {} | Hits: {}",p.score,hits);
    Ok(())
}
