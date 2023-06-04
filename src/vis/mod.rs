extern crate image;
extern crate rand;

use rand::Rng;

use image::{ImageBuffer, RgbImage};

use crate::sim;

pub fn create_1d_sim_image<O>(
    max_org_count: u32,
    sim_history: &[Vec<O>],
    org_to_display_pos: fn(&O) -> f32,
) {
    // a default (black) image containing Rgb values
    // let mut image: RgbImage = ImageBuffer::new(max_org_count, sim_history.len() as u32);
    // ImageBuffer::from_fn(max_org_count, 5, |x, y| {
    //     let pixel = image.get_pixel_mut(x, y);
    //     *pixel = image::Rgb([0, 0, 0]);
    //     *pixel
    // });

    let mut image: RgbImage = ImageBuffer::new(max_org_count, sim_history.len() as u32);
    // Populate the image with the sim history
    for (t, orgs) in sim_history.iter().enumerate() {
        for org in orgs.iter() {
            let mut x = (org_to_display_pos(org) * max_org_count as f32) as u32;
            while x + 1 < max_org_count && image.get_pixel(x, t as u32)[0] == 255 {
                x += 1;
            }
            let pixel = image.get_pixel_mut(x, t as u32);
            *pixel = image::Rgb([255, 255, 255]);
        }
    }

    // write it out to a file
    image.save("output.png").unwrap();
}

pub fn create_image() {
    let mut rng = rand::thread_rng();

    let sim_hist = (0..400)
        .map(|_| {
            (0..rng.gen_range(0..200))
                .map(|_| rng.gen_range(0..100))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    create_1d_sim_image(200, &sim_hist, |x| *x as f32 / 100.0)
}
