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
    format!("{}/.config/clock/config.toml", home)
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



fn calculate_window_size(font_size: f32, padding: f32) -> (usize, usize) {
    // Calcular tamaño basado en el texto completo con nanosegundos: "HH:MM:SS.999"
    let char_width = font_size * 0.65; // Mejor aproximación para caracteres monoespaciados
    let text_length = 12.0; // "HH:MM:SS.999" = 12 caracteres
    let width = (char_width * text_length + padding * 2.0) as usize;
    let height = (font_size * 1.4 + padding * 2.0) as usize; // Más altura para mejor renderizado
    
    // Asegurar mínimos más generosos
    let width = width.max(250).min(720);
    let height = height.max(350).min(1080);
    
    (width, height)
}

// Función removida ya que minifb no soporta posicionamiento de ventana

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
            "/System/Library/Fonts/Helvetica.ttc", 
            "/System/Library/Fonts/Arial.ttf",
            "/System/Library/Fonts/Courier.ttc",
            "/System/Library/Fonts/Monaco.ttf",
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
        (400, 200) // Tamaño por defecto más grande si auto_resize está deshabilitado
    };

    let mut window = Window::new(
        "Clock - ESC para salir | B = fondo | F = fuente",
        initial_width,
        initial_height,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Stretch,
            ..Default::default()
        },
    ).expect("No se pudo crear la ventana");

    window.limit_update_rate(Some(Duration::from_micros(1_000_000 / 60))); // 60 FPS es suficiente

    let font_data = find_system_font();
    let font = Font::try_from_vec(font_data).expect("No se pudo cargar ninguna fuente");

    let mut width = initial_width;
    let mut height = initial_height;
    let mut buffer = vec![0u32; width * height];

		// Definir paletas de colores
		let background_colors = vec![
				// Oscuros / Backgrounds principales
				0x000000, // Negro absoluto
				0xFFFFFF, // Blanco
				0x1E1E2E, // Catppuccin - Mocha
				0x2E3440, // Nord - Polar Night
				0x0F1419, // Base ultra oscuro
				0x282C34, // One Dark
				0x1A1B23, // Gris profundo
				0x2D3748, // Tailwind Slate 800
				0x1A202C, // Tailwind Gray 900
				0x0D1117, // GitHub dark
				0x1C1C1C, // Gruvbox dark0
				0x3B4252, // Nord - dark gray
				0x112240, // Oceanic dark blue
				0x002B36, // Solarized base03
				0x011627, // Night Owl background
				0x191724, // Rose Pine - base
				0x1F2937, // Tailwind Gray 800
				0x121212, // Material dark
		];

		// Colores de fuente
		let font_colors = vec![
				0xFFFFFF, // Blanco puro
				0xE06C75, // One Dark - rojo
				0xFF5555, // Rojo suave
				0x50FA7B, // Verde neón
				0x8BE9FD, // Cyan claro
				0xFFB86C, // Naranja pastel
				0xFF79C6, // Magenta claro
				0xBD93F9, // Púrpura neón
				0xF1FA8C, // Amarillo pastel
				0x6272A4, // Gris azulado
				0x44475A, // Gris oscuro
				0xFF6E6E, // Rojo coral
				0x69FF94, // Verde menta
				0x92A5FF, // Azul lavanda
				0xFFD93D, // Amarillo dorado
				0xFF9FF3, // Rosa claro
				0xFAB387, // Catppuccin Peach
				0xA6E3A1, // Catppuccin Green
				0x89DCEB, // Catppuccin Sky
				0xF38BA8, // Catppuccin Red
				0xCBA6F7, // Catppuccin Mauve
				0xF2CDCD, // Rosado pálido
				0xD3869B, // Gruvbox - Purple
				0xB8BB26, // Gruvbox - Green
				0xFABD2F, // Gruvbox - Yellow
				0x83A598, // Gruvbox - Aqua
				0x8FBCBB, // Nord - Frost
				0x88C0D0, // Nord - Blue
				0x5E81AC, // Nord - Indigo
				0xBF616A, // Nord - Red
				0xD08770, // Nord - Orange
				0xEBCB8B, // Nord - Yellow
				0xA3BE8C, // Nord - Green
				0xB48EAD, // Nord - Purple
				];

    let mut background_index = 0;
    let mut font_index = 0;
    
    // Variables para controlar el debounce de las teclas
    let mut b_key_pressed = false;
    let mut f_key_pressed = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (w, h) = window.get_size();
        if w != width || h != height {
            width = w;
            height = h;
            buffer = vec![0u32; width * height];
        }

        let now = Local::now();
        // Cambiar formato para mostrar 3 dígitos de milisegundos en lugar de nanosegundos
        let time_str = format!("{}", now.format("%H:%M:%S.%3f"));

        // Manejar entrada de teclado con debounce
        let b_key_down = window.is_key_down(Key::B);
        let f_key_down = window.is_key_down(Key::F);
        
        // Cambiar color de fondo (tecla B)
        if b_key_down && !b_key_pressed {
            background_index = (background_index + 1) % background_colors.len();
            println!("Fondo cambiado a índice: {} (0x{:06X})", background_index, background_colors[background_index]);
        }
        b_key_pressed = b_key_down;
        
        // Cambiar color de fuente (tecla F)
        if f_key_down && !f_key_pressed {
            font_index = (font_index + 1) % font_colors.len();
            println!("Fuente cambiada a índice: {} (0x{:06X})", font_index, font_colors[font_index]);
        }
        f_key_pressed = f_key_down;

        // Limpiar completamente el buffer con el color de fondo actual
        let bg_color = background_colors[background_index];
        for pixel in buffer.iter_mut() {
            *pixel = bg_color;
        }
        
        // Dibujar el texto con el color de fuente actual
        draw_text_centered(&time_str, &font, scale, &mut buffer, width, height, font_colors[font_index], bg_color);

        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

