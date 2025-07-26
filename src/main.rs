use chrono::Local;
use minifb::{Key, Window, WindowOptions, ScaleMode};
use rusttype::{point, Font, Scale};
use std::fs;
use std::path::Path;
use std::time::Duration;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    settings: Settings,
}

#[derive(Deserialize)]
struct Settings {
    font_size: f32,
    #[serde(default = "default_padding")]
    padding: f32,
    #[serde(default = "default_auto_resize")]
    auto_resize: bool,
}

fn default_padding() -> f32 { 20.0 }
fn default_auto_resize() -> bool { true }

fn get_config_path() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/.config/big_clock/config.toml", home)
}

fn read_config() -> (f32, f32, bool) {
    let config_path = get_config_path();
    let content = fs::read_to_string(&config_path).unwrap_or_else(|_| {
        println!("Config no encontrado en {}, usando valores por defecto.", config_path);
        "[settings]\nfont_size = 80\npadding = 20.0\nauto_resize = true".to_string()
    });

    let config: Config = toml::from_str(&content).unwrap_or_else(|_| {
        println!("Error al leer config, usando valores por defecto.");
        Config { 
            settings: Settings { 
                font_size: 80.0,
                padding: 20.0,
                auto_resize: true,
            } 
        }
    });

    (config.settings.font_size, config.settings.padding, config.settings.auto_resize)
}

fn calculate_text_dimensions(text: &str, font: &Font, scale: Scale) -> (f32, f32) {
    let glyphs: Vec<_> = font.layout(text, scale, point(0.0, 0.0)).collect();
    
    if glyphs.is_empty() {
        return (100.0, 50.0);
    }
    
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    
    for glyph in &glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            min_x = min_x.min(bb.min.x as f32);
            max_x = max_x.max(bb.max.x as f32);
            min_y = min_y.min(bb.min.y as f32);
            max_y = max_y.max(bb.max.y as f32);
        }
    }
    
    let width = if min_x.is_finite() && max_x.is_finite() {
        (max_x - min_x).max(100.0)
    } else {
        100.0
    };
    
    let height = if min_y.is_finite() && max_y.is_finite() {
        (max_y - min_y).max(50.0)
    } else {
        50.0
    };
    
    (width, height)
}

fn calculate_window_size(font_size: f32, padding: f32) -> (usize, usize) {
    // Aproximación del tamaño de ventana basado en el tamaño de fuente
    // Para formato "HH:MM:SS" (8 caracteres)
    let char_width = font_size * 0.6; // Aproximación del ancho por carácter
    let width = (char_width * 8.5 + padding * 2.0) as usize;
    let height = (font_size * 1.2 + padding * 2.0) as usize;
    
    // Mínimos y máximos razonables
    let width = width.max(200).min(1920);
    let height = height.max(100).min(1080);
    
    (width, height)
}

fn find_system_font() -> Vec<u8> {
    let font_paths = if cfg!(target_os = "windows") {
        vec![
            "C:\\Windows\\Fonts\\consola.ttf",
            "C:\\Windows\\Fonts\\arial.ttf", 
            "C:\\Windows\\Fonts\\calibri.ttf",
            "C:\\Windows\\Fonts\\cour.ttf",
        ]
    } else if cfg!(target_os = "macos") {
        vec![
            "/System/Library/Fonts/Monaco.ttf",
            "/System/Library/Fonts/Helvetica.ttc", 
            "/System/Library/Fonts/Arial.ttf",
            "/System/Library/Fonts/Courier.ttc",
        ]
    } else {
        vec![
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
            "/usr/share/fonts/droid/DroidSansMono.ttf",
            "/usr/share/fonts/TTF/DejaVuSansMono.ttf", 
            "/usr/share/fonts/liberation/LiberationMono-Regular.ttf",
            "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf",
            "/usr/share/fonts/truetype/ubuntu/UbuntuMono-R.ttf",
        ]
    };

    for path in font_paths {
        if Path::new(path).exists() {
            match fs::read(path) {
                Ok(font_data) => {
                    println!("Usando fuente: {}", path);
                    return font_data;
                },
                Err(e) => {
                    println!("Error leyendo fuente {}: {}", path, e);
                    continue;
                }
            }
        }
    }

    // Si no encuentra ninguna fuente, terminar con error descriptivo
    //panic!("❌ No se pudo encontrar ninguna fuente del sistema compatible.\n\
     //      Instala una fuente monoespaciada como DejaVu Sans Mono o Liberation Mono.");
    // Fallback font embebida
    println!("No se encontró fuente del sistema, usando fuente por defecto");
    include_bytes!("../assets/Inter-Regular.otf").to_vec()
}

fn main() {
    let (font_size, padding, auto_resize) = read_config();
    let scale = Scale::uniform(font_size);
    
    // Calcular tamaño de ventana basado en el tamaño de fuente
    let (initial_width, initial_height) = if auto_resize {
        calculate_window_size(font_size, padding)
    } else {
        (800, 200) // Tamaño por defecto si auto_resize está deshabilitado
    };

    let mut window = Window::new(
        "Clock - ESC para salir",
        initial_width,
        initial_height,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Stretch,
            ..Default::default()
        },
    ).expect("No se pudo crear la ventana");

    window.limit_update_rate(Some(Duration::from_micros(1_000_000 / 144)));

    let font_data = find_system_font();
    let font = Font::try_from_vec(font_data).expect("No se pudo cargar ninguna fuente");

    let mut width = initial_width;
    let mut height = initial_height;
    let mut buffer = vec![0u32; width * height];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (w, h) = window.get_size();
        if w != width || h != height {
            width = w;
            height = h;
            buffer = vec![0u32; width * height];
        }

        let now = Local::now();
        let time_str = format!("{}", now.format("%H:%M:%S.%3f"));

        buffer.fill(0x000000);
        draw_text_centered(&time_str, &font, scale, &mut buffer, width, height);

        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

fn draw_text_centered(text: &str, font: &Font, scale: Scale, buffer: &mut [u32], width: usize, height: usize) {
    let v_metrics = font.v_metrics(scale);
    
    // Calcular dimensiones del texto
    let (text_width, text_height) = calculate_text_dimensions(text, font, scale);
    
    // Centrar el texto
    let start_x = ((width as f32 - text_width) / 2.0).max(0.0);
    let start_y = ((height as f32 - text_height) / 2.0).max(0.0) + v_metrics.ascent;
    
    let glyphs = font.layout(text, scale, point(start_x, start_y));

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
