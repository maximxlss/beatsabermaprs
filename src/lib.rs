pub mod error;
pub mod types;

//! See [Beatmap] and [BeatmapSetMeta]

pub use types::primary::BeatmapSetMeta;
pub use types::primary::Beatmap;

// TODO: writing
// TODO: implement timing calculations

#[cfg(test)]
mod tests {
    use crate::Beatmap;
    use crate::types::{Beatmap, Level};
    use crate::error::Result;

    #[test]
    fn reading_level() -> Result<()> {
        let _level = Level::read_from_file("test_beatmap/info.dat")?;
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
}
