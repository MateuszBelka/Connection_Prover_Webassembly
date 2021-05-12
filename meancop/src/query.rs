use serde::{Deserialize, Serialize};
use cop::lean::Cuts;

// might require custom serialization/deserialization
#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    cut: bool,

    cuts: Option<Cuts>,

    pub conj: bool,

    pub nopaths: bool,

    lim: Option<usize>,

    // pub stats: Option<PathBuf>,

    pub problem: String,
}

impl Query {
    pub fn get_cuts(&self) -> Cuts {
        if self.cut {
            Cuts::max()
        } else {
            self.cuts.unwrap_or_default()
        }
    }

    pub fn depths(&self) -> Box<dyn Iterator<Item = usize>> {
        match self.lim {
            Some(lim) => Box::new(1..lim),
            None => Box::new(1..),
        }
    }
}