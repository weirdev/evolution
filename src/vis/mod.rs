extern crate image;
extern crate rand;

use std::cmp::min;

use rand::Rng;

use image::{ImageBuffer, RgbImage};

pub fn create_1d_sim_image<'a, S>(
    max_org_count: u32,
    sim_history: &'a [S],
    sim_to_org_display_pos: fn(&'a S) -> Box<dyn Iterator<Item = f32> + 'a>,
    sim_to_safe_zone_bounds: fn(&S) -> (f32, f32),
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
    for (t, sim) in sim_history.iter().enumerate() {
        let (sz_low, sz_high) = sim_to_safe_zone_bounds(sim);
        let sz_low = min((sz_low * max_org_count as f32) as u32, max_org_count - 1);
        let sz_high = min((sz_high * max_org_count as f32) as u32, max_org_count - 1);

        let y = t as u32;
        for pos in sim_to_org_display_pos(sim) {
            let mut x = min((pos * max_org_count as f32) as u32, max_org_count - 1);
            let color = if x >= sz_low && x <= sz_high {
                [0, 255, 0]
            } else {
                [255, 255, 255]
            };
            while x + 1 < max_org_count && image.get_pixel(x, y)[1] != 0 {
                x += 1;
                break;
            }
            let pixel = image.get_pixel_mut(x, y);
            *pixel = image::Rgb(color);
        }

        let pixel = image.get_pixel_mut(sz_low, y);
        *pixel = image::Rgb([255, 0, 0]);
        let pixel = image.get_pixel_mut(sz_high, y);
        *pixel = image::Rgb([255, 0, 0]);
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

    create_1d_sim_image(200, &sim_hist, |xs| {
        Box::new(xs.iter().map(|x| *x as f32 / 100.0))
    }, |_: &Vec<u32>| (0.25, 0.75));
}
