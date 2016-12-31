#![feature(test)]

extern crate test;
extern crate roaring_bench;

extern crate roaring;
extern crate croaring;

macro_rules! single_data {
    ($cb:tt) => {
        mod $cb {
            $cb!(census_income_0, CENSUS_INCOME[0]);
            #[cfg(not(feature = "quick"))]
            $cb!(census_income_1, CENSUS_INCOME[1]);
            #[cfg(not(feature = "quick"))]
            $cb!(census_income_2, CENSUS_INCOME[2]);
            #[cfg(not(feature = "quick"))]
            $cb!(census_income_3, CENSUS_INCOME[3]);
            #[cfg(not(feature = "quick"))]
            $cb!(census_income_4, CENSUS_INCOME[4]);
            #[cfg(not(feature = "quick"))]
            $cb!(census_income_5, CENSUS_INCOME[5]);
        }
    }
}

macro_rules! multi_data {
    ($cb:tt) => {
        mod $cb {
            $cb!(census_income_0_1, CENSUS_INCOME[0], CENSUS_INCOME[1]);
            #[cfg(not(feature = "quick"))]
            $cb!(census_income_1_2, CENSUS_INCOME[1], CENSUS_INCOME[2]);
            #[cfg(not(feature = "quick"))]
            $cb!(census_income_2_3, CENSUS_INCOME[2], CENSUS_INCOME[3]);
            #[cfg(not(feature = "quick"))]
            $cb!(census_income_3_4, CENSUS_INCOME[3], CENSUS_INCOME[4]);
            #[cfg(not(feature = "quick"))]
            $cb!(census_income_4_5, CENSUS_INCOME[4], CENSUS_INCOME[5]);
            #[cfg(not(feature = "quick"))]
            $cb!(census_income_5_6, CENSUS_INCOME[5], CENSUS_INCOME[6]);
        }
    }
}

mod bench {
    mod roaring {
        macro_rules! create {
            ($n:ident, $d:ident[$i:expr]) => {
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
            ($n:ident, $d:ident[$i:expr]) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data = &::roaring_bench::$d[$i];
                    let bitmap = data.iter().collect::<::roaring::RoaringBitmap<u32>>();
                    b.iter(|| ::test::black_box(&bitmap).clone());
                }
            }
        }

