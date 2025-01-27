use bevy::color::Srgba;
use rand::Rng;

pub fn get_random_color() -> Srgba {
    let mut rng = rand::thread_rng();
    let r: f32 = rng.gen_range(0.0..1.0);
    let g: f32 = rng.gen_range(0.0..1.0);
    let b: f32 = rng.gen_range(0.0..1.0);

    //info!("getting a random color: {}-{}-{}", r, g, b);
    Srgba {
        red: r,
        green: g,
        blue: b,
        alpha: 1.0,
    }
}
