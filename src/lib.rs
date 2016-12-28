#[macro_use] extern crate error_chain;
#[macro_use] extern crate lazy_static;
extern crate zip;

use std::io::Read;

lazy_static! {
    pub static ref CENSUS_INCOME: Vec<Vec<u32>> = load_census_income().unwrap();
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Zip(zip::result::ZipError);
        ParseInt(std::num::ParseIntError);
    }
    errors {
        MissingRecord {
            description("missing record")
        }
    }
}

fn load(bytes: &[u8]) -> Result<Vec<Vec<u32>>> {
    let cursor = std::io::Cursor::new(bytes);
    let mut zip = zip::ZipArchive::new(cursor)?;

    (0..zip.len())
        .map(|i| {
            zip.by_index(i)
                .map_err(Error::from)
                .and_then(|mut file| {
                    let mut str = String::with_capacity(file.size() as usize);
                    file.read_to_string(&mut str)?;
                    str.trim().split(",").map(|s| s.parse::<u32>().map_err(Error::from)).collect()
                })
        })
        .collect()
}

fn load_census_income() -> Result<Vec<Vec<u32>>> {
    load(include_bytes!("../real-roaring-datasets/census-income.zip"))
}

fn load_census_income_srt() -> Result<Vec<Vec<u32>>> {
    load(include_bytes!("../real-roaring-datasets/census-income_srt.zip"))
}

fn load_census1881() -> Result<Vec<Vec<u32>>> {
    load(include_bytes!("../real-roaring-datasets/census1881.zip"))
}

fn load_census1881_srt() -> Result<Vec<Vec<u32>>> {
    load(include_bytes!("../real-roaring-datasets/census1881_srt.zip"))
}

fn load_dimension_003() -> Result<Vec<Vec<u32>>> {
    load(include_bytes!("../real-roaring-datasets/dimension_003.zip"))
}

fn load_dimension_008() -> Result<Vec<Vec<u32>>> {
    load(include_bytes!("../real-roaring-datasets/dimension_008.zip"))
}

fn load_dimension_033() -> Result<Vec<Vec<u32>>> {
    load(include_bytes!("../real-roaring-datasets/dimension_033.zip"))
}

fn load_uscensus2000() -> Result<Vec<Vec<u32>>> {
    load(include_bytes!("../real-roaring-datasets/uscensus2000.zip"))
}

fn load_weather_sept_85() -> Result<Vec<Vec<u32>>> {
    load(include_bytes!("../real-roaring-datasets/weather_sept_85.zip"))
}

fn load_weather_sept_85_srt() -> Result<Vec<Vec<u32>>> {
    load(include_bytes!("../real-roaring-datasets/weather_sept_85_srt.zip"))
}

fn load_wikileaks_noquotes() -> Result<Vec<Vec<u32>>> {
    load(include_bytes!("../real-roaring-datasets/wikileaks-noquotes.zip"))
}

fn load_wikileaks_noquotes_srt() -> Result<Vec<Vec<u32>>> {
    load(include_bytes!("../real-roaring-datasets/wikileaks-noquotes_srt.zip"))
}
