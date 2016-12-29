#![feature(test)]

extern crate test;
extern crate roaring_bench;

macro_rules! single_data {
    ($cb:tt) => {
        $cb!(census_income_0, CENSUS_INCOME, 0);
        $cb!(census_income_1, CENSUS_INCOME, 1);
        $cb!(census_income_2, CENSUS_INCOME, 2);
        $cb!(census_income_3, CENSUS_INCOME, 3);
        $cb!(census_income_4, CENSUS_INCOME, 4);
        $cb!(census_income_5, CENSUS_INCOME, 5);
    }
}

mod roaring {
    mod create {
        extern crate roaring;

        macro_rules! create {
            ($n:ident, $d:ident, $i:expr) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data = &::roaring_bench::$d[$i];
                    b.iter(|| -> roaring::RoaringBitmap<u32> {
                        ::test::black_box(data).iter().collect()
                    });
                }
            }
        }

        single_data!(create);
    }
}

mod croaring {
    mod create {
        extern crate croaring;

        macro_rules! create {
            ($n:ident, $d:ident, $i:expr) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data = &::roaring_bench::$d[$i];
                    b.iter(|| {
                        let mut bitmap = croaring::Bitmap::create_with_capacity(::test::black_box(data).len() as u32);
                        bitmap.add_many(::test::black_box(data));
                        bitmap
                    });
                }
            }
        }

        single_data!(create);
    }
}
