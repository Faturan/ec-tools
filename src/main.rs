use core::slice;
use std::ffi::c_void;

extern crate jerasure_sys;

fn is_matrix_invertible(matrix: &mut [i32], rows: i32, w: i32) -> bool {
    assert!((rows * rows) as usize == matrix.len());
    let result = unsafe { jerasure_sys::jerasure_invertible_matrix(matrix.as_mut_ptr(), rows, w) };
    result > 0
}

fn gmult(a: i32, b: i32, w: i32) -> i32 {
    unsafe { jerasure_sys::galois_single_multiply(a, b, w) }
}

fn reed_sol_vandermonde_coding_matrix(k: usize, m: usize, w: usize) -> Vec<i32> {
    let mut matrix = Vec::new();
    matrix.reserve(k * m);

    unsafe {
        let matrix_ptr =
            jerasure_sys::reed_sol_vandermonde_coding_matrix(k as i32, m as i32, w as i32);
        let matrix_slice = slice::from_raw_parts(matrix_ptr, k * m);
        matrix.extend_from_slice(matrix_slice);
        jerasure_sys::free(matrix_ptr as *mut c_void);
    }

    matrix
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
            matrix[(i * rows + j) as usize] = curr;
            curr = gmult(curr, eval_point, w);
        }
        eval_point = gmult(eval_point, primitive, w);
    }

    let invertible = is_matrix_invertible(&mut matrix, rows, w);

    println!("{}", invertible);

    let mut rs_matrix = reed_sol_vandermonde_coding_matrix(5, 5, 8);
    let rs_invertible = is_matrix_invertible(&mut rs_matrix, 5, 5);

    println!("{}", rs_invertible);
}
