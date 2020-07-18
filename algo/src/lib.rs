#![feature(test)]
extern crate test;
mod insertion_sort;
mod merge_sort;
mod selection_sort;
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_merge(b: &mut Bencher) {
        b.iter(|| merge_sort::merge::test_sort());
    }
    #[bench]
    fn bench_selection(b: &mut Bencher) {
        b.iter(|| selection_sort::selection::test_sort());
    }
    #[bench]
    fn bench_insertion(b: &mut Bencher) {
        b.iter(|| insertion_sort::insertion::test_sort());
    }
}
