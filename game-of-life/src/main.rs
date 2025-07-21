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

// Blinker
fn spawn_blinker(grid: &mut Vec<Vec<bool>>, ox: usize, oy: usize) {
    let pattern = [
        (0, 1),
        (1, 1),
        (2, 1),
    ];
    for &(dx, dy) in pattern.iter() {
        let x = (ox + dx) % GRID_W;
        let y = (oy + dy) % GRID_H;
        grid[y][x] = true;
    }
}

// Intento de pulsar period 3
fn spawn_pulsar(grid: &mut Vec<Vec<bool>>, ox: usize, oy: usize) {
    let pattern = [
        // brazos horizontales arriba
        (2,0),(3,0),(4,0),(8,0),(9,0),(10,0),
        // segmentos verticales
        (0,2),(5,2),(7,2),(12,2),
        (0,3),(5,3),(7,3),(12,3),
        (0,4),(5,4),(7,4),(12,4),
        // centro medio
        (2,5),(3,5),(4,5),(8,5),(9,5),(10,5),
        // espejo vertical
        (2,7),(3,7),(4,7),(8,7),(9,7),(10,7),
        (0,8),(5,8),(7,8),(12,8),
        (0,9),(5,9),(7,9),(12,9),
        (0,10),(5,10),(7,10),(12,10),
        (2,12),(3,12),(4,12),(8,12),(9,12),(10,12),
    ];
    for &(dx, dy) in pattern.iter() {
        let x = (ox + dx) % GRID_W;
        let y = (oy + dy) % GRID_H;
        grid[y][x] = true;
    }
}

    // Intento de dinosaurio
    fn spawn_dinosaur(grid: &mut Vec<Vec<bool>>, ox: usize, oy: usize) {
        let pattern = [
            // cabeza
            (6,0),(7,0),(8,0),
            (5,1),(6,1),(7,1),(8,1),
            // ojo
            (7,1),
            // cuello y lomo
            (4,2),(5,2),(6,2),(7,2),(8,2),(9,2),
            (3,3),(4,3),(5,3),(6,3),(7,3),(8,3),(9,3),
            // patas delanteras
            (6,4),(6,5),
            // cuerpo y cola
            (2,4),(3,4),(4,4),(5,4),
            (1,5),(2,5),(3,5),(4,5),
            (0,6),(1,6),(2,6),(3,6),
            (0,7),(1,7),(2,7),
            // patas traseras
            (6,6),(7,6),
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
        .title("Juan Fer's Game of Life")
        .build();

    // Buffers lógicos
    let mut current = vec![vec![false; GRID_W]; GRID_H];
    let mut next    = current.clone();

    // Patrón inicial
    // Gliders
    spawn_glider(&mut current, 2, 2);
    spawn_glider(&mut current, 50, 50);
    spawn_glider(&mut current, 100, 20);
    spawn_glider(&mut current, 150, 80);
    spawn_glider(&mut current, 180, 30);
    spawn_glider(&mut current, 120, 90);
    spawn_glider(&mut current, 30, 70);
    spawn_glider(&mut current, 70, 10);

    // Blinkers
    spawn_blinker(&mut current, 10, 10);
    spawn_blinker(&mut current, 20, 20);
    spawn_blinker(&mut current, 30, 30);
    spawn_blinker(&mut current, 40, 40);
    spawn_blinker(&mut current, 50, 50);
    spawn_blinker(&mut current, 60, 60);
    spawn_blinker(&mut current, 70, 70);
    spawn_blinker(&mut current, 80, 80);

    // Pulsar p3
    spawn_pulsar(&mut current, 100, 50);
    spawn_pulsar(&mut current, 150, 50);
    spawn_pulsar(&mut current, 50, 50);
    spawn_pulsar(&mut current, 50, 100);
    spawn_pulsar(&mut current, 100, 100);
    spawn_pulsar(&mut current, 150, 100);

    // Dinosaurio pixel-art
    spawn_dinosaur(&mut current, 50, 20);
    spawn_dinosaur(&mut current, 150, 70);
    spawn_dinosaur(&mut current, 100, 30);
    spawn_dinosaur(&mut current, 20, 80);
    spawn_dinosaur(&mut current, 180, 10);
    spawn_dinosaur(&mut current, 70, 50);
    spawn_dinosaur(&mut current, 120, 90);
    spawn_dinosaur(&mut current, 30, 60);
    spawn_dinosaur(&mut current, 80, 40);
    spawn_dinosaur(&mut current, 10, 10);
    spawn_dinosaur(&mut current, 90, 20);


    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        // Limpiamos el fondo con el color deseado
        d.clear_background(Color::new(41, 43, 48, 255));

        // Pintar solo las células vivas
        for y in 0..GRID_H {
            for x in 0..GRID_W {
                if current[y][x] {
                    let color = Color::new(207, 171, 74, 255);
                    for sy in 0..5 {
                        for sx in 0..5 {
                            d.draw_pixel((x*5 + sx) as i32, (y*5 + sy) as i32, color);
                        }
                    }
                }
            }
        }

        // Calcular siguiente generación
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

        // Swap buffers
        std::mem::swap(&mut current, &mut next);

        // Delay
        thread::sleep(time::Duration::from_millis(100));
    }
}
