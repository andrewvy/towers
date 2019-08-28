use std::path;

use ggez::Context;
use log::*;
use warmy;

use crate::game::board::Board;
use crate::screen::Screen;
use crate::{input, resources};

pub struct World {
    pub resources: resources::Store,
    pub input: input::State,
    pub boards: Vec<Board>,
    pub screen: Screen,
}

impl World {
    pub fn new(ctx: &Context, resource_dir: &path::Path) -> Self {
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

        let screen = Screen::new(ctx);

        Self {
            resources: store,
            boards: vec![Board::default()],
            input: input::State::new(),
            screen,
        }
    }
}
