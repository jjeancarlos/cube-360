use std::{thread, time::Duration};

// Configurações da tela
const WIDTH: usize = 160;
const HEIGHT: usize = 44;
const BACKGROUND_ASCII: char = '.';
const DISTANCE_FROM_CAM: f32 = 100.0;
const K1: f32 = 40.0;
const INCREMENT_SPEED: f32 = 0.6;

// --- Códigos de Cores ANSI ---
#[allow(dead_code)] // <--- Eu sei que não estou usando isso agora, mas quero deixar aí para usar depois(ant warning)
const RESET: &str = "\x1b[0m";
#[allow(dead_code)]
const RED: &str = "\x1b[31m";
#[allow(dead_code)]
const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
const BLUE: &str = "\x1b[34m";
#[allow(dead_code)]
const MAGENTA: &str = "\x1b[35m";
#[allow(dead_code)]
const CYAN: &str = "\x1b[36m";
#[allow(dead_code)]
const WHITE: &str = "\x1b[37m";

fn main() {
    let mut a: f32 = 0.0;
    let mut b: f32 = 0.0;
    let mut c: f32 = 0.0;

    print!("\x1b[2J"); // Limpa a tela

    loop {
        let mut z_buffer: [f32; WIDTH * HEIGHT] = [0.0; WIDTH * HEIGHT];
        let mut char_buffer: [char; WIDTH * HEIGHT] = [BACKGROUND_ASCII; WIDTH * HEIGHT];
        
        // NOVO: Buffer de cores. Inicialmente, tudo é "RESET" (cor padrão do terminal)
        let mut color_buffer: [&str; WIDTH * HEIGHT] = [RESET; WIDTH * HEIGHT];

        // --- Cubo 1 (Grande) - Ciano ---
        let cube_width = 20.0;
        let horizontal_offset = -2.0 * cube_width;
        render_cube(cube_width, horizontal_offset, a, b, c, CYAN, &mut char_buffer, &mut z_buffer, &mut color_buffer);

        // --- Cubo 2 (Médio) - Magenta ---
        let cube_width = 10.0;
        let horizontal_offset = 1.0 * cube_width;
        render_cube(cube_width, horizontal_offset, a, b, c, MAGENTA, &mut char_buffer, &mut z_buffer, &mut color_buffer);

        // --- Cubo 3 (Pequeno) - Amarelo ---
        let cube_width = 5.0;
        let horizontal_offset = 8.0 * cube_width;
        render_cube(cube_width, horizontal_offset, a, b, c, YELLOW, &mut char_buffer, &mut z_buffer, &mut color_buffer);

        // --- Renderização ---
        print!("\x1b[H"); // Move cursor para o topo
        
        let mut output = String::with_capacity((WIDTH * HEIGHT) * 10); // Aumentei a capacidade por causa dos códigos de cor
        
        for k in 0..WIDTH * HEIGHT {
            if k % WIDTH == 0 && k != 0 {
                output.push('\n');
            }
            
            // Se o pixel não for o fundo, aplicamos a cor
            if char_buffer[k] != BACKGROUND_ASCII {
                output.push_str(color_buffer[k]); // Aplica a cor (ex: torna Vermelho)
                output.push(char_buffer[k]);      // Imprime o caractere
                output.push_str(RESET);           // Reseta para não "vazar" a cor para o próximo pixel
            } else {
                // Fundo fica numa cor cinza/padrão fraca para destacar os cubos
                output.push_str("\x1b[90m"); 
                output.push(char_buffer[k]);
                output.push_str(RESET);
            }
        }
        println!("{}", output);

        a += 0.05;
        b += 0.05;
        c += 0.01;

        thread::sleep(Duration::from_millis(16));
    }
}

fn render_cube(
    cube_width: f32,
    horizontal_offset: f32,
    a: f32, b: f32, c: f32,
    color: &'static str, // NOVO: Recebe a cor
    char_buffer: &mut [char],
    z_buffer: &mut [f32],
    color_buffer: &mut [&'static str] // NOVO: Referência para o buffer de cores
) {
    let mut cube_x = -cube_width;
    while cube_x < cube_width {
        let mut cube_y = -cube_width;
        while cube_y < cube_width {
            // Passamos a cor para cada face
            calculate_for_surface(cube_x, cube_y, -cube_width, '@', horizontal_offset, a, b, c, color, char_buffer, z_buffer, color_buffer);
            calculate_for_surface(cube_width, cube_y, cube_x, '$', horizontal_offset, a, b, c, color, char_buffer, z_buffer, color_buffer);
            calculate_for_surface(-cube_width, cube_y, -cube_x, '~', horizontal_offset, a, b, c, color, char_buffer, z_buffer, color_buffer);
            calculate_for_surface(-cube_x, cube_y, cube_width, '#', horizontal_offset, a, b, c, color, char_buffer, z_buffer, color_buffer);
            calculate_for_surface(cube_x, -cube_width, -cube_y, ';', horizontal_offset, a, b, c, color, char_buffer, z_buffer, color_buffer);
            calculate_for_surface(cube_x, cube_width, cube_y, '+', horizontal_offset, a, b, c, color, char_buffer, z_buffer, color_buffer);
            
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
    color: &'static str, // NOVO
    char_buffer: &mut [char], z_buffer: &mut [f32], color_buffer: &mut [&'static str] // NOVO
) {
    let x = calculate_x(cube_x, cube_y, cube_z, a, b, c);
    let y = calculate_y(cube_x, cube_y, cube_z, a, b, c);
    let z = calculate_z(cube_x, cube_y, cube_z, a, b) + DISTANCE_FROM_CAM;

    let ooz = 1.0 / z;

    let xp = (WIDTH as f32 / 2.0 + horizontal_offset + K1 * ooz * x * 2.0) as i32;
    let yp = (HEIGHT as f32 / 2.0 + K1 * ooz * y) as i32;

    let idx = xp + yp * (WIDTH as i32);

    if idx >= 0 && idx < (WIDTH * HEIGHT) as i32 {
        let idx_usize = idx as usize;
        if ooz > z_buffer[idx_usize] {
            z_buffer[idx_usize] = ooz;
            char_buffer[idx_usize] = ch;
            color_buffer[idx_usize] = color; // NOVO: Salva a cor no buffer paralelo
        }
    }
}