fn draw_text_centered(text: &str, font: &Font, scale: Scale, buffer: &mut [u32], width: usize, height: usize, font_color: u32, bg_color: u32) {
    let v_metrics = font.v_metrics(scale);
    
    // SOLUCIÓN AL TEMBLOR: Usar un ancho fijo basado en el texto más largo posible
    // Para "HH:MM:SS.999" = 12 caracteres, usamos un ancho fijo
    let fixed_text_width = scale.x * 0.65 * 12.0; // 12 caracteres con factor 0.65
    
    // Centrar horizontalmente usando el ancho fijo (evita temblor)
    let start_x = ((width as f32 - fixed_text_width) / 2.0).max(0.0);
    
    // Centrar verticalmente basado en la altura real de la ventana
    let start_y = (height as f32 / 2.0) + (v_metrics.ascent / 2.0);
    
    let glyphs = font.layout(text, scale, point(start_x, start_y));

    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|x, y, v| {
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;

                if x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height {
                    let idx = y as usize * width + x as usize;
                    let alpha = v;
                    
                    if alpha > 0.01 { // Umbral mínimo para evitar píxeles fantasma
                        // Extraer componentes RGB del color de fuente y fondo
                        let font_r = ((font_color >> 16) & 0xFF) as f32;
                        let font_g = ((font_color >> 8) & 0xFF) as f32;
                        let font_b = (font_color & 0xFF) as f32;
                        
                        let bg_r = ((bg_color >> 16) & 0xFF) as f32;
                        let bg_g = ((bg_color >> 8) & 0xFF) as f32;
                        let bg_b = (bg_color & 0xFF) as f32;
                        
                        // Alpha blending correcto
                        let final_r = ((font_r * alpha + bg_r * (1.0 - alpha)) as u32).min(255);
                        let final_g = ((font_g * alpha + bg_g * (1.0 - alpha)) as u32).min(255);
                        let final_b = ((font_b * alpha + bg_b * (1.0 - alpha)) as u32).min(255);
                        
                        buffer[idx] = (final_r << 16) | (final_g << 8) | final_b;
                    }
                }
            });
        }
    }
}
