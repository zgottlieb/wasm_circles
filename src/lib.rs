use std::f32;

static CIRCLE_COUNT: i32 = 1000;

pub struct Circle {
    x: f32,
    y: f32,
    r: f32,
}

pub struct CircleVelocity {
    vx: f32,
    vy: f32,
}

pub struct State {
    circles: Vec<Circle>,
    circle_velocities: Vec<CircleVelocity>,
}

extern "C" {
    fn randomf() -> f32;
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

// Set values to random positions
#[no_mangle]
pub unsafe fn init(state_ptr: *mut State, display_width: f32, display_height: f32) {
    let state = &mut *state_ptr;
    // let mut circles = Vec::with_capacity(CIRCLE_COUNT as usize);
    // let mut circles = *state.circles;
    // let circle_velocities = &mut state.circle_velocities;

    for i in 0..CIRCLE_COUNT {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut r: f32 = 0.0 as f32;
        
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

    // console_log_int(state.circles[100].x);
    // state.circles.replace(circles);
    // mem::replace(&mut state.circles, circles.as_mut_ptr());
}

#[no_mangle]
pub unsafe fn time_step(state_ptr: *mut State, display_width: f32, display_height: f32) {
    let state = &mut *state_ptr;

    let circles = &mut state.circles;
    let circle_velocities = &mut state.circle_velocities;

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

        for j in 0..CIRCLE_COUNT {
            let j = j as usize;

            let xj = circles[j].x;
            let yj = circles[j].y;
            let rj = circles[j].r;

            if detect_circle_collision(xi, yi, ri, xj, yj, rj) {
                let vxj = circle_velocities[j].vx;
                let vyj = circle_velocities[j].vy;

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

                circle_velocities[i].vx = vxi + coll_dx * dcvi;
                circle_velocities[i].vy = vyi + coll_dy * dcvi;
                circle_velocities[j].vx = vxj + coll_dx * dcvj;
                circle_velocities[j].vy = vyj + coll_dy * dcvj;
            }
        }
    }
}

#[no_mangle]
pub fn new_state() -> *mut State {
    let state = Box::new(State {
        circles: Vec::with_capacity(CIRCLE_COUNT as usize),
        circle_velocities: Vec::with_capacity(CIRCLE_COUNT as usize),
    });

    Box::into_raw(state)
}

#[no_mangle]
pub fn get_circle_count() -> i32 {
    CIRCLE_COUNT
}

#[no_mangle]
pub fn get_circle_data_offset(state_ptr: *mut State) -> *mut Circle {
    unsafe { (*state_ptr).circles.as_mut_ptr() }
}
