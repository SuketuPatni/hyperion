/*
The Ray Tracer Challenge (Jamis Buck)
-------------------------------------

Chapter 2: Drawing on a Canvas - Projectile Drawing test
*/
use std::fs;

#[path = "tuples.rs"]
mod tuples;

#[path = "canvas.rs"]
mod canvas;

#[derive(Debug, Copy, Clone)]
pub struct Projectile {
    pub position: tuples::Tuple,
    pub velocity: tuples::Tuple,
}

#[derive(Debug, Copy, Clone)]
pub struct Environment {
    pub gravity: tuples::Tuple,
    pub wind: tuples::Tuple,
}

#[allow(dead_code)]
pub fn projectile_vec() -> Vec<tuples::Tuple> {
    
    let mut projectile = Projectile{
        position: tuples::point(0.0, 0.0, 0.0),
        velocity: tuples::vector(30.0, 50.0, 0.0)};
        // x_max = 260, y_max = 130

    let environ1 = Environment{
        gravity: tuples::vector(0.0, -9.8, 0.0),
        wind: tuples::vector(-0.012, -0.005, 0.0)
    };

    let dt = 0.01;
    let mut positions_vec:Vec<tuples::Tuple> = vec![];

    while projectile.position.y >= 0.0 {
        // println!("{:?}", projectile.position);
        positions_vec.push(projectile.position);

        projectile.position = tuples::add(projectile.position, tuples::multiply(dt,projectile.velocity));
        projectile.velocity = tuples::add(projectile.velocity, tuples::multiply(dt,environ1.gravity));
        projectile.velocity = tuples::add(projectile.velocity, environ1.wind);

    }

    positions_vec
}

#[allow(dead_code)]
pub fn projectile_plot() {
    let main_vec = projectile_vec();
    let mut canvas1 = canvas::canvas(260, 130);
    let white = canvas::color(1.0,1.0,1.0);

    for i in main_vec {
        canvas1 = canvas::write_pixel(canvas1, i.x.ceil() as usize,(129.0 - i.y.ceil()) as usize, white);
    }

    let mut ppm_string = canvas::header(260, 130);

    for i in canvas1 {
        if ppm_string.len() % 70 == 69 {
            ppm_string.push_str("\n");
        }
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

    fs::write("renders/projectile.ppm", ppm_string).expect("");
}

