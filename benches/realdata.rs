#![feature(test)]

extern crate test;
extern crate roaring_bench;

mod roaring {
    extern crate roaring;

    use std::iter::FromIterator;
    use test::{ black_box, Bencher };
    use roaring_bench;

    #[bench]
    fn create_census_income(b: &mut Bencher) {
        let data = &roaring_bench::CENSUS_INCOME[0];
        b.iter(|| roaring::RoaringBitmap::from_iter(black_box(data)));
    }
}

mod croaring {
    extern crate croaring;

    use test::{ black_box, Bencher };
    use roaring_bench;

    #[bench]
    fn create_census_income(b: &mut Bencher) {
        let data = &roaring_bench::CENSUS_INCOME[0];
        b.iter(|| {
            let mut bitmap = croaring::Bitmap::create_with_capacity(black_box(data).len() as u32);
            bitmap.add_many(black_box(data));
            bitmap
        });
    }
}
