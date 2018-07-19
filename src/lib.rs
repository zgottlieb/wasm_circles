use std::f32;

static CIRCLE_COUNT: i32 = 2000;
static GRID_WIDTH: i32 = 70;
static GRID_HEIGHT: i32 = 120;

pub struct Circle {
    x: f32,
    y: f32,
    r: f32,
}

pub struct CircleVelocity {
    vx: f32,
    vy: f32,
}

// Holder for app state
pub struct State {
    circles: Vec<Circle>,
    circle_velocities: Vec<CircleVelocity>,
    grid: Vec<Vec<Vec<u32>>>,
}

// Functions coming in from Javascript
extern "C" {
    fn randomf() -> f32;
    fn floorf(val: f32) -> i32;
    fn console_log_int(val: f32);
    fn console_log_str(val: *const u8, len: usize);
}

fn detect_circle_collision(x1: f32, y1: f32, r1: f32, x2: f32, y2: f32, r2: f32) -> bool {
    if x1 + r1 < x2 - r2 || x1 - r1 > x2 + r2 ||
    y1 + r1 < y2 - r2 || y1 - r1 > y2 + r2 {
        return false;
    }
    
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt() <= r1 + r2
}

// Create circles of random size and velocity and set random positioning
#[no_mangle]
pub unsafe fn init(state_ptr: *mut State, display_width: f32, display_height: f32) {
    let state = &mut *state_ptr;

    for _i in 0..CIRCLE_COUNT {
        let mut x: f32;
        let mut y: f32;
        let mut r: f32;
        
        loop {
            
            let mut collision = false;
            x = display_width * randomf();
            y = display_height * randomf();
            r = ((randomf() * 8.0).exp() / 23.2887).ceil() as f32;

            if display_width - (x + r) < 0.0
                || x - r < 0.0
                || display_height - (y + r) < 0.0
                || y - r < 0.0
            {
                
                collision = true;
            } else {
                for j in 0..state.circles.len() {
                    let j = j as usize;

                    if detect_circle_collision(x, y, r, state.circles[j].x, state.circles[j].y, state.circles[j].r) {
                        collision = true;
                        break;
                    }
                }
            }

            if !collision {
                break;
            }
        }
        
        state.circles.push(Circle { x: x, y: y, r: r });

        state.circle_velocities.push(CircleVelocity {
            vx: (randomf() - 0.5) * 0.01,
            vy: (randomf() - 0.5) * 0.01,
        });
    }
}

