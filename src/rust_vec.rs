use crate::vector::VecOps;
use crate::vector::Vector;

#[repr(C)]
pub struct RustVec<T: VecOps<T>> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T: VecOps<T>> RustVec<T> {
    pub fn from(mut s: Vec<T>) -> Self {
        let ret = RustVec {
            ptr: s.as_mut_ptr(),
            len: s.len(),
            capacity: s.capacity(),
        };
        std::mem::forget(s);
        ret
    }

    pub fn into_vec(self) -> Vec<T> {
        self.to_vec()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn to_vector(&self, vec: &mut Vector<T>) {
        let v = self.to_vec();
        for item in &v {
            vec.push_back(item);
        }
        std::mem::forget(v);
    }

    fn to_vec(&self) -> Vec<T> {
        unsafe { Vec::<T>::from_raw_parts(self.ptr, self.len, self.capacity) }
    }
}

impl<T: VecOps<T>> Drop for RustVec<T> {
    fn drop(&mut self) {
        let v = self.to_vec();
        drop(v);
    }
}
