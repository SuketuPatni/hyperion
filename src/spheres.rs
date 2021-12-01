/*
The Ray Tracer Challenge (Jamis Buck)
-------------------------------------

Chapter 5: Ray-Sphere Intersections
*/

#[path = "matrix.rs"]
mod matrix;

pub struct Ray {
    pub origin: matrix::tuples::Tuple,
    pub direction: matrix::tuples::Tuple
}

pub fn position(ray: Ray, t:f32) -> matrix::tuples::Tuple {
    matrix::tuples::add(ray.origin, matrix::tuples::multiply(t, ray.direction))
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Sphere {

}

pub fn sphere() -> Sphere {
    Sphere {}
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Intersection {
    pub t:f32,
    pub object: Sphere
}

// pub fn intersections(vec1:Vec<Intersection>) -> Vec<Intersection> {
//     vec1
// }

pub fn intersect(s:Sphere, r:Ray) -> Vec<Intersection> {
    let sphere_to_ray = matrix::tuples::subtract(r.origin, matrix::tuples::point(0.0,0.0,0.0));
    let a = matrix::tuples::dot_product(r.direction,r.direction);
    let b = 2.0 * matrix::tuples::dot_product(r.direction, sphere_to_ray);
    let c = matrix::tuples::dot_product(sphere_to_ray, sphere_to_ray) - 1.0;
    let d = b * b - 4.0 * a * c;

    if d < 0.0 {
        vec![]
    } else {
        let t1 = (-b - d.sqrt())/(2.0 * a);
        let t2 = (-b + d.sqrt())/(2.0 * a);
        vec![
            Intersection {
                t: t1,
                object: s
            },
            Intersection {
                t: t2,
                object: s
            }
        ]
    }
}

pub fn hit(vec:Vec<Intersection>) -> Vec<Intersection> {

    let mut vec2 = vec;
    vec2.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

    let mut hit_value = vec![];
    for i in vec2 {
        if i.t > 0.0 {
            hit_value.push(i);
            break;
        }
    }

    hit_value
    
}

pub fn transform_ray(ray:Ray, transform:matrix::Matrix) -> Ray {
    Ray {
        origin: matrix::multiply_matrix_tup(transform,ray.origin),
        direction: matrix::multiply_matrix_tup(transform,ray.direction),
    }
}

#[allow(dead_code)]
pub fn ray_sphere_tests() {
    let r = Ray {
        origin: matrix::tuples::point(2.0,3.0,4.0),
        direction: matrix::tuples::vector(1.0,0.0,0.0)
    };
    println!("{:?}", position(r,2.5));

    let s1 = sphere();
    let s2 = sphere();
    let s3 = sphere();
    let s4 = sphere();
    let s5 = sphere();


    let r1 = Ray {
        origin: matrix::tuples::point(0.0,0.0,-5.0),
        direction: matrix::tuples::vector(0.0,0.0,1.0)
    };
    let r2 = Ray {
        origin: matrix::tuples::point(0.0,1.0,-5.0),
        direction: matrix::tuples::vector(0.0,0.0,1.0)
    };
    let r3 = Ray {
        origin: matrix::tuples::point(0.0,2.0,-5.0),
        direction: matrix::tuples::vector(0.0,0.0,1.0)
    };
    let r4 = Ray {
        origin: matrix::tuples::point(0.0,0.0,0.0),
        direction: matrix::tuples::vector(0.0,0.0,1.0)
    };
    let r5 = Ray {
        origin: matrix::tuples::point(0.0,0.0,5.0),
        direction: matrix::tuples::vector(0.0,0.0,1.0)
    };

    let r6 = Ray {
        origin: matrix::tuples::point(0.0,0.0,-5.0),
        direction: matrix::tuples::vector(0.0,0.0,1.0),
    };
    let s6 = sphere();

    // hit tests
    let i1 = Intersection {
        t: 5.0,
        object: sphere()
    };
    let i2 = Intersection {
        t: 7.0,
        object: sphere()
    };
    let i3 = Intersection {
        t: -3.0,
        object: sphere()
    };
    let i4 = Intersection {
        t: 2.0,
        object: sphere()
    };
    let i5 = Intersection {
        t: 1.0,
        object: sphere()
    };
    let i6 = Intersection {
        t: 2.0,
        object: sphere()
    };
    let i7 = Intersection {
        t: -1.0,
        object: sphere()
    };
    let i8 = Intersection {
        t: 1.0,
        object: sphere()
    };
    let i9 = Intersection {
        t: -2.0,
        object: sphere()
    };
    let i10 = Intersection {
        t: -1.0,
        object: sphere()
    };


    println!("{:?}", intersect(s1, r1));
    println!("{:?}", intersect(s2, r2));
    println!("{:?}", intersect(s3, r3));
    println!("{:?}", intersect(s4, r4));
    println!("{:?}", intersect(s5, r5));

    println!("{:?}", intersect(s6,r6));

    println!("{:?}", hit(vec![i5,i6]));
    println!("{:?}", hit(vec![i7,i8]));
    println!("{:?}", hit(vec![i9,i10]));
    println!("{:?}", hit(vec![i1,i2,i3,i4]));

}