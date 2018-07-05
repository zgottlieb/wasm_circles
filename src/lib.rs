use std::mem;

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

// Set values to random positions
#[no_mangle]
pub unsafe fn init(state_ptr: *mut State, display_width: f32, display_height: f32) {
    let state = &mut *state_ptr;
    // let mut circles = Vec::with_capacity(CIRCLE_COUNT as usize);
    // let mut circles = *state.circles;
    // let circle_velocities = &mut state.circle_velocities;
    
    for _ in 0..CIRCLE_COUNT {

        let circle = Circle {
            x: display_width * randomf(),
            y: display_height * randomf(),
            r: 10 as f32,
        };

        state.circles.push(circle);

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

        // let x = circles[i].x;
        // let y = circles[i].y;
        // let r = circles[i].r;
        // let vx = circle_velocities[i].vx;
        // let vy = circle_velocities[i].vy;

        circles[i].x += circle_velocities[i].vx;
        circles[i].y += circle_velocities[i].vy;

        if circles[i].x > display_width || circles[i].x < 0 as f32 {
            circle_velocities[i].vx = -circle_velocities[i].vx;
        }

        if circles[i].y > display_height || circles[i].y < 0 as f32 {
            circle_velocities[i].vy = -circle_velocities[i].vy;
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
