use std::{thread, time::Duration};

// Constantes de configuração da tela
const WIDTH: usize = 160;
const HEIGHT: usize = 44;
const BACKGROUND_ASCII: char = '.';
const DISTANCE_FROM_CAM: f32 = 100.0;
const K1: f32 = 40.0;
const INCREMENT_SPEED: f32 = 0.6;

fn main() {
    let mut a: f32 = 0.0;
    let mut b: f32 = 0.0;
    let mut c: f32 = 0.0;

    // Limpa a tela inteira antes de começar
    print!("\x1b[2J");

    loop {
        // Inicializa os buffers
        // z_buffer guarda a profundidade (para saber o que fica na frente)
        let mut z_buffer: [f32; WIDTH * HEIGHT] = [0.0; WIDTH * HEIGHT];
        // buffer guarda os caracteres que serão impressos
        let mut buffer: [char; WIDTH * HEIGHT] = [BACKGROUND_ASCII; WIDTH * HEIGHT];

        // --- Configuração do Primeiro Cubo ---
        let cube_width = 20.0;
        let horizontal_offset = -2.0 * cube_width;
        render_cube(cube_width, horizontal_offset, a, b, c, &mut buffer, &mut z_buffer);

        // --- Configuração do Segundo Cubo ---
        let cube_width = 10.0;
        let horizontal_offset = 1.0 * cube_width;
        render_cube(cube_width, horizontal_offset, a, b, c, &mut buffer, &mut z_buffer);

        // --- Configuração do Terceiro Cubo ---
        let cube_width = 5.0;
        let horizontal_offset = 8.0 * cube_width;
        render_cube(cube_width, horizontal_offset, a, b, c, &mut buffer, &mut z_buffer);

        // --- Renderização na Tela ---
        // Move o cursor para o topo (Home) para sobrescrever o frame anterior
        print!("\x1b[H");
        
        // Construímos uma string única para imprimir tudo de uma vez (evita flickering)
        let mut output = String::with_capacity(WIDTH * HEIGHT + HEIGHT);
        for k in 0..WIDTH * HEIGHT {
            if k % WIDTH == 0 && k != 0 {
                output.push('\n');
            }
            output.push(buffer[k]);
        }
        println!("{}", output);

        // Atualiza ângulos
        a += 0.05;
        b += 0.05;
        c += 0.01;

        // Dorme um pouco para controlar o FPS
        thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }
}

// Função auxiliar para renderizar um cubo específico nos buffers
fn render_cube(
    cube_width: f32,
    horizontal_offset: f32,
    a: f32,
    b: f32,
    c: f32,
    buffer: &mut [char],
    z_buffer: &mut [f32],
) {
    let mut cube_x = -cube_width;
    while cube_x < cube_width {
        let mut cube_y = -cube_width;
        while cube_y < cube_width {
            calculate_for_surface(cube_x, cube_y, -cube_width, '@', horizontal_offset, a, b, c, buffer, z_buffer);
            calculate_for_surface(cube_width, cube_y, cube_x, '$', horizontal_offset, a, b, c, buffer, z_buffer);
            calculate_for_surface(-cube_width, cube_y, -cube_x, '~', horizontal_offset, a, b, c, buffer, z_buffer);
            calculate_for_surface(-cube_x, cube_y, cube_width, '#', horizontal_offset, a, b, c, buffer, z_buffer);
            calculate_for_surface(cube_x, -cube_width, -cube_y, ';', horizontal_offset, a, b, c, buffer, z_buffer);
            calculate_for_surface(cube_x, cube_width, cube_y, '+', horizontal_offset, a, b, c, buffer, z_buffer);
            
            cube_y += INCREMENT_SPEED;
        }
        cube_x += INCREMENT_SPEED;
    }
}

// Funções de cálculo de rotação
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

// Função principal que projeta o 3D no 2D e atualiza os buffers
fn calculate_for_surface(
    cube_x: f32, cube_y: f32, cube_z: f32, ch: char, 
    horizontal_offset: f32, a: f32, b: f32, c: f32,
    buffer: &mut [char], z_buffer: &mut [f32]
) {
    let x = calculate_x(cube_x, cube_y, cube_z, a, b, c);
    let y = calculate_y(cube_x, cube_y, cube_z, a, b, c);
    let z = calculate_z(cube_x, cube_y, cube_z, a, b) + DISTANCE_FROM_CAM;

    let ooz = 1.0 / z; // One over Z (inverso da profundidade)

    // Projeção na tela
    let xp = (WIDTH as f32 / 2.0 + horizontal_offset + K1 * ooz * x * 2.0) as i32;
    let yp = (HEIGHT as f32 / 2.0 + K1 * ooz * y) as i32;

    let idx = xp + yp * (WIDTH as i32);

    // Verificação de limites (Bounds checking)
    // No C, se sair da tela dava Segmentation Fault ou lixo de memória.
    // No Rust, precisamos garantir que o índice existe.
    if idx >= 0 && idx < (WIDTH * HEIGHT) as i32 {
        let idx_usize = idx as usize;
        if ooz > z_buffer[idx_usize] {
            z_buffer[idx_usize] = ooz;
            buffer[idx_usize] = ch;
        }
    }
}