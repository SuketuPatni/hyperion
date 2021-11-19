#[path = "tuples.rs"]
mod tuples;

/*
The Ray Tracer Challenge (Jamis Buck)
-------------------------------------

Chapter 3: Matrices
*/

pub struct Matrix2 (
    [[f32;2];2]
);

#[derive(Debug)]
pub struct Matrix3 (
    [[f32;3];3]
);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix (
    pub [[f32;4];4]
);

pub fn multiply_matrix(a:Matrix, b:Matrix) -> Matrix {
    let mut result = Matrix([
        [0.0,0.0,0.0,0.0],
        [0.0,0.0,0.0,0.0],
        [0.0,0.0,0.0,0.0],
        [0.0,0.0,0.0,0.0]
    ]);

    for row in 0..4 {
        for col in 0..4 {
            let mut sum = 0.0;
            for n in 0..4 {
                sum += a.0[row][n] * b.0[n][col];
            }
            result.0[row][col] = sum;
        }
    }

    result
}

pub fn multiply_matrix_tup(a:Matrix, b:tuples::Tuple) -> tuples::Tuple {
    let mut result = tuples::Tuple{x:0.0,y:0.0,z:0.0,w:0.0};
    result.x = a.0[0][0] * b.x + a.0[0][1] * b.y + a.0[0][2] * b.z + a.0[0][3] * b.w;
    result.y = a.0[1][0] * b.x + a.0[1][1] * b.y + a.0[1][2] * b.z + a.0[1][3] * b.w;
    result.z = a.0[2][0] * b.x + a.0[2][1] * b.y + a.0[2][2] * b.z + a.0[2][3] * b.w;
    result.w = a.0[3][0] * b.x + a.0[3][1] * b.y + a.0[3][2] * b.z + a.0[3][3] * b.w;

    result
}

pub fn transpose(a:Matrix) -> Matrix {
    let mut at = a.clone();
    at.0[0][1] = a.0[1][0];
    at.0[0][2] = a.0[2][0];
    at.0[0][3] = a.0[3][0];
    at.0[1][0] = a.0[0][1];
    at.0[1][2] = a.0[2][1];
    at.0[1][3] = a.0[3][1];
    at.0[2][0] = a.0[0][2];
    at.0[2][1] = a.0[1][2];
    at.0[2][3] = a.0[3][2];
    at.0[3][0] = a.0[0][3];
    at.0[3][1] = a.0[1][3];
    at.0[3][2] = a.0[2][3];

    at
}

pub fn determinant_2x2(a:Matrix2) -> f32 {
    a.0[0][0] * a.0[1][1] - a.0[0][1] * a.0[1][0]
}

pub fn determinant_3x3(a:Matrix3) -> f32 {
    let sub1 = determinant_2x2(
        Matrix2([
            [a.0[1][1], a.0[1][2]],
            [a.0[2][1], a.0[2][2]]
        ])
    );

    let sub2 = determinant_2x2(
        Matrix2([
            [a.0[1][0], a.0[1][2]],
            [a.0[2][0], a.0[2][2]]
        ])
    );

    let sub3 = determinant_2x2(
        Matrix2([
            [a.0[1][0], a.0[1][1]],
            [a.0[2][0], a.0[2][1]]
        ])
    );

    a.0[0][0] * sub1
    - a.0[0][1] * sub2
    + a.0[0][2] * sub3
}

pub fn determinant_4x4(a:Matrix) -> f32 {
    let sub1 = determinant_3x3(
        Matrix3([
            [a.0[1][1], a.0[1][2], a.0[1][3]],
            [a.0[2][1], a.0[2][2], a.0[2][3]],
            [a.0[3][1], a.0[3][2], a.0[3][3]]
        ])
    );

    let sub2 = determinant_3x3(
        Matrix3([
            [a.0[1][0], a.0[1][2], a.0[1][3]],
            [a.0[2][0], a.0[2][2], a.0[2][3]],
            [a.0[3][0], a.0[3][2], a.0[3][3]]
        ])
    );

    let sub3 = determinant_3x3(
        Matrix3([
            [a.0[1][0], a.0[1][1], a.0[1][3]],
            [a.0[2][0], a.0[2][1], a.0[2][3]],
            [a.0[3][0], a.0[3][1], a.0[3][3]]
        ])
    );

    let sub4 = determinant_3x3(
        Matrix3([
            [a.0[1][0], a.0[1][1], a.0[1][2]],
            [a.0[2][0], a.0[2][1], a.0[2][2]],
            [a.0[3][0], a.0[3][1], a.0[3][2]]
        ])
    );

    a.0[0][0] * sub1
    - a.0[0][1] * sub2
    + a.0[0][2] * sub3
    - a.0[0][3] * sub4
}

pub fn det_submatrix(a:Matrix, row:usize, col:usize) -> f32 {
    let mut matrix_vec:Vec<f32> = vec![];
    for i in 0..4 {
        if i != row {
            for j in a.0[i] {
                matrix_vec.push(j);
            }
        }
    }

    matrix_vec.remove(col);
    matrix_vec.remove(col + 3);
    matrix_vec.remove(col + 6);

    let submatrix = Matrix3([
        [matrix_vec[0],matrix_vec[1],matrix_vec[2]],
        [matrix_vec[3],matrix_vec[4],matrix_vec[5]],
        [matrix_vec[6],matrix_vec[7],matrix_vec[8]]
    ]);

    // submatrix

    determinant_3x3(submatrix)
}

