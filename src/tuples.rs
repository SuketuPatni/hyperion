/*
The Ray Tracer Challenge (Jamis Buck)
-------------------------------------

Chapter 1: Tuples, Points and Vectors
*/

pub fn float_equal(a:f32, b:f32) -> bool {
    (a-b).abs() <= f32::EPSILON
}

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x:f32,
    pub y:f32,
    pub z:f32,
    pub w:f32,
}

pub fn point(a:f32, b:f32, c:f32) -> Tuple {
    Tuple{x:a,y:b,z:c,w:1.0}
}

pub fn vector(a:f32, b:f32, c:f32) -> Tuple {
    Tuple{x:a,y:b,z:c,w:0.0}
}

pub fn equal_tuples(tup1:Tuple, tup2:Tuple) -> bool {
    float_equal(tup1.x, tup2.x) 
        && float_equal(tup1.y, tup2.y) 
        && float_equal(tup1.z, tup2.z) 
        && float_equal(tup1.w, tup2.w)
}

pub fn add(tup1:Tuple, tup2:Tuple) -> Tuple {
    Tuple{x:tup1.x + tup2.x, y:tup1.y + tup2.y, z:tup1.z + tup2.z, w:tup1.w + tup2.w}
}

pub fn subtract(tup1:Tuple, tup2:Tuple) -> Tuple {
    Tuple{x:tup1.x - tup2.x, y:tup1.y - tup2.y, z:tup1.z - tup2.z, w:tup1.w - tup2.w}
}

pub fn negate(tup1:Tuple) -> Tuple {
    Tuple{x:-tup1.x, y:-tup1.y, z:-tup1.z, w:-tup1.w}
}

pub fn multiply(scalar:f32, tup1:Tuple) -> Tuple {
    Tuple{x:scalar*tup1.x, y:scalar*tup1.y, z:scalar*tup1.z, w:scalar*tup1.w}
}

pub fn magnitude(tup1:Tuple) -> f32  {
    (tup1.x * tup1.x + tup1.y * tup1.y + tup1.z * tup1.z + tup1.w * tup1.w).sqrt()
}

pub fn normalize(tup1:Tuple) -> Tuple {
    // direction of tuple
    multiply(1.0/magnitude(tup1), tup1)
}

pub fn dot_product(vec1:Tuple, vec2:Tuple) -> f32 {
    vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z + vec1.w * vec2.w
}

pub fn cross_product(vec1:Tuple, vec2:Tuple) -> Tuple {
    vector(
        vec1.y * vec2.z - vec1.z * vec2.y, 
        vec1.z * vec2.x - vec1.x * vec2.z,
        vec1.x * vec2.y - vec1.y * vec2.x)
}


#[allow(dead_code)]
pub fn tuple_tests() {
    let point1 = point(3.2,4.2,3.2);
    let vec1 = vector(3.2,4.2,3.2);
    println!("{:?}", point1);
    println!("{:?}", vec1);

    let point2 = point(3.2,-4.2,3.0);
    let vec2 = vector(4.5, -10.0, 0.0);
    let point3 = point(3.2,-4.2,3.0);
    println!("{}",equal_tuples(point2, vec2));
    println!("{}",equal_tuples(point2, point3));

    println!("{:?}",add(point2, vec2));
    println!("{:?}", add(vec2, vec1));

    println!("{:?}", subtract(point3, point1));

    println!("{}", equal_tuples(subtract(vector(0.0,0.0,0.0),     
    vec1), negate(vec1)));

    println!("{:?}", multiply(5.0, vec2));

    let vec3 = vector(1.0, 2.0, 3.0);
    println!("{}", magnitude(vec3));
    println!("{:?}", normalize(vec3));

    println!("{:?}", dot_product(vec2, vec3));

    let vec4 = vector(1.0,2.0,3.0);
    let vec5 = vector(2.0,3.0,4.0);

    println!("{:?}", cross_product(vec4, vec5));
    println!("{:?}", cross_product(vec5, vec4));

}
