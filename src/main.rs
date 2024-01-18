// Rotation values
static mut A: f64 = 0.0;
static mut B: f64 = 0.0;
static mut C: f64 = 0.0;

// Constants
const CUBE_WIDTH: f64 = 20.0; // width of the cube (should be an integer but less cast to f64 if it already is)
const WIDTH: usize = 75; // width of the canva
const HEIGHT: usize = 40; // height of the canva
const DISTANCE_FROM_CAM: u32 = 100;
const HORIZONTAL_OFFSET: f64 = 0.0; // offset of the cube to display
const K1: f64 = 40.0;
const INCREMENT_SPEED: f64 = 0.6;

//function list

// Compute the new x value of a point after the rotation
fn calculate_x(i: f64, j: f64, k: f64) -> f64 {
    unsafe {
        return 
            j * A.sin() * B.sin() * C.cos() -
            k * A.cos() * B.sin() * C.cos() +
            j * A.cos() * C.sin() +
            k * A.sin() * C.sin() +
            i * B.cos() * C.cos();
    }
}

// Compute the new y value of a point after the rotation
fn calculate_y(i: f64, j: f64, k: f64) -> f64 {
    unsafe {
        return 
            j * A.cos() * C.cos() +
            k * A.sin() * C.cos() -
            j * A.sin() * B.sin() * C.sin() +
            k * A.cos() * B.sin() * C.sin() -
            i * B.cos() * C.sin();
    }
}

// Compute the new z value of a point after the rotation
fn calculate_z(i: f64, j: f64, k: f64) -> f64 {
    unsafe {
        return k * A.cos() * B.cos() - j * A.sin() * B.cos() + i * B.sin();
    }
}

// Compute all new coordinate of each point of a face and put the corresponding char in buffer
fn calculate_for_surface(
    buffer: &mut [char; WIDTH * HEIGHT],
    zbuffer: &mut [f64; WIDTH * HEIGHT],
    cube_x: f64,
    cube_y: f64,
    cube_z: f64,
    ch: char
) {
    let x = calculate_x(cube_x, cube_y, cube_z);
    let y = calculate_y(cube_x, cube_y, cube_z);
    let z = calculate_z(cube_x, cube_y, cube_z) + (DISTANCE_FROM_CAM as f64);

    let ooz: f64 = (1 as f64) / z;

    let xp = ((WIDTH as f64) / 2.0 + HORIZONTAL_OFFSET + K1 * ooz * x * 2.0) as usize;
    let yp = ((HEIGHT as f64) / 2.0 + K1 * ooz * y) as usize;

    let idx: usize = xp + yp * WIDTH;
    if idx < WIDTH * HEIGHT {
        if ooz > zbuffer[idx] {
            zbuffer[idx] = ooz;
            buffer[idx] = ch;
        }
    }
}

// Well, it is just a regular main()
fn main() {
    print!("\x1b[2J");

    loop {
        let mut zbuffer: [f64; WIDTH * HEIGHT] = [0.0; WIDTH * HEIGHT];
        let mut buffer: [char; WIDTH * HEIGHT] = [' '; WIDTH * HEIGHT];

        let mut cube_x: f64 = -CUBE_WIDTH;
        while cube_x < CUBE_WIDTH {
            let mut cube_y: f64 = -CUBE_WIDTH;
            while cube_y < CUBE_WIDTH {
                calculate_for_surface(&mut buffer, &mut zbuffer, cube_x, cube_y, -CUBE_WIDTH, '@');
                calculate_for_surface(&mut buffer, &mut zbuffer, CUBE_WIDTH, cube_y, cube_x, '$');
                calculate_for_surface(&mut buffer, &mut zbuffer, -CUBE_WIDTH, cube_y, -cube_x, '~');
                calculate_for_surface(&mut buffer, &mut zbuffer, -cube_x, cube_y, CUBE_WIDTH, '#');
                calculate_for_surface(&mut buffer, &mut zbuffer, cube_x, -CUBE_WIDTH, -cube_y, ';');
                calculate_for_surface(&mut buffer, &mut zbuffer, cube_x, CUBE_WIDTH, cube_y, '+');
                cube_y += INCREMENT_SPEED;
            }

            cube_x += INCREMENT_SPEED;
        }
        print!("\x1b[H");
        for k in 0..WIDTH * HEIGHT {
            if k % WIDTH == 0 {
                println!();
            } else {
                print!("{}", buffer[k]);
            }
        }

        unsafe {
            A += 0.05;
            B += 0.05;
            C += 0.05;
        }
    }
}
