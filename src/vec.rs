//! vector 

use core::slice;
use std::ops::{IndexMut, Index, self};

///simple Vector act like Vec
/// 
/// the diffence is this one is faster when there's only one element
/// 
/// and no alloc if element count is less than one, almost used by battle system for area operation
#[derive(Debug, Clone)]
pub enum Vector<T> {
    Empty,
    One(T),
    Dynamic(Vec<T>),
}
impl<T> Default for Vector<T> {
    #[inline]
    fn default() -> Vector<T> {
        Self::Empty
    }
}
impl<T> Vector<T> {
    #[inline]
    pub const fn new() -> Self{
        Self::Empty
    }
    ///data size 
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Vector::Empty => 0,
            Vector::One(_) => 1,
            Vector::Dynamic(v) => v.len(),
        }
    }
    #[inline(always)]
    ///is this vector empty?
    pub fn is_empty(&self) -> bool{ self.len() == 0 }
    ///push value into vector
    #[inline(always)]
    pub fn push(&mut self, v: T) {
        match self {
            Vector::Empty => *self = Vector::One(v),
            Vector::One(_) => {
                let old = std::mem::replace(self, Self::Dynamic(Vec::with_capacity(2)));
                if let Self::Dynamic(list) = self{
                    if let Vector::One(v0) = old{
                        list.push(v0);
                    }
                    list.push(v);
                }
            },
            Vector::Dynamic(list) => list.push(v),
        }
    }
    ///remove element at index
    pub fn remove_at(&mut self, index: usize) -> Option<T>{
        match self {
            Vector::One(_) if index == 0 => {
                let old = std::mem::replace(self, Self::Empty);
                if let Vector::One(v) = old{
                    return Some(v);
                }
                unreachable!()
            },
            Vector::Dynamic(v) if index < v.len() => {
                Some(v.remove(index))
            },
            _ => None,
        }
    }
    ///remove all elements matches predicter
    pub fn remove_all<F: FnMut(&T) -> bool>(&mut self, mut predicter: F){
        match self {
            Vector::Empty => (),
            Vector::One(x) => {
                if predicter(&*x){
                    *self = Vector::Empty;
                }
            },
            Vector::Dynamic(list) => {
                for index in (0..list.len()).rev() {
                    if predicter(&list[index]){
                        list.remove(index);
                    }
                }
            },
        }
    }
    ///remove last element match predicter
    pub fn remove_last<F: FnMut(&T) -> bool>(&mut self, mut predicter: F) -> Option<T>{
        match self {
            Vector::Empty => (),
            Vector::One(x) => {
                if predicter(&*x){
                    let old = std::mem::replace(self, Self::Empty);
                    if let Vector::One(v) = old{
                        return Some(v);
                    }
                    unreachable!()
                }
            },
            Vector::Dynamic(list) => {
                for index in (0..list.len()).rev() {
                    if predicter(&list[index]){
                        return Some(list.remove(index));
                    }
                }
            },
        }
        None
    }
    ///clear vector memory
    pub fn clear(&mut self){
        *self = Self::Empty
    }
}
impl<T: PartialEq> Vector<T> {
    #[must_use]
    pub fn contains(&self, x: &T) -> bool {
        match self{
            Vector::One(v) if v == x => true,
            Vector::Dynamic(v) => v.contains(x),
            _ => false,
        }
    }
}
impl<T> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match self{
            Vector::One(v) if index == 0 => v,
            Vector::Dynamic(v) if index < v.len() => &v[index],
            _ => panic!("index {} out of range {}", index, self.len())
        }
    }
}
impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self{
            Vector::One(v) if index == 0 => v,
            Vector::Dynamic(v) if index < v.len() => &mut v[index],
            _ => panic!("index {} out of range", index)
        }
    }
}
const EMPTY_VEC: [();0] = [];
impl<T> ops::Deref for Vector<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe {
            match self {
                Vector::Empty => slice::from_raw_parts(EMPTY_VEC.as_ptr() as *const T, 0),
                Vector::One(v) => slice::from_raw_parts(v as *const T, 1),
                Vector::Dynamic(list) => list.as_slice(),
            }
        }
    }
}

impl<T> ops::DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            match self {
                Vector::Empty => slice::from_raw_parts_mut(EMPTY_VEC.as_ptr() as *mut T, 0),
                Vector::One(v) => slice::from_raw_parts_mut(v as *mut T, 1),
                Vector::Dynamic(list) => list.as_mut_slice(),
            }
        }
    }
}
pub struct IntoIter<T> {
    data: Vector<T>,
    current: usize,
    end: usize,
}
impl<T> IntoIter<T>{
    /// Returns the remaining items of this iterator as a slice.
    pub fn as_slice(&self) -> &[T] {
        let len = self.end - self.current;
        unsafe { core::slice::from_raw_parts(self.data.as_ptr().add(self.current), len) }
    }

    /// Returns the remaining items of this iterator as a mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        let len = self.end - self.current;
        unsafe { core::slice::from_raw_parts_mut(self.data.as_mut_ptr().add(self.current), len) }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.current == self.end {
            None
        } else {
            unsafe {
                let current = self.current;
                self.current += 1;
                Some(std::ptr::read(self.data.as_ptr().add(current)))
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.end - self.current;
        (size, Some(size))
    }
}

impl<T> IntoIterator for Vector<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        IntoIter {
            data: self,
            current: 0,
            end: len,
        }
    }
}

impl<'a, T> IntoIterator for &'a Vector<T> {
    type IntoIter = slice::Iter<'a, T>;
    type Item = &'a T;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vector<T> {
    type IntoIter = slice::IterMut<'a, T>;
    type Item = &'a mut T;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
impl<T> FromIterator<T> for Vector<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let mut vector = vector![];
        while let Some(t) = iter.next() {
            vector.push(t);
        }
        vector
    }
}
#[allow(unused)]
#[allow(dead_code)]
#[cfg(test)]
mod vec_unit_test {
    //cargo.exe test --package lib_shared --lib -- vec::vec_unit_test::vector_ --nocapture 
    use std::sync::atomic::fence;
    // use rand::Rng;
    use smallvec::SmallVec;

    struct Point2{
        pub x: i32,
        pub y: i32,
    }
    impl Point2 {
        pub const fn new(x: i32, y: i32) -> Point2{
            Self { x, y }
        }
    }

    use super::*;
    const LOOP_COUNT: usize = 100_000_000;
    #[test]
    fn vector_collect(){
        let col = (0..10i32).into_iter().filter(|x| * x == 1).collect::<Vector<_>>();
        println!("collecting {:?}", col);
    }
}