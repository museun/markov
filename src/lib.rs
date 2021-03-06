use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use snap::{read::FrameDecoder, write::FrameEncoder};
use std::{fs::File, io::BufReader, path::Path};

mod error;
pub use error::Error;

mod markov;
pub use self::markov::Markov;

mod linkset;

pub mod types {
    #[doc(inline)]
    pub use super::linkset::{Link, LinkSet, Token};
}

use types::*;

pub fn load(input: impl AsRef<Path>) -> Result<Markov, Error> {
    let input = input.as_ref();
    log::debug!(target: "brain", "loading from file: '{}'", input.display());
    let reader = BufReader::new(File::open(input)?);
    let reader = FrameDecoder::new(reader);
    let markov: Markov = bincode::deserialize_from(reader).map_err(Error::Deserialize)?;
    log::trace!(target: "brain", "done deserializing data, got: {}", markov.name);
    Ok(markov)
}

pub fn save(markov: &Markov, output: impl AsRef<Path>) -> Result<(), Error> {
    let output = output.as_ref();
    log::debug!(target: "brain", "saving '{}' to file: {}", markov.name, output.display());
    let writer = File::create(output)?;
    let writer = FrameEncoder::new(writer);
    bincode::serialize_into(writer, &markov).map_err(Error::Serialize)?;
    log::trace!(target: "brain", "done serializing data");
    Ok(())
}
