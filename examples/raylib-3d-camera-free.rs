use std::time::Instant;

use raylib::{ffi::Fade, prelude::*};

fn main() {
    const screen_width: i32 = 1360;
    const screen_height: i32 = 768;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("raylib [core] example - 3d camera free")
        .build();

    let camera = Camera3D::perspective(
        Vector3 {
            x: 10.0,
            y: 10.0,
            z: 10.0,
        },
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        45.0,
    );

    let cube_position = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    // rl.disable_cursor();

    // rl.set_target_fps(60);

    while !rl.window_should_close() {
        let start = Instant::now();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        let fps = d.get_fps();
        let text = format!("Hello, world! - {fps}");
        d.draw_text(&text, 12, 12, 25, Color::BLACK);

        let mut e = d.begin_mode3D(camera);

        
        e.draw_fps(100, 100);
        // e.draw_cube(cube_position, 2.0, 2.0, 2.0, Color::RED);
        // e.draw_cube_wires(cube_position, 2.0, 2.0, 2.0, Color::MAROON);
        e.draw_grid(10, 1.0);
        //  e.draw_rectangle(10, 10, 320, 93, Color::SKYBLUE);
        // e.draw_rectangle_lines(10, 10, 320, 93, Color::BLUE);
        
        
        
        

        println!("fps: {fps}");
    }
}
