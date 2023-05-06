pub mod iter;
mod _serde;

use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::mem;
use std::ops::Index;
use crate::iter::{IntoSliceIterOrSingle, SliceIterOrSingle};

/// Can be an Array or an Object
///
/// # Serialize Conditions
/// On Serialize if it is an array with one element it will be serialized as an object
/// # Deserialize Conditions
/// If None is provided it will default to an empty array
/// If one element is provided it will be deserialized as an object
pub enum VecOrObject<T> {
    Array(Vec<T>),
    Object(T),
}
impl<T> Index<usize> for VecOrObject<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            VecOrObject::Array(v) => &v[index],
            VecOrObject::Object(v) => {
                assert_eq!(index, 0);
                v
            }
        }
    }
}

impl<T: Debug> Debug for VecOrObject<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VecOrObject::Array(v) => write!(f, "Array: {:?}", v),
            VecOrObject::Object(v) => write!(f, "Object ({:?})", v),
        }
    }
}
impl<T: Hash> Hash for VecOrObject<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            VecOrObject::Array(v) => v.hash(state),
            VecOrObject::Object(v) => v.hash(state),
        }
    }
}
impl<T: Eq> Eq for VecOrObject<T> {}
impl<T: PartialEq> PartialEq for VecOrObject<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VecOrObject::Array(v1), VecOrObject::Array(v2)) => v1 == v2,
            (VecOrObject::Object(v1), VecOrObject::Object(v2)) => v1 == v2,
            _ => false,
        }
    }
}

impl<T: Default> Default for VecOrObject<T> {
    fn default() -> Self {
        VecOrObject::Object(T::default())
    }
}

impl<T: Clone> Clone for VecOrObject<T> {
    fn clone(&self) -> Self {
        match self {
            VecOrObject::Array(v) => VecOrObject::Array(v.clone()),
            VecOrObject::Object(v) => VecOrObject::Object(v.clone()),
        }
    }
}
impl<T> From<Vec<T>> for VecOrObject<T> {
    fn from(v: Vec<T>) -> Self {
        VecOrObject::Array(v)
    }
}

impl<T> From<T> for VecOrObject<T> {
    fn from(v: T) -> Self {
        VecOrObject::Object(v)
    }
}
impl<T> Into<Vec<T>> for VecOrObject<T> {
    fn into(self) -> Vec<T> {
        match self {
            VecOrObject::Array(v) => v,
            VecOrObject::Object(v) => vec![v],
        }
    }
}
impl<T> IntoIterator for VecOrObject<T> {
    type Item = T;
    type IntoIter = IntoSliceIterOrSingle< T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoSliceIterOrSingle::new(self)
    }
}
impl<T> VecOrObject<T> {
    pub fn iter(&self) -> SliceIterOrSingle<'_, T> {
        SliceIterOrSingle::new(self)
    }

    pub fn first(self) -> Option<T> {
        match self {
            VecOrObject::Array(mut v) => {
                if v.is_empty() {
                    None
                } else {
                    Some(v.remove(0))
                }
            }
            VecOrObject::Object(v) => Some(v),
        }
    }
    pub fn first_ref(&self) -> Option<&T> {
        match self {
            VecOrObject::Array(v) => v.first(),
            VecOrObject::Object(v) => Some(v),
        }
    }
    /// Is it single like me?
    ///
    /// Probably should stop over-engineering this and go talk to people instead
    pub fn is_single(&self) -> bool {
        match self {
            VecOrObject::Array(v) => v.len() == 1,
            VecOrObject::Object(_) => true,
        }
    }
    pub fn size(&self) -> usize {
        match self {
            VecOrObject::Array(v) => v.len(),
            VecOrObject::Object(_) => 1,
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            VecOrObject::Array(v) => v.is_empty(),
            VecOrObject::Object(_) => false,
        }
    }
    ///
    pub fn make_vec(&mut self) -> bool{
        if let VecOrObject::Array(_) = self{
             false
        }else {
            let old = mem::replace(  self, VecOrObject::Array(Vec::with_capacity(1)));
            if let (VecOrObject::Object(v),  VecOrObject::Array(target)) = (old,  self){
                target.push(v)
            }
            true
        }
    }
    pub fn increase_capacity(&mut self, additional: usize) {
        if let VecOrObject::Array(v) = self {
            v.reserve(additional);
        }else {
            let old = mem::replace(  self, VecOrObject::Array(Vec::with_capacity(additional+1)));
            if let (VecOrObject::Object(v),  VecOrObject::Array(target)) = (old,  self){
                target.push(v)
            }
        }
    }
}

mod tests{
    #[test]
    pub fn test(){
        let mut v = super::VecOrObject::Object(1);
        let x = v[0];
    }
}