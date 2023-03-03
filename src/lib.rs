//! See [Beatmap] and [BeatmapSetMeta]

pub mod error;
pub mod types;

pub use types::primary::BeatmapSetMeta;
pub use types::primary::Beatmap;

// TODO: writing
// TODO: implement timing calculations

#[cfg(test)]
mod tests {
    use std::fs::File;
    use crate::{Beatmap, BeatmapSetMeta};
    use crate::error::Result;
    use std::io::{BufReader, Read};

    #[test]
    fn reading_level() -> Result<()> {
        let _level = BeatmapSetMeta::read_from_file("test_beatmap/info.dat")?;
        Ok(())
    }

    #[test]
    fn reading_beatmap_v2() -> Result<()> {
        let _beatmap = Beatmap::read_from_file("test_beatmap/beatmapv2.dat")?;
        Ok(())
    }

    #[test]
    fn reading_beatmap_v3() -> Result<()> {
        let _beatmap = Beatmap::read_from_file("test_beatmap/beatmapv3.dat")?;
        Ok(())
    }

    #[ignore]
    #[test]
    fn reading_your_beatmap() -> Result<()> {
        let file = File::open("test_beatmap.zip")?;
        let reader = BufReader::new(file);
        let mut archive = zip::ZipArchive::new(reader).unwrap();
        let mut meta_file = vec![];
        archive.by_name("Info.dat").unwrap().read_to_end(&mut meta_file).unwrap();
        let meta = BeatmapSetMeta::read_from_str(&String::from_utf8_lossy(&meta_file.as_slice())).unwrap();

        let filenames = meta.difficulty_sets
            .into_iter()
            .map(|x| x.beatmaps)
            .flatten()
            .map(|x| x.filename);
        for filename in filenames {
            let mut map_file = vec![];
            archive.by_name(&filename).unwrap().read_to_end(&mut map_file).unwrap();
            let _map = Beatmap::read_from_str(&String::from_utf8_lossy(&map_file.as_slice())).unwrap();
        }
        Ok(())
    }
}
