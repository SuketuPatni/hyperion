use std::fs;

#[path = "canvas.rs"]
mod canvas;

#[path = "matrix.rs"]
mod matrix;

#[allow(dead_code)]
pub fn clock_render() {
    let mut canvas1 = canvas::canvas(80,80);
    // origin is (20,20)
    let yellow = canvas::color(1.0,1.0,0.0);
    
    // length of clock arm is 15 pixels
    let mut p_12 = matrix::tuples::point(0.0,30.0,0.0);

    let rotate = matrix::rotation_z(3.14159265/6.0);

    for _ in 0..12 {
        // (0,15) should become (20,5)
        // So always (x + 20, 20 - y)
        let temp = matrix::multiply_matrix_tup(rotate,p_12);
        p_12 = temp;
        canvas1 = canvas::write_pixel(
            canvas1, (temp.x.ceil() + 40.0) as usize, (40.0 - temp.y.ceil()) as usize, yellow
        );
        // points_vec.push(matrix::multiply_matrix_tup(rotate,p_12));
    }

    let mut ppm_string = canvas::header(80,80);

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
    
    fs::write("renders/clock.ppm", ppm_string).expect("");  

}