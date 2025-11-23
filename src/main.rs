use crossterm::terminal;
use std::{thread, time::Duration};

// Constantes apenas para configuração da cena (não mais da tela)
const BACKGROUND_ASCII: char = ' '; // Mudei para espaço para ficar mais limpo, pode voltar para '.'
const DISTANCE_FROM_CAM: f32 = 100.0;
const K1: f32 = 40.0;
const INCREMENT_SPEED: f32 = 0.6;

// --- Códigos de Cores ANSI ---
#[allow(dead_code)]
const RESET: &str = "\x1b[0m";
#[allow(dead_code)]
const CYAN: &str = "\x1b[36m";
#[allow(dead_code)]
const MAGENTA: &str = "\x1b[35m";
#[allow(dead_code)]
const YELLOW: &str = "\x1b[33m";
// (Pode manter as outras cores aqui se quiser)

fn main() {
    let mut a: f32 = 0.0;
    let mut b: f32 = 0.0;
    let mut c: f32 = 0.0;

    print!("\x1b[2J"); // Limpa a tela inicial

    loop {
        // 1. Pega o tamanho atual do terminal
        // O unwrap() é seguro aqui pois se falhar em pegar o tamanho, o programa deve parar mesmo.
        let (cols, rows) = terminal::size().unwrap_or((80, 24));
        let width = cols as usize;
        let height = rows as usize;

        // 2. Cria buffers dinâmicos (Vec) baseados no tamanho atual
        let total_pixels = width * height;
        let mut z_buffer: Vec<f32> = vec![0.0; total_pixels];
        let mut char_buffer: Vec<char> = vec![BACKGROUND_ASCII; total_pixels];
        let mut color_buffer: Vec<&str> = vec![RESET; total_pixels];

        // --- Renderiza Cubos ---
        // Precisamos passar width/height para as funções agora
        
        let cube_width = 20.0;
        let horizontal_offset = -2.0 * cube_width;
        render_cube(cube_width, horizontal_offset, a, b, c, CYAN, width, height, &mut char_buffer, &mut z_buffer, &mut color_buffer);

        let cube_width = 10.0;
        let horizontal_offset = 1.0 * cube_width;
        render_cube(cube_width, horizontal_offset, a, b, c, MAGENTA, width, height, &mut char_buffer, &mut z_buffer, &mut color_buffer);

        let cube_width = 5.0;
        let horizontal_offset = 8.0 * cube_width;
        render_cube(cube_width, horizontal_offset, a, b, c, YELLOW, width, height, &mut char_buffer, &mut z_buffer, &mut color_buffer);

        // --- Desenha na Tela ---
        print!("\x1b[H"); // Home cursor
        
        let mut output = String::with_capacity(total_pixels * 10);
        
        for k in 0..total_pixels {
            // Quebra de linha ao atingir a largura
            if k % width == 0 && k != 0 {
                output.push('\n');
            }
            
            if char_buffer[k] != BACKGROUND_ASCII {
                output.push_str(color_buffer[k]);
                output.push(char_buffer[k]);
                output.push_str(RESET);
            } else {
                output.push(char_buffer[k]);
            }
        }
        println!("{}", output);

        a += 0.05;
        b += 0.05;
        c += 0.01;

        thread::sleep(Duration::from_millis(33)); // ~30 FPS (Aumentei um pouco o sleep para não piscar tanto ao redimensionar)
    }
}

fn render_cube(
    cube_width: f32,
    horizontal_offset: f32,
    a: f32, b: f32, c: f32,
    color: &'static str,
    width: usize, height: usize, // NOVO: Recebe dimensões
    char_buffer: &mut [char],
    z_buffer: &mut [f32],
    color_buffer: &mut [&'static str]
) {
    let mut cube_x = -cube_width;
    while cube_x < cube_width {
        let mut cube_y = -cube_width;
        while cube_y < cube_width {
            // Passamos width/height adiante
            calculate_for_surface(cube_x, cube_y, -cube_width, '@', horizontal_offset, a, b, c, color, width, height, char_buffer, z_buffer, color_buffer);
            calculate_for_surface(cube_width, cube_y, cube_x, '$', horizontal_offset, a, b, c, color, width, height, char_buffer, z_buffer, color_buffer);
            calculate_for_surface(-cube_width, cube_y, -cube_x, '~', horizontal_offset, a, b, c, color, width, height, char_buffer, z_buffer, color_buffer);
            calculate_for_surface(-cube_x, cube_y, cube_width, '#', horizontal_offset, a, b, c, color, width, height, char_buffer, z_buffer, color_buffer);
            calculate_for_surface(cube_x, -cube_width, -cube_y, ';', horizontal_offset, a, b, c, color, width, height, char_buffer, z_buffer, color_buffer);
            calculate_for_surface(cube_x, cube_width, cube_y, '+', horizontal_offset, a, b, c, color, width, height, char_buffer, z_buffer, color_buffer);
            
            cube_y += INCREMENT_SPEED;
        }
        cube_x += INCREMENT_SPEED;
    }
}

fn calculate_x(i: f32, j: f32, k: f32, a: f32, b: f32, c: f32) -> f32 {
    j * a.sin() * b.sin() * c.cos() - k * a.cos() * b.sin() * c.cos() +
    j * a.cos() * c.sin() + k * a.sin() * c.sin() + i * b.cos() * c.cos()
}

fn calculate_y(i: f32, j: f32, k: f32, a: f32, b: f32, c: f32) -> f32 {
    j * a.cos() * c.cos() + k * a.sin() * c.cos() -
    j * a.sin() * b.sin() * c.sin() + k * a.cos() * b.sin() * c.sin() -
    i * b.cos() * c.sin()
}

fn calculate_z(i: f32, j: f32, k: f32, a: f32, b: f32) -> f32 {
    k * a.cos() * b.cos() - j * a.sin() * b.cos() + i * b.sin()
}

fn calculate_for_surface(
    cube_x: f32, cube_y: f32, cube_z: f32, ch: char, 
    horizontal_offset: f32, a: f32, b: f32, c: f32,
    color: &'static str,
    width: usize, height: usize, // NOVO
    char_buffer: &mut [char], z_buffer: &mut [f32], color_buffer: &mut [&'static str]
) {
    let x = calculate_x(cube_x, cube_y, cube_z, a, b, c);
    let y = calculate_y(cube_x, cube_y, cube_z, a, b, c);
    let z = calculate_z(cube_x, cube_y, cube_z, a, b) + DISTANCE_FROM_CAM;

    let ooz = 1.0 / z;

    // Centraliza usando width e height dinâmicos
    let xp = (width as f32 / 2.0 + horizontal_offset + K1 * ooz * x * 2.0) as i32;
    let yp = (height as f32 / 2.0 + K1 * ooz * y) as i32;

    let idx = xp + yp * (width as i32);

    if idx >= 0 && idx < (width * height) as i32 {
        let idx_usize = idx as usize;
        if ooz > z_buffer[idx_usize] {
            z_buffer[idx_usize] = ooz;
            char_buffer[idx_usize] = ch;
            color_buffer[idx_usize] = color;
        }
    }
}