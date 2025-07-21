use raylib::prelude::*;
use std::{thread, time};

// Tamaño de la pantalla
const GRID_W: usize = 200;
const GRID_H: usize = 100;

// Fondo: #292b30 → RGB(41, 43, 48)
// Células: #cfab4a → RGB(207, 171, 74)
fn get_color(alive: bool) -> Color {
    if alive {
        Color::new(207, 171, 74, 255)  // célula viva
    } else {
        Color::new(41, 43, 48, 255)    // Fondo
    }
}

// Cuenta vecinos vivos 
fn count_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> u8 {
    let mut count = 0;
    for dy in [-1isize, 0, 1].iter() {
        for dx in [-1isize, 0, 1].iter() {
            if *dx == 0 && *dy == 0 { continue; }
            let nx = ((x as isize + dx + GRID_W as isize) % GRID_W as isize) as usize;
            let ny = ((y as isize + dy + GRID_H as isize) % GRID_H as isize) as usize;
            if grid[ny][nx] { count += 1; }
        }
    }
    count
}

// Inserta un “glider” en la posición (ox, oy)
fn spawn_glider(grid: &mut Vec<Vec<bool>>, ox: usize, oy: usize) {
    let pattern = [
        (1, 0),
        (2, 1),
        (0, 2), (1, 2), (2, 2),
    ];
    for &(dx, dy) in pattern.iter() {
        let x = (ox + dx) % GRID_W;
        let y = (oy + dy) % GRID_H;
        grid[y][x] = true;
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size((GRID_W * 5) as i32, (GRID_H * 5) as i32) // Menos escala de la pantalla
        .title("Conway's Game of Life")
        .build();

    // Buffers lógicos
    let mut current = vec![vec![false; GRID_W]; GRID_H];
    let mut next    = current.clone();

    // Patrón inicial: un glider
    spawn_glider(&mut current, 2, 2);
    spawn_glider(&mut current, 50, 50);
    spawn_glider(&mut current, 100, 20);
    spawn_glider(&mut current, 150, 80);
    spawn_glider(&mut current, 180, 30);
    spawn_glider(&mut current, 120, 90);
    spawn_glider(&mut current, 30, 70);
    spawn_glider(&mut current, 70, 10);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        // Limpiamos el fondo con el color deseado
        d.clear_background(Color::new(41, 43, 48, 255));

        // 1) Render: pintar solo las células vivas
        for y in 0..GRID_H {
            for x in 0..GRID_W {
                if current[y][x] {
                    let color = Color::new(207, 171, 74, 255);
                    // pintamos un bloque 5×5
                    for sy in 0..5 {
                        for sx in 0..5 {
                            d.draw_pixel((x*5 + sx) as i32, (y*5 + sy) as i32, color);
                        }
                    }
                }
            }
        }

        // 2) Lógica: calcular siguiente generación
        for y in 0..GRID_H {
            for x in 0..GRID_W {
                let alive = current[y][x];
                let n = count_neighbors(&current, x, y);
                next[y][x] = match (alive, n) {
                    (true, 2) | (true, 3) => true,   // sobrevive
                    (false, 3)            => true,   // reproducción
                    _                     => false,  // muere o sigue muerto
                };
            }
        }

        // 3) Swap buffers
        std::mem::swap(&mut current, &mut next);

        // 4) Un pequeño delay
        thread::sleep(time::Duration::from_millis(100));
    }
}
