use std::fmt::Debug;
use std::slice::Iter;
use std::vec;
use std::vec::IntoIter;
use crate::VecOrObject;

pub enum SliceIterOrSingle<'a,T>{
    Vec(Iter<'a, T>),
    Single(&'a T),
    Empty,
}
impl<T: Debug> Debug for SliceIterOrSingle<'_, T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SliceIterOrSingle::Vec(v) => write!(f, "SliceIterOrSingle::Vec({:?})", v),
            SliceIterOrSingle::Single(v) => write!(f, "SliceIterOrSingle::Single({:?})", v),
            SliceIterOrSingle::Empty => write!(f, "SliceIterOrSingle::Empty"),
        }
    }
}

impl<'a, T> SliceIterOrSingle<'a, T> {
    pub fn new(v: &'a VecOrObject<T>) -> Self {
        match v {
            VecOrObject::Array(v) => SliceIterOrSingle::Vec(v.iter()),
            VecOrObject::Object(v) => SliceIterOrSingle::Single(v),
        }
    }
}
impl<'a, T> ExactSizeIterator for SliceIterOrSingle<'a, T> {
    fn len(&self) -> usize {
        match self {
            SliceIterOrSingle::Vec(v) => v.len(),
            SliceIterOrSingle::Single(_) => 1,
            SliceIterOrSingle::Empty => 0,
        }
    }
}

impl<'a, T> Iterator for SliceIterOrSingle<'a, T>{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SliceIterOrSingle::Vec(v) => return v.next(),
            SliceIterOrSingle::Empty => return None,
            _ => {}
        }
        let v = std::mem::replace(self, SliceIterOrSingle::Empty);
        if let SliceIterOrSingle::Single(v) = v {
            Some(v)
        }else {
            unreachable!()
        }
    }
}

pub enum IntoSliceIterOrSingle<T>{
    Vec(IntoIter<T>),
    Single(T),
    /// After calling 'next' on a 'Single' variant, it will be replaced with 'Empty' variant.
    Empty,
}
impl<T: Debug> Debug for IntoSliceIterOrSingle<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntoSliceIterOrSingle::Vec(v) => write!(f, "SliceIterOrSingle::Vec({:?})", v),
            IntoSliceIterOrSingle::Single(v) => write!(f, "SliceIterOrSingle::Single({:?})", v),
            IntoSliceIterOrSingle::Empty => write!(f, "SliceIterOrSingle::Empty"),
        }
    }
}
impl<T> Into<IntoIter<T>> for IntoSliceIterOrSingle<T> {
    fn into(self) -> IntoIter<T> {
        match self {
            IntoSliceIterOrSingle::Vec(v) => v,
            IntoSliceIterOrSingle::Single(v) => vec![v].into_iter(),
            IntoSliceIterOrSingle::Empty => vec![].into_iter(),
        }
    }
}


impl <T> IntoSliceIterOrSingle<T>{
    pub fn new(v: VecOrObject<T>) -> Self {
        match v {
            VecOrObject::Array(v) => IntoSliceIterOrSingle::Vec(v.into_iter()),
            VecOrObject::Object(v) => IntoSliceIterOrSingle::Single(v),
        }
    }
}

impl<T> Iterator for IntoSliceIterOrSingle<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let IntoSliceIterOrSingle::Vec(v) = self {
            v.next()
        }else  if let IntoSliceIterOrSingle::Empty = self {
            None
        }else {
            let v = std::mem::replace(self, IntoSliceIterOrSingle::Empty);
            if let IntoSliceIterOrSingle::Single(v) = v {
                Some(v)
            }else {
                unreachable!()
            }
        }
    }
}

impl<T> ExactSizeIterator for IntoSliceIterOrSingle<T> {
    fn len(&self) -> usize {
        match self {
            IntoSliceIterOrSingle::Vec(v) => v.len(),
            IntoSliceIterOrSingle::Single(_) => 1,
            IntoSliceIterOrSingle::Empty => {
                0
            }
        }
    }
}