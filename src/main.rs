#![feature(test)]
extern crate test;

mod aligned_alloc;

use aligned_alloc::Memory;

fn main() {
    unsafe {
        let a: Memory<f32> = Memory::new(1024, 32).filled_with(1.0);
        let b: Memory<f32> = Memory::new(1024, 32).filled_with(2.0);
        let mut c: Memory<f32> = Memory::new(1024, 32);
        // let c = a[0] + b[0];
        for i in 0..1024 {
            c[i] = a[i] * b[i];
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;

    #[bench]
    fn non_simd_add(bencher: &mut Bencher) {
        const SIZE: usize = 1024 * 6;
        unsafe {
            let a: Memory<f32> = Memory::new(SIZE, 32).filled_with(1.0);
            let b: Memory<f32> = Memory::new(SIZE, 32).filled_with(2.0);
            let mut c: Memory<f32> = Memory::new(SIZE, 32);
            // let c = a[0] + b[0];
            bencher.iter(|| {
                for i in 0..SIZE {
                    c[i] = a[i] * b[i] + 2.0;
                }
            });
            // assert!(c[0] == 2.0);
        }
    }

    #[bench]
    fn simd_add(bencher: &mut Bencher) {
        unsafe {
            use std::arch::x86_64::_mm256_add_ps;
            use std::arch::x86_64::{
                _mm256_load_ps, _mm256_mul_ps, _mm256_store_ps, _mm_undefined_ps,
            };
            const SIZE: usize = 1024 * 6;
            let zeros = _mm_undefined_ps();
            let a: Memory<f32> = Memory::new(SIZE, 32).filled_with(1.0);
            let b: Memory<f32> = Memory::new(SIZE, 32).filled_with(2.0);
            let mut c: Memory<f32> = Memory::new(SIZE, 32);
            let a_ptr = a.as_ptr();
            let b_ptr = b.as_ptr();
            let c_ptr = c.as_mut_ptr();
            bencher.iter(|| {
                // let mut i = 0;
                // while i < SIZE {
                for i in (0..SIZE).step_by(64) {
                    let a256_0 = _mm256_load_ps(a_ptr.add(i));
                    let a256_1 = _mm256_load_ps(a_ptr.add(i + 8));
                    let a256_2 = _mm256_load_ps(a_ptr.add(i + 16));
                    let a256_3 = _mm256_load_ps(a_ptr.add(i + 24));
                    let a256_4 = _mm256_load_ps(a_ptr.add(i + 32));
                    let a256_5 = _mm256_load_ps(a_ptr.add(i + 40));
                    let a256_6 = _mm256_load_ps(a_ptr.add(i + 48));
                    let a256_7 = _mm256_load_ps(a_ptr.add(i + 56));
                    let b256_0 = _mm256_load_ps(b_ptr.add(i));
                    let b256_1 = _mm256_load_ps(b_ptr.add(i + 8));
                    let b256_2 = _mm256_load_ps(b_ptr.add(i + 16));
                    let b256_3 = _mm256_load_ps(b_ptr.add(i + 24));
                    let b256_4 = _mm256_load_ps(b_ptr.add(i + 32));
                    let b256_5 = _mm256_load_ps(b_ptr.add(i + 40));
                    let b256_6 = _mm256_load_ps(b_ptr.add(i + 48));
                    let b256_7 = _mm256_load_ps(b_ptr.add(i + 56));

                    let res_0 = _mm256_mul_ps(a256_0, b256_0);
                    let res_1 = _mm256_mul_ps(a256_1, b256_1);
                    let res_2 = _mm256_mul_ps(a256_2, b256_2);
                    let res_3 = _mm256_mul_ps(a256_3, b256_3);
                    let res_4 = _mm256_mul_ps(a256_4, b256_4);
                    let res_5 = _mm256_mul_ps(a256_5, b256_5);
                    let res_6 = _mm256_mul_ps(a256_6, b256_6);
                    let res_7 = _mm256_mul_ps(a256_7, b256_7);

                    _mm256_store_ps(c_ptr.add(i), res_0);
                    _mm256_store_ps(c_ptr.add(i + 8), res_1);
                    _mm256_store_ps(c_ptr.add(i + 16), res_2);
                    _mm256_store_ps(c_ptr.add(i + 24), res_3);
                    _mm256_store_ps(c_ptr.add(i + 32), res_4);
                    _mm256_store_ps(c_ptr.add(i + 40), res_5);
                    _mm256_store_ps(c_ptr.add(i + 48), res_6);
                    _mm256_store_ps(c_ptr.add(i + 56), res_7);

                    // i += 32;
                }
            });
            // assert!(c[0] == 2.0);
        }
    }
}
