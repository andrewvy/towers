use crate::{input, resources, util};

use log::*;
use warmy;

use std::path;

pub struct World {
    pub resources: resources::Store,
    pub input: input::State,
}

impl World {
    pub fn new(resource_dir: &path::Path) -> Self {
        // We to bridge the gap between ggez and warmy path
        // handling here; ggez assumes its own absolute paths, warmy
        // assumes system-absolute paths; so, we make warmy look in
        // the specified resource dir (normally
        // $CARGO_MANIFEST_DIR/resources) or the ggez default resource
        // dir.
        //
        // TODO: ...though this doesn't SEEM to quite do reloading right, so
        // work on it more.
        info!("Setting up resource path: {:?}", resource_dir);
        let opt = warmy::StoreOpt::default().set_root(resource_dir);
        let store = warmy::Store::new(opt)
            .expect("Could not create asset store?  Does the directory exist?");

        Self {
            resources: store,
            input: input::State::new(),
        }
    }
}
