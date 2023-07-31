extern crate jerasure_rs;

fn is_matrix_invertible(matrix: &mut [i32], rows: i32, w: i32) -> bool {
    assert!((rows * rows) as usize == matrix.len());
    let result = unsafe { jerasure_rs::jerasure_invertible_matrix(matrix.as_mut_ptr(), rows, w) };
    result > 0
}

fn gmult(a: i32, b: i32, w: i32) -> i32 {
    unsafe{ jerasure_rs::galois_single_multiply(a, b, w) }
}

fn main() {
    let rows: i32 = 10;
    let w = 8;
    let mut matrix: [i32; 100] = [0; 100];

    let mut eval_point: i32 = 1;
    let primitive: i32 = 2;

    for i in 0..rows {
        let mut curr = eval_point;
        for j in 0..rows {
            matrix[(i*rows + j) as usize] = curr;
            curr = gmult(curr, eval_point, w);
        }
        eval_point = gmult(eval_point, primitive, w);
    }

    let invertible = is_matrix_invertible(&mut matrix, rows, w);

    println!("{}", invertible);
}
