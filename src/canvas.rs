/*
The Ray Tracer Challenge (Jamis Buck)
-------------------------------------

Chapter 2: Drawing on a Canvas
*/

use std::fs;
#[path = "tuples.rs"]
mod tuples;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

pub fn color(r: f32, g:f32, b:f32) -> Color {
    Color{red: r, green: g, blue:b}
}

#[allow(dead_code)]
pub fn equal_colors(c1:Color, c2:Color) -> bool {
    tuples::float_equal(c1.red, c2.red)
    && tuples::float_equal(c1.green, c2.green)
    && tuples::float_equal(c1.blue, c2.blue)
}

#[allow(dead_code)]
pub fn add(col1:Color, col2:Color) -> Color {
    Color{red: col1.red + col2.red, 
        green: col1.green + col2.green, 
        blue: col1.blue + col2.blue
    }
}

#[allow(dead_code)]
pub fn subtract(col1:Color, col2:Color) -> Color {
    Color{red: col1.red - col2.red, 
        green: col1.green - col2.green, 
        blue: col1.blue - col2.blue
    }
}

#[allow(dead_code)]
pub fn multiply(scalar:f32, col1:Color) -> Color {
    Color{red: col1.red * scalar, 
        green: col1.green * scalar, 
        blue: col1.blue * scalar
    }
}

#[allow(dead_code)]
pub fn hadamard_product(col1: Color, col2: Color) -> Color {
    Color {
        red: col1.red * col2.red,
        green: col1.green * col2.green, 
        blue: col1.blue * col2.blue
    }
}

pub fn canvas(width:usize, height:usize) -> Vec<Vec<Color>> {
    // let mut pixelled: Vec<Vec<Color>> = vec![];
    vec![vec![color(0.0,0.0,0.0); width]; height]
}

pub fn write_pixel(
    mut canvas1: Vec<Vec<Color>>, 
    x:usize, 
    y:usize, 
    colour: Color) -> Vec<Vec<Color>> {
    canvas1[y][x] = colour;
    canvas1
}

pub fn header(width:usize,height:usize) -> String {
    // makes PPM header
    // "P3\n{width} {height}\n255"
    let mut header = String::from("P3\n");
    header.push_str(&width.to_string());
    header.push_str(" ");
    header.push_str(&height.to_string());
    header.push_str("\n255\n");
    header
}

#[allow(dead_code)]
pub fn canvas_tests() {
    // println!("{}", header(32, 32));
    // let trial_header = "P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255";
    // fs::write("canvas.ppm", trial_header).expect("can't read file");
    
    let mut canvas1 = canvas(5,3);
    let red = Color{red:1.0, green:0.0, blue:0.0};
    let green = Color{red:0.0, green:1.0, blue:0.0};
    let blue = Color{red:0.0, green:0.0, blue:1.0};
    canvas1 = write_pixel(canvas1, 0, 0, red);
    canvas1 = write_pixel(canvas1, 2, 1, green);
    canvas1 = write_pixel(canvas1, 4, 2, blue);

    let mut ppm_string = header(5,3);

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
    fs::write("renders/canvas.ppm", ppm_string).expect("");

    // println!("{}", write_string);
}