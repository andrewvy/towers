//! Example of integrating ggez types with the `warmy` resource loader.
//! @todo(vy): Learn how to do hot resource reloading with `warmy`.

use std::path;

use ggez::{self, graphics};
use log::*;
use warmy;

use crate::game::{mob, unit};
use crate::types::Error;

/// Again, because `warmy` assumes direct filesystem dirs
/// and ggez assumes all its resources live in a specific
/// (relative) location, we make our own key type here which
/// doesn't get `warmy`'s root path attached to it like its
/// `SimpleKey` types do.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Path(path::PathBuf),
}

impl From<&path::Path> for Key {
    fn from(p: &path::Path) -> Self {
        Key::Path(p.to_owned())
    }
}

impl Key {
    pub fn from_path<P>(p: P) -> Self
    where
        P: AsRef<path::Path>,
    {
        Key::Path(p.as_ref().to_owned())
    }
}

impl warmy::key::Key for Key {
    fn prepare_key(self, _root: &path::Path) -> Self {
        self
    }
}

/// Store and Storage are different things in `warmy`; the `Store`
/// is what actually stores things, and the `Storage` is I think
/// a handle to it.
pub type Store = warmy::Store<ggez::Context, Key>;
type Storage = warmy::Storage<ggez::Context, Key>;
pub type Loaded<T> = warmy::Loaded<T, Key>;

/// A wrapper for a ggez Image, so we can implement warmy's `Load` trait on it.
#[derive(Debug, Clone)]
pub struct Image(pub graphics::Image);

#[derive(Debug)]
pub struct Unit(pub unit::Unit);

#[derive(Debug)]
pub struct MobDefinition(pub mob::MobDefinition);

/// And, here actually tell Warmy how to load things.
impl warmy::Load<ggez::Context, Key> for Image {
    type Error = Error;
    fn load(
        key: Key,
        _storage: &mut Storage,
        ctx: &mut ggez::Context,
    ) -> Result<Loaded<Self>, Self::Error> {
        debug!("Loading image {:?}", key);

        match key {
            Key::Path(path) => graphics::Image::new(ctx, path)
                .map(|x| warmy::Loaded::from(Image(x)))
                .map_err(|e| Error::GgezError(e)),
        }
    }
}

impl warmy::Load<ggez::Context, Key> for Unit {
    type Error = Error;
    fn load(
        key: Key,
        _storage: &mut Storage,
        ctx: &mut ggez::Context,
    ) -> Result<Loaded<Self>, Self::Error> {
        debug!("Loading unit {:?}", key);

        match key {
            Key::Path(path) => {
                let file = ggez::filesystem::open(ctx, &path).map_err(|e| Error::GgezError(e))?;
                let ron = ron::de::from_reader(file).map_err(|e| Error::DeserializeError(e))?;

                Ok(warmy::Loaded::from(Unit(ron)))
            }
        }
    }
}

impl warmy::Load<ggez::Context, Key> for MobDefinition {
    type Error = Error;
    fn load(
        key: Key,
        _storage: &mut Storage,
        ctx: &mut ggez::Context,
    ) -> Result<Loaded<Self>, Self::Error> {
        debug!("Loading unit {:?}", key);

        match key {
            Key::Path(path) => {
                let file = ggez::filesystem::open(ctx, &path).map_err(|e| Error::GgezError(e))?;
                let ron = ron::de::from_reader(file).map_err(|e| Error::DeserializeError(e))?;

                Ok(warmy::Loaded::from(MobDefinition(ron)))
            }
        }
    }
}
