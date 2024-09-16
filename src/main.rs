use macroquad::prelude::*;
use ears::{Music, Sound, AudioController};
use diff::Diff;

#[macroquad::main("BasicShapes")]
async fn main() {

    let mut sound = Sound::new("mono.wav").unwrap();
    //sound.set_air_absorption_factor(1.);
    
    sound.set_position([0., 0., 0.]);
    sound.set_reference_distance(100.);
    sound.play();
    

    // let mut sound = Sound::new("your-sound-effect.wav").unwrap();
    // sound.play();

    let origin = Vec3::new(300., 300., 0.);

    let mut previous_state = sound.clone();

    //let diff = previous_state.diff(&sound);

    loop {

        previous_state = sound.clone();


        if is_key_pressed(KeyCode::Escape) {
            sound.set_volume(0.5);
        }

        let mouse_pos_wrt_origin = Vec3::new(
            (origin.x - mouse_position().0) * -1.,
            (origin.y - mouse_position().1) * -1.,
            0.
        );

        let diff = previous_state.diff(&sound);

        println!("{:?}", diff.source_path);
        
    
        
        draw_circle(mouse_position().0, mouse_position().1, 5., BLUE);

        draw_circle(origin.x, origin.y, 10., RED);

        sound.set_position([mouse_pos_wrt_origin.x, mouse_pos_wrt_origin.y, 0.]);

        next_frame().await
    }
}
