use core::{
    ops::{Index, Range},
    slice::SliceIndex,
};

pub trait Ranged {
    fn range(&self, start: usize, end: usize) -> &Self;
}

impl<T> Ranged for T
where
    T: Index<Range<usize>, Output = T> + ?Sized,
    Range<usize>: SliceIndex<T>,
{
    fn range(&self, start: usize, end: usize) -> &Self {
        &self[start..end]
    }
}