        macro_rules! iter_sum {
            ($n:ident, $d:ident[$i:expr]) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data = &::roaring_bench::$d[$i];
                    let bitmap = data.iter().collect::<::roaring::RoaringBitmap<u32>>();
                    b.iter(|| ::test::black_box(&bitmap).iter().sum::<u32>());
                }
            }
        }

        macro_rules! or {
            ($n:ident, $d1:ident[$i1:expr], $d2:ident[$i2:expr]) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data1 = &::roaring_bench::$d1[$i1];
                    let data2 = &::roaring_bench::$d2[$i2];
                    let bitmap1 = data1.iter().collect::<::roaring::RoaringBitmap<u32>>();
                    let bitmap2 = data2.iter().collect::<::roaring::RoaringBitmap<u32>>();
                    b.iter(|| ::test::black_box(&bitmap1) | ::test::black_box(&bitmap2));
                }
            }
        }

        macro_rules! and {
            ($n:ident, $d1:ident[$i1:expr], $d2:ident[$i2:expr]) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data1 = &::roaring_bench::$d1[$i1];
                    let data2 = &::roaring_bench::$d2[$i2];
                    let bitmap1 = data1.iter().collect::<::roaring::RoaringBitmap<u32>>();
                    let bitmap2 = data2.iter().collect::<::roaring::RoaringBitmap<u32>>();
                    b.iter(|| ::test::black_box(&bitmap1) & ::test::black_box(&bitmap2));
                }
            }
        }

        macro_rules! xor {
            ($n:ident, $d1:ident[$i1:expr], $d2:ident[$i2:expr]) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data1 = &::roaring_bench::$d1[$i1];
                    let data2 = &::roaring_bench::$d2[$i2];
                    let bitmap1 = data1.iter().collect::<::roaring::RoaringBitmap<u32>>();
                    let bitmap2 = data2.iter().collect::<::roaring::RoaringBitmap<u32>>();
                    b.iter(|| ::test::black_box(&bitmap1) ^ ::test::black_box(&bitmap2));
                }
            }
        }

        macro_rules! sub {
            ($n:ident, $d1:ident[$i1:expr], $d2:ident[$i2:expr]) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data1 = &::roaring_bench::$d1[$i1];
                    let data2 = &::roaring_bench::$d2[$i2];
                    let bitmap1 = data1.iter().collect::<::roaring::RoaringBitmap<u32>>();
                    let bitmap2 = data2.iter().collect::<::roaring::RoaringBitmap<u32>>();
                    b.iter(|| ::test::black_box(&bitmap1) - ::test::black_box(&bitmap2));
                }
            }
        }

        single_data!(create);
        single_data!(clone);
        single_data!(iter_sum);
        multi_data!(or);
        multi_data!(and);
        multi_data!(xor);
        multi_data!(sub);
    }

    mod croaring {
        macro_rules! create {
            ($n:ident, $d:ident[$i:expr]) => {
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
            ($n:ident, $d:ident[$i:expr]) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data = &::roaring_bench::$d[$i];
                    let mut bitmap = ::croaring::Bitmap::create_with_capacity(data.len() as u32);
                    bitmap.add_many(data);
                    b.iter(|| ::test::black_box(&bitmap).clone());
                }
            }
        }

        macro_rules! iter_sum {
            ($n:ident, $d:ident[$i:expr]) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data = &::roaring_bench::$d[$i];
                    let mut bitmap = ::croaring::Bitmap::create_with_capacity(data.len() as u32);
                    bitmap.add_many(data);
                    b.iter(|| ::test::black_box(&bitmap).into_iter().sum::<u32>());
                }
            }
        }

        macro_rules! or {
            ($n:ident, $d1:ident[$i1:expr], $d2:ident[$i2:expr]) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data1 = &::roaring_bench::$d1[$i1];
                    let mut bitmap1 = ::croaring::Bitmap::create_with_capacity(data1.len() as u32);
                    bitmap1.add_many(data1);
                    let data2 = &::roaring_bench::$d2[$i2];
                    let mut bitmap2 = ::croaring::Bitmap::create_with_capacity(data2.len() as u32);
                    bitmap2.add_many(data2);
                    b.iter(|| ::test::black_box(&bitmap1) | ::test::black_box(&bitmap2));
                }
            }
        }

        macro_rules! and {
            ($n:ident, $d1:ident[$i1:expr], $d2:ident[$i2:expr]) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data1 = &::roaring_bench::$d1[$i1];
                    let mut bitmap1 = ::croaring::Bitmap::create_with_capacity(data1.len() as u32);
                    bitmap1.add_many(data1);
                    let data2 = &::roaring_bench::$d2[$i2];
                    let mut bitmap2 = ::croaring::Bitmap::create_with_capacity(data2.len() as u32);
                    bitmap2.add_many(data2);
                    b.iter(|| ::test::black_box(&bitmap1) & ::test::black_box(&bitmap2));
                }
            }
        }

        macro_rules! xor {
            ($n:ident, $d1:ident[$i1:expr], $d2:ident[$i2:expr]) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data1 = &::roaring_bench::$d1[$i1];
                    let mut bitmap1 = ::croaring::Bitmap::create_with_capacity(data1.len() as u32);
                    bitmap1.add_many(data1);
                    let data2 = &::roaring_bench::$d2[$i2];
                    let mut bitmap2 = ::croaring::Bitmap::create_with_capacity(data2.len() as u32);
                    bitmap2.add_many(data2);
                    b.iter(|| ::test::black_box(&bitmap1) ^ ::test::black_box(&bitmap2));
                }
            }
        }

        macro_rules! sub {
            ($n:ident, $d1:ident[$i1:expr], $d2:ident[$i2:expr]) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data1 = &::roaring_bench::$d1[$i1];
                    let mut bitmap1 = ::croaring::Bitmap::create_with_capacity(data1.len() as u32);
                    bitmap1.add_many(data1);
                    let data2 = &::roaring_bench::$d2[$i2];
                    let mut bitmap2 = ::croaring::Bitmap::create_with_capacity(data2.len() as u32);
                    bitmap2.add_many(data2);
                    b.iter(|| ::test::black_box(&bitmap1) - ::test::black_box(&bitmap2));
                }
            }
        }

        single_data!(create);
        single_data!(clone);
        single_data!(iter_sum);
        multi_data!(or);
        multi_data!(and);
        multi_data!(xor);
        multi_data!(sub);
    }
}