// Updates each circle position and handle collisions
#[no_mangle]
pub unsafe fn time_step(state_ptr: *mut State, display_width: f32, display_height: f32) {
    let state = &mut *state_ptr;

    let circles = &mut state.circles;
    let circle_velocities = &mut state.circle_velocities;
    let grid = &mut state.grid;

    for i in 0..GRID_WIDTH {        
        let i = i as usize;
    
        for j in 0..GRID_HEIGHT {
            let j = j as usize;
            grid[i][j].clear();
        }
    }
    
    for i in 0..CIRCLE_COUNT {
        let i = i as usize;

        let xi = circles[i].x;
        let yi = circles[i].y;
        let ri = circles[i].r;

        let mut vxi = circle_velocities[i].vx;
        let mut vyi = circle_velocities[i].vy;

        vyi += 0.0001;

        if (display_width - (xi + ri) < 0.0 && vxi > 0.0) || (xi - ri < 0.0 && vxi < 0.0) {
            vxi = -vxi;
        }

        if (display_height - (yi + ri) < 0.0 && vyi > 0.0) || (yi - ri < 0.0 && vyi < 0.0) {
            vyi = -vyi;
        }

        circles[i].x = xi + vxi;
        circles[i].y = yi + vyi;
        circle_velocities[i].vx = vxi;
        circle_velocities[i].vy = vyi;

        let mut left_col = floorf((xi - ri) / display_width * GRID_WIDTH as f32) as i32;
        let mut right_col = floorf((xi + ri) / display_width * GRID_WIDTH as f32) as i32;
        let mut top_row = floorf((yi - ri) / display_height * GRID_HEIGHT as f32) as i32;
        let mut bottom_row = floorf((yi + ri) / display_height * GRID_HEIGHT as f32) as i32;

        if left_col < 0 {
            left_col = 0;
        }

        if right_col >= GRID_WIDTH {
            right_col = GRID_WIDTH - 1;
        }

        if top_row < 0 {
            top_row = 0;
        }

        if bottom_row >= GRID_HEIGHT {
            bottom_row = GRID_HEIGHT - 1;
        }

        let mut p = left_col;

        while p <= right_col {
            let mut q = top_row;

            while q <= bottom_row {
                grid[p as usize][q as usize].push(i as u32);
                q += 1;
            }
            
            p += 1;
        }
    }

    for i in 0..GRID_WIDTH {
        let i = i as usize;

        for j in 0..GRID_HEIGHT {
            let j = j as usize;

            let cell = &grid[i][j];
            
            for k in 0..cell.len() {
                let circ1_index = cell[k] as usize;
            
                let xi = circles[circ1_index].x;
                let yi = circles[circ1_index].y;
                let ri = circles[circ1_index].r;

                let vxi = circle_velocities[circ1_index].vx;
                let vyi = circle_velocities[circ1_index].vy;

                for l in (k + 1)..cell.len() {
                    let l = l as usize;

                    let circ2_index = cell[l] as usize;

                    let xj = circles[circ2_index].x;
                    let yj = circles[circ2_index].y;
                    let rj = circles[circ2_index].r;

                    if detect_circle_collision(xi, yi, ri, xj, yj, rj) {
                        let vxj = circle_velocities[circ2_index].vx;
                        let vyj = circle_velocities[circ2_index].vy;

                        let mut coll_dx = xj - xi;
                        let mut coll_dy = yj - yi;

                        let coll_len = (coll_dx * coll_dx + coll_dy * coll_dy).sqrt();

                        coll_dx = coll_dx / coll_len;
                        coll_dy = coll_dy / coll_len;

                        let cui = coll_dx * vxi + coll_dy * vyi;
                        let cuj = coll_dx * vxj + coll_dy * vyj;

                        if cui - cuj <= 0.0 {
                            continue;
                        }

                        let mass_sum = ri + rj;
                        let mass_diff = ri - rj;
                        let cvi = (cui * mass_diff + 2.0 * rj * cuj) / mass_sum;
                        let cvj = (2.0 * ri * cui - cuj * mass_diff) / mass_sum;

                        let dcvi = cvi - cui;
                        let dcvj = cvj - cuj;

                        circle_velocities[circ1_index].vx = vxi + coll_dx * dcvi;
                        circle_velocities[circ1_index].vy = vyi + coll_dy * dcvi;
                        circle_velocities[circ2_index].vx = vxj + coll_dx * dcvj;
                        circle_velocities[circ2_index].vy = vyj + coll_dy * dcvj;
                    }
                }
            }
        }
    } 
}

/*
* new_state() 
* - creates a new State struct and allocates memory for grid, circles, and circle_velocities arrays and returns a Box pointer 
* - Chose to return the Box pointer here so that when calling init() and time_step() from Javascript, the wasm module
*   knows where in memory the arrays contained in State exist; one alternative to this is to store the arrays in global state
*   (as is done in the C code in the egghead.io demo), but I wanted to avoid using global state
*/
#[no_mangle]
pub fn new_state() -> *mut State {

    // This was my solution for creating nested vectors to create a grid. Doing something like `vec![Vec::with_capacity(1000); 120]`
    // does not work because the clone() that happens in that macro only copies values, not capacities!
    // TODO: Determine a way to use less memory here: making vectors of size CIRCLE_COUNT for each cell in a 120 x 70 grid is too expensive!
    let grid: Vec<Vec<Vec<u32>>> = (0..GRID_WIDTH)
        .map(|_| (0..GRID_HEIGHT).map(|_| Vec::with_capacity(CIRCLE_COUNT as usize)).collect())
        .collect();

    let state = Box::new(State {
        circles: Vec::with_capacity(CIRCLE_COUNT as usize),
        circle_velocities: Vec::with_capacity(CIRCLE_COUNT as usize),
        grid,
    });

    Box::into_raw(state)
}

#[no_mangle]
pub fn get_circle_count() -> i32 {
    CIRCLE_COUNT
}

/*
*   get_circle_data_offset()
*   - takes a raw pointer to a State struct
*   - returns a raw pointer to the circles array in that State struct
*/
#[no_mangle]
pub fn get_circle_data_offset(state_ptr: *mut State) -> *mut Circle {
    unsafe { (*state_ptr).circles.as_mut_ptr() }
}
