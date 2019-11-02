use crate::collect::Collect;

/// A wrapper type that implements Collect whenever the contained T is 'static, which is useful in
/// generic contexts
#[derive(Debug)]
pub struct StaticCollect<T>(pub T);

unsafe impl<T: 'static> Collect for StaticCollect<T> {
    #[inline]
    fn needs_trace() -> bool {
        false
    }
}


impl<T: Clone> Clone for StaticCollect<T> {
    fn clone(&self) -> StaticCollect<T> {
        StaticCollect(self.0.clone())
    } 
}

impl<T: PartialEq> PartialEq for StaticCollect<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    } 
}