// pub fn invert(a:Matrix) -> Result<Matrix, String> {
//     // Check if invertible
//     if determinant_4x4(a) != 0.0 {
//         Ok(a)
//     } else {
//         Err(String::from("Matrix not invertible"))
//     }
    
// }

pub fn invert(a:Matrix) -> Matrix {
    let det_a = determinant_4x4(a);
    let mut temp1 = Matrix([
        [0.0,0.0,0.0,0.0],
        [0.0,0.0,0.0,0.0],
        [0.0,0.0,0.0,0.0],
        [0.0,0.0,0.0,0.0]
    ]);

    for i in 0..4 {
        for j in 0..4 {
            temp1.0[i][j] = (-1.0_f32).powf((i + j) as f32) * det_submatrix(a,i,j) / det_a;
        }
    }

    transpose(temp1)
}

#[allow(dead_code)]
pub fn matrix_tests() {
    let matrix1 = Matrix([
        [1.0,1.0,1.0,1.0],
        [1.0,1.0,1.0,1.0],
        [1.0,1.0,1.0,1.0],
        [1.0,1.0,1.0,1.0],
    ]);

    let a = Matrix([
        [1.0,2.0,3.0,4.0],
        [5.0,6.0,7.0,8.0],
        [9.0,8.0,7.0,6.0],
        [5.0,4.0,3.0,2.0]
    ]);

    let b = Matrix([
        [-2.0,1.0,2.0,3.0],
        [3.0,2.0,1.0,-1.0],
        [4.0,3.0,6.0,5.0],
        [1.0,2.0,7.0,8.0]
    ]);

    let c = Matrix([
        [1.0,2.0,3.0,4.0],
        [2.0,4.0,4.0,2.0],
        [8.0,6.0,4.0,1.0],
        [0.0,0.0,0.0,1.0]
    ]);

    let a2x2 = Matrix2([
        [1.0,2.0],
        [3.0,4.0]
    ]);

    let b3x3 = Matrix3([
        [1.0,2.0,6.0],
        [-5.0,8.0,-4.0],
        [2.0,6.0,4.0]
    ]);

    let e = Matrix([
        [-2.0,-8.0,3.0,5.0],
        [-3.0,1.0,7.0,3.0],
        [1.0,2.0,-9.0,6.0],
        [-6.0,7.0,7.0,-9.0]
    ]);

    let det0 = Matrix([
        [-4.0,2.0,-2.0,-3.0],
        [9.0,6.0,2.0,6.0],
        [0.0,-5.0,1.0,-5.0],
        [0.0,0.0,0.0,0.0]
    ]);

    let f1 = Matrix([
        [-5.0,2.0,6.0,-8.0],
        [1.0,-5.0,1.0,8.0],
        [7.0,7.0,-6.0,-7.0],
        [1.0,-3.0,7.0,4.0]
    ]);

    let f2 = Matrix([
        [8.0,-5.0,9.0,2.0],
        [7.0,5.0,6.0,1.0],
        [-6.0,0.0,9.0,6.0],
        [-3.0,0.0,-9.0,-4.0]
    ]);

    let f3 = Matrix([
        [9.0,3.0,0.0,9.0],
        [-5.0,-2.0,-6.0,-3.0],
        [-4.0,9.0,6.0,4.0],
        [-7.0,6.0,6.0,2.0]
    ]);

    let d = tuples::Tuple{x:1.0,y:2.0,z:3.0,w:1.0};

    println!("{:?}", matrix1);
    println!("{:?}", c.0[3][2]);

    println!("{:?}", multiply_matrix(a, b));
    println!("{:?}", multiply_matrix_tup(c, d));

    println!("{:?}", transpose(c));

    println!("{:?}", determinant_2x2(a2x2));
    println!("{:?}", determinant_3x3(b3x3));
    println!("{:?}",determinant_4x4(e));

    println!("{:?}", invert(e));
    println!("{:?}", invert(det0));

    println!("{:?}", det_submatrix(f1,0,1));

    println!("\n{:?}", invert(f1));
    println!("\n{:?}", invert(f2));
    println!("\n{:?}", invert(f3));

    let f4 = multiply_matrix(f1,f2);

    println!("\n{:?}", f4);
    println!("\n{:?}", multiply_matrix(invert(f1),f4));
    println!("\n{:?}", multiply_matrix(f4,invert(f2)));

    let identity = Matrix([
        [1.0,0.0,0.0,0.0],
        [0.0,1.0,0.0,0.0],
        [0.0,0.0,1.0,0.0],
        [0.0,0.0,0.0,1.0],
    ]);

    println!("\n{:?}", invert(identity));

    println!("{:?}", multiply_matrix(f1,invert(f1)));

    println!("\n{:?}", invert(transpose(f1)));
    println!("\n{:?}", transpose(invert(f1)));
    println!("{}", invert(transpose(f1)) == transpose(invert(f1)));
}