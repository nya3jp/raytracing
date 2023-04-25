#[cfg(rayon)]
pub(crate) fn par_iter_mut<'data, T: 'data + Send>(
    v: &'data mut [T],
) -> rayon::slice::IterMut<'data, T> {
    use rayon::prelude::*;
    v.par_iter_mut()
}

#[cfg(not(rayon))]
pub(crate) fn par_iter_mut<'data, T: 'data + Send>(
    v: &'data mut [T],
) -> std::slice::IterMut<'data, T> {
    v.iter_mut()
}
