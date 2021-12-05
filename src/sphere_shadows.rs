use std::fs;

#[path = "matrix.rs"]
pub mod matrix;

#[path = "spheres.rs"]
pub mod spheres;

#[path = "canvas.rs"]
mod canvas;

pub fn shadow1_plot() {
    // Definitions
    // ------------------------------------------------------
    // wall definitions
    // Assume boundary ray is tangent at (0.0,1.0,0.0); by similarity, square wall at
    // z = 12 must have side length at least 6 units
    let wall_side = 8.0;
    let half_shift = wall_side/2.0; // wall is centred at origin
    let wall_z = 12.0;

    // canvas definitions
    let canvas_width = 100; // in pixels
    let canvas_height = 100;
    // now, pixels should only be squares (otherwise the rectangular pixels are coloured,
    // leading to skewed image)
    // so let their side length be the minimum of the width and height.
    let pixel_side = wall_side / (*vec![
        canvas_width, canvas_height
    ].iter().min().unwrap() as f32);

    let red = canvas::color(1.0,0.0,0.0);
    let mut canvas1 = canvas::canvas(canvas_width, canvas_height);
    // we'll be throwing a ray at each pixel

    // sphere definitions
    let s = spheres::sphere(); // declare as mutable if transform is to be applied.
    // s = spheres::set_transform(
    //     s,
    //     spheres::matrix::shearing(0.0,0.0,0.0,0.0,1.0,0.0),
    // );

    // ------------------------------------------------------

    for i in 0..canvas_width { // x-loop from x = 0 to x = width - 1
        let pixel_x = pixel_side * (i as f32) - half_shift;
        for j in 0..canvas_height { // y-loop from y = 0 to y = height - 1
            let pixel_y = pixel_side * (j as f32) - half_shift;
            let pixel_location = spheres::matrix::tuples::point(
                pixel_x,
                pixel_y,
                wall_z
            );
            let ray = spheres::Ray {
                origin: spheres::matrix::tuples::point(0.0,0.0,-6.0),
                direction: spheres::matrix::tuples::normalize(
                    spheres::matrix::tuples::subtract(
                        pixel_location,
                        spheres::matrix::tuples::point(0.0,0.0,-6.0)
                    )
                )
            };
            let xs1 = spheres::intersect(s,ray);
            // colour the pixel only if it hits the sphere
            if spheres::hit(xs1).len() != 0 {
                canvas1 = canvas::write_pixel(canvas1,i,j,red);
            }
        }
    }

    let mut ppm_string = canvas::header(canvas_width,canvas_height);

    for i in canvas1 {
        for j in i {
            ppm_string.push_str(&((j.red * 255.0).ceil()).to_string());
            ppm_string.push_str(" ");
            ppm_string.push_str(&((j.green * 255.0).ceil()).to_string());
            ppm_string.push_str(" ");
            ppm_string.push_str(&((j.blue * 255.0).ceil()).to_string());
            ppm_string.push_str(" ");
        }
        ppm_string.push_str("\n");
    }

    // fs::write("canvas.ppm", header(5,3));
    fs::write("renders/sphere_shadows/sphere_shadow1.ppm", ppm_string).expect("");

}