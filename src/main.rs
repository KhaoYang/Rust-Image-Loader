use std::env;
use std::path::Path;
use image::DynamicImage;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        eprintln!("Supported formats: PNG, JPEG, GIF, BMP, ICO, TIFF, WebP");
        eprintln!("Press ESC to close the window, or arrow keys to navigate multiple images");
        std::process::exit(1);
    }

    let image_paths: Vec<String> = args[1..].to_vec();
    let mut current_image_index = 0;

    let mut window = Window::new(
        "Rust Image Loader - Press ESC to exit, Arrow keys to navigate",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("Failed to create window: {}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_millis(16)));

    let mut current_buffer = vec![0u32; WIDTH * HEIGHT];
    let mut loaded_image: Option<DynamicImage> = None;

    if !image_paths.is_empty() {
        loaded_image = load_and_display_image(&image_paths[current_image_index], &mut current_buffer);
        if loaded_image.is_some() {
            println!("Loaded: {} ({}/{})", 
                image_paths[current_image_index], 
                current_image_index + 1, 
                image_paths.len()
            );
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut image_changed = false;
        
        if window.is_key_pressed(Key::Right, minifb::KeyRepeat::No) && image_paths.len() > 1 {
            current_image_index = (current_image_index + 1) % image_paths.len();
            image_changed = true;
        }
        
        if window.is_key_pressed(Key::Left, minifb::KeyRepeat::No) && image_paths.len() > 1 {
            current_image_index = if current_image_index == 0 { 
                image_paths.len() - 1 
            } else { 
                current_image_index - 1 
            };
            image_changed = true;
        }

        if image_changed {
            loaded_image = load_and_display_image(&image_paths[current_image_index], &mut current_buffer);
            if loaded_image.is_some() {
                println!("Loaded: {} ({}/{})", 
                    image_paths[current_image_index], 
                    current_image_index + 1, 
                    image_paths.len()
                );
            }
        }

        window.update_with_buffer(&current_buffer, WIDTH, HEIGHT).unwrap();
    }

    println!("Image viewer closed.");
}

fn load_and_display_image(path: &str, buffer: &mut Vec<u32>) -> Option<DynamicImage> {
    buffer.fill(0x000000); 

    match image::open(path) {
        Ok(img) => {
            println!("Successfully loaded: {}", path);
            println!("  Original dimensions: {}x{}", img.width(), img.height());
            
            let img_rgb = img.to_rgb8();
            let (orig_width, orig_height) = (img.width() as f32, img.height() as f32);
            
            let scale_x = WIDTH as f32 / orig_width;
            let scale_y = HEIGHT as f32 / orig_height;
            let scale = scale_x.min(scale_y);
            
            let new_width = (orig_width * scale) as u32;
            let new_height = (orig_height * scale) as u32;
            
            println!("  Display dimensions: {}x{} (scale: {:.2})", new_width, new_height, scale);
            
            let resized = img.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3);
            let resized_rgb = resized.to_rgb8();
            
            let offset_x = (WIDTH - new_width as usize) / 2;
            let offset_y = (HEIGHT - new_height as usize) / 2;
            
            for y in 0..new_height as usize {
                for x in 0..new_width as usize {
                    let pixel = resized_rgb.get_pixel(x as u32, y as u32);
                    let rgb = ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32);
                    
                    let buf_x = offset_x + x;
                    let buf_y = offset_y + y;
                    
                    if buf_x < WIDTH && buf_y < HEIGHT {
                        buffer[buf_y * WIDTH + buf_x] = rgb;
                    }
                }
            }
            
            Some(img)
        }
        Err(e) => {
            eprintln!("Failed to load {}: {}", path, e);
            
            buffer.fill(0x330000); 
            None
        }
    }
}
