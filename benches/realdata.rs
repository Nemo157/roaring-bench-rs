#![feature(test)]

extern crate test;
extern crate roaring_bench;

extern crate roaring;
extern crate croaring;

macro_rules! single_data {
    ($cb:tt) => {
        mod $cb {
            $cb!(census_income_0, CENSUS_INCOME, 0);
            $cb!(census_income_1, CENSUS_INCOME, 1);
            $cb!(census_income_2, CENSUS_INCOME, 2);
            $cb!(census_income_3, CENSUS_INCOME, 3);
            $cb!(census_income_4, CENSUS_INCOME, 4);
            $cb!(census_income_5, CENSUS_INCOME, 5);
        }
    }
}

mod bench {
    mod roaring {
        macro_rules! create {
            ($n:ident, $d:ident, $i:expr) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data = &::roaring_bench::$d[$i];
                    b.iter(|| -> ::roaring::RoaringBitmap<u32> {
                        ::test::black_box(data).iter().collect()
                    });
                }
            }
        }

        macro_rules! clone {
            ($n:ident, $d:ident, $i:expr) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data = &::roaring_bench::$d[$i];
                    let bitmap = data.iter().collect::<::roaring::RoaringBitmap<u32>>();
                    b.iter(|| ::test::black_box(&bitmap).clone());
                }
            }
        }

        single_data!(create);
        single_data!(clone);
    }

    mod croaring {
        macro_rules! create {
            ($n:ident, $d:ident, $i:expr) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data = &::roaring_bench::$d[$i];
                    b.iter(|| {
                        let mut bitmap = ::croaring::Bitmap::create_with_capacity(::test::black_box(data).len() as u32);
                        bitmap.add_many(::test::black_box(data));
                        bitmap
                    });
                }
            }
        }

        macro_rules! clone {
            ($n:ident, $d:ident, $i:expr) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data = &::roaring_bench::$d[$i];
                    let mut bitmap = ::croaring::Bitmap::create_with_capacity(::test::black_box(data).len() as u32);
                    bitmap.add_many(data);
                    b.iter(|| ::test::black_box(&bitmap).clone());
                }
            }
        }

        single_data!(create);
        single_data!(clone);
    }
}
