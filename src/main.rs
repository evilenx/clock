use chrono::Local;
use minifb::{Key, Window, WindowOptions, ScaleMode};
use rusttype::{point, Font, Scale};
use std::fs;
use std::time::Duration;

use serde::Deserialize;

const DEFAULT_WIDTH: usize = 1000;
const DEFAULT_HEIGHT: usize = 240;

#[derive(Deserialize)]
struct Config {
    settings: Settings,
}

#[derive(Deserialize)]
struct Settings {
    font_size: f32,
}

fn read_config() -> f32 {
    let content = fs::read_to_string("config.toml").unwrap_or_else(|_| {
        println!("No se pudo leer config.toml, usando tamaño por defecto 80.");
        "[settings]\nfont_size = 80".to_string()
    });

    let config: Config = toml::from_str(&content).unwrap_or_else(|_| {
        println!("Error al leer config.toml, usando tamaño por defecto 80.");
        Config {
            settings: Settings { font_size: 80.0 },
        }
    });

    config.settings.font_size
}

fn main() {
    let font_size = read_config();
    let scale = Scale::uniform(font_size);

    let mut window = Window::new(
        "Reloj en Grande - ESC para salir",
        DEFAULT_WIDTH,
        DEFAULT_HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Stretch,
            ..Default::default()
        },
    )
    .expect("No se pudo crear la ventana");

    window.limit_update_rate(Some(Duration::from_micros(1_000_000 / 144)));

    let font_data = include_bytes!("/usr/share/fonts/droid/DroidSansMono.ttf") as &[u8];
    let font = Font::try_from_bytes(font_data).expect("No se pudo cargar la fuente");

    let mut width = DEFAULT_WIDTH;
    let mut height = DEFAULT_HEIGHT;
    let mut buffer = vec![0u32; width * height];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Redimensionar si es necesario
        let (w, h) = window.get_size();
            if w != width || h != height {
                width = w;
                height = h;
                buffer = vec![0u32; width * height];
        }

        let now = Local::now();
        let time_str = format!("{}", now.format("%H:%M:%S.%3f"));

        buffer.fill(0x000000); // fondo negro
        draw_text(&time_str, &font, scale, &mut buffer, width, height);

        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

fn draw_text(text: &str, font: &Font, scale: Scale, buffer: &mut [u32], width: usize, height: usize) {
    let v_metrics = font.v_metrics(scale);
    let glyphs = font.layout(text, scale, point(30.0, 100.0 + v_metrics.ascent));

    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|x, y, v| {
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;

                if x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height {
                    let idx = y as usize * width + x as usize;
                    let shade = (v * 255.0) as u32;
                    buffer[idx] = (shade << 16) | (shade << 8) | shade;
                }
            });
        }
    }
}

