#![feature(portable_simd)]

use std::{ops::{Index, IndexMut},  simd::{Simd, num::SimdFloat}};

pub struct Matrix {
    data: Box<[f32]>,
    width: usize,
    height: usize,
    col_order: bool
}

impl Matrix {
    pub fn empty(width: usize, height: usize) -> Matrix {
        Matrix { data:vec![0.;width*height].into_boxed_slice(), width, height, col_order: false }
    }
    pub fn set(&mut self, data: &[f32]) {
        self.data = data.to_owned().into_boxed_slice();
    }
    pub fn log(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}, ",self[(x,y)])
            }
            println!()
        }
        println!()
    }
    pub fn multiply(&self, other:&Matrix, result: &mut Matrix) {
        result.transpose();

        for i in 0..self.height {
            for j in 0..other.width {
                let mut sum = 0.;
                let mut k = 0;
                while k < self.width {
                    if k + 7 < self.width {
                        let a: Simd<f32, 8> = Simd::from_slice(&self.data[self.ix(k, i)..=self.ix(k + 7, i)]);
                        let b: Simd<f32, 8> = Simd::from_slice(&other.data[other.ix(j, k)..=other.ix(j, k + 7)]);

                        sum += (a*b).reduce_sum();

                        k += 8;
                    } else if k + 3 < self.width {
                        let a: Simd<f32, 4> = Simd::from_slice(&self.data[self.ix(k, i)..=self.ix(k + 3, i)]);
                        let b: Simd<f32, 4> = Simd::from_slice(&other.data[other.ix(j, k)..=other.ix(j, k + 3)]);

                        sum += (a*b).reduce_sum();

                        k += 4;
                    } else {
                        sum += self[(k, i)] * other[(j, k)];
                        k += 1;
                    }
                }
                result[(j,i)] = sum;
            }
        }
    }
    pub const fn ix(&self, x: usize, y: usize) -> usize {
        if self.col_order {
            return x * self.height + y
        }
        y * self.width + x
    }
    pub fn transpose(&mut self) {
        let mut m = Matrix::empty(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                m.data[x * self.height + y] = self[(x, y)];
            }
        }
        m.col_order = true;
        *self = m;
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f32;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if self.col_order {
            return &self.data[index.0 * self.height + index.1]
        }
        &self.data[index.1 * self.width + index.0]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if self.col_order {
            return &mut self.data[index.0 * self.height + index.1]
        }
        &mut self.data[index.1 * self.width + index.0]
    }
}

