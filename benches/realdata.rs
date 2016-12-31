#![feature(test)]

extern crate test;
extern crate roaring_bench;

extern crate roaring;
extern crate croaring;

#[cfg(feature = "quick")]
macro_rules! single_data {
    ($cb:tt) => {
        mod $cb {
            $cb! { census_income_0(CENSUS_INCOME[0]) }
        }
    }
}

#[cfg(feature = "quick")]
macro_rules! multi_data {
    ($cb:tt) => {
        mod $cb {
            $cb! { census_income_0_1(CENSUS_INCOME[0], CENSUS_INCOME[1]) }
        }
    }
}

#[cfg(not(feature = "quick"))]
macro_rules! single_data {
    ($cb:tt) => {
        mod $cb {
            $cb! { census_income_0(CENSUS_INCOME[0]) }
            $cb! { census_income_1(CENSUS_INCOME[1]) }
            $cb! { census_income_2(CENSUS_INCOME[2]) }
            $cb! { census_income_3(CENSUS_INCOME[3]) }
            $cb! { census_income_4(CENSUS_INCOME[4]) }
            $cb! { census_income_5(CENSUS_INCOME[5]) }
        }
    }
}

#[cfg(not(feature = "quick"))]
macro_rules! multi_data {
    ($cb:tt) => {
        mod $cb {
            $cb! { census_income_0_1(CENSUS_INCOME[0], CENSUS_INCOME[1]) }
            $cb! { census_income_1_2(CENSUS_INCOME[1], CENSUS_INCOME[2]) }
            $cb! { census_income_2_3(CENSUS_INCOME[2], CENSUS_INCOME[3]) }
            $cb! { census_income_3_4(CENSUS_INCOME[3], CENSUS_INCOME[4]) }
            $cb! { census_income_4_5(CENSUS_INCOME[4], CENSUS_INCOME[5]) }
            $cb! { census_income_5_6(CENSUS_INCOME[5], CENSUS_INCOME[6]) }
        }
    }
}

mod bench {
    mod roaring {
        macro_rules! s {
            (($bitmap:ident) $($t:ident { $($body:tt)* })*) => {
                $(
                    macro_rules! $t {
                        ($n:ident($d:ident[$i:expr])) => {
                            #[bench]
                            fn $n(b: &mut ::test::Bencher) {
                                let data = &::roaring_bench::$d[$i];
                                let $bitmap = data.iter().collect::<::roaring::RoaringBitmap<u32>>();
                                b.iter(|| {
                                    let $bitmap = ::test::black_box(&$bitmap);
                                    $($body)*
                                });
                            }
                        }
                    }
                    single_data!($t);
                )*
            }
        }

        macro_rules! m {
            (($bitmap1:ident, $bitmap2:ident) $($t:ident { $($body:tt)* })*) => {
                $(
                    macro_rules! $t {
                        ($n:ident($d1:ident[$i1:expr], $d2:ident[$i2:expr])) => {
                            #[bench]
                            fn $n(b: &mut ::test::Bencher) {
                                let data1 = &::roaring_bench::$d1[$i1];
                                let data2 = &::roaring_bench::$d2[$i2];
                                let $bitmap1 = data1.iter().collect::<::roaring::RoaringBitmap<u32>>();
                                let $bitmap2 = data2.iter().collect::<::roaring::RoaringBitmap<u32>>();
                                b.iter(|| {
                                    let $bitmap1 = ::test::black_box(&$bitmap1);
                                    let $bitmap2 = ::test::black_box(&$bitmap2);
                                    $($body)*
                                });
                            }
                        }
                    }
                    multi_data!($t);
                )*
            }
        }

        macro_rules! create {
            ($n:ident($d:ident[$i:expr])) => {
                #[bench]
                fn $n(b: &mut ::test::Bencher) {
                    let data = &::roaring_bench::$d[$i];
                    b.iter(|| -> ::roaring::RoaringBitmap<u32> {
                        ::test::black_box(data).iter().collect()
                    });
                }
            }
        }

        single_data!(create);

        s! { (bitmap)
            clone { bitmap.clone() }
            iter_sum { bitmap.iter().sum::<u32>() }
            is_empty { bitmap.is_empty() }
            len { bitmap.len() }
        }

        m! { (bitmap1, bitmap2)
            or { bitmap1 | bitmap2 }
            and { bitmap1 & bitmap2 }
            xor { bitmap1 ^ bitmap2 }
            sub { bitmap1 - bitmap2 }
            is_disjoint { bitmap1.is_disjoint(bitmap2) }
            is_subset { bitmap1.is_subset(bitmap2) }
            is_superset { bitmap1.is_superset(bitmap2) }
        }
    }

    mod croaring {
        macro_rules! s {
            (($bitmap:ident) $($t:ident { $($body:tt)* })*) => {
                $(
                    macro_rules! $t {
                        ($n:ident($d:ident[$i:expr])) => {
                            #[bench]
                            fn $n(b: &mut ::test::Bencher) {
                                let data = &::roaring_bench::$d[$i];
                                let mut $bitmap = ::croaring::Bitmap::create_with_capacity(data.len() as u32);
                                $bitmap.add_many(data);
                                b.iter(|| {
                                    let $bitmap = ::test::black_box(&$bitmap);
                                    $($body)*
                                });
                            }
                        }
                    }
                    single_data!($t);
                )*
            }
        }

        macro_rules! m {
            (($bitmap1:ident, $bitmap2:ident) $($t:ident { $($body:tt)* })*) => {
                $(
                    macro_rules! $t {
                        ($n:ident($d1:ident[$i1:expr], $d2:ident[$i2:expr])) => {
                            #[bench]
                            fn $n(b: &mut ::test::Bencher) {
                                let data1 = &::roaring_bench::$d1[$i1];
                                let data2 = &::roaring_bench::$d2[$i2];
                                let mut $bitmap1 = ::croaring::Bitmap::create_with_capacity(data1.len() as u32);
                                let mut $bitmap2 = ::croaring::Bitmap::create_with_capacity(data2.len() as u32);
                                $bitmap1.add_many(data1);
                                $bitmap2.add_many(data2);
                                b.iter(|| {
                                    let $bitmap1 = ::test::black_box(&$bitmap1);
                                    let $bitmap2 = ::test::black_box(&$bitmap2);
                                    $($body)*
                                });
                            }
                        }
                    }
                    multi_data!($t);
                )*
            }
        }

        macro_rules! create {
            ($n:ident($d:ident[$i:expr])) => {
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
        single_data!(create);

        s! { (bitmap)
            clone { bitmap.clone() }
            iter_sum { bitmap.into_iter().sum::<u32>() }
            is_empty { bitmap.is_empty() }
            len { bitmap.cardinality() }
        }

        m! { (bitmap1, bitmap2)
            or { bitmap1 | bitmap2 }
            and { bitmap1 & bitmap2 }
            xor { bitmap1 ^ bitmap2 }
            sub { bitmap1 - bitmap2 }
            is_subset { bitmap1.is_subset(bitmap2) }
            is_strict_subset { bitmap1.is_strict_subset(bitmap2) }
        }
    }
}
