//! Custom build (M3): favourites, the transition playlist and the "combine
//! two" transition. The daemon turns the "playlist" and "combo" choices into
//! a real renderer transition name before each transition
//! (`concrete_shader`); the renderer rolls "random" and "random-mix" itself.

use wall_proto::transitions::{self, COMBO_PREFIX};

use super::TransitionConfig;
use crate::domain::wallpaper::{DeckOrder, transition_list};

/// Keys of the custom build, spelled out like sketch.rs (the layer guard keeps
/// the skwd-config transition key module in `value/transition.rs`; a test
/// checks these match the schema).
const FAVOURITES: &str = "transition.favourites";
const PLAYLIST: &str = "transition.playlist";
const PLAYLIST_ORDER: &str = "transition.playlistOrder";
const COMBO_FIRST: &str = "transition.comboFirst";
const COMBO_SECOND: &str = "transition.comboSecond";
const COMBO_MODE: &str = "transition.comboMode";
const COMBO_SPLIT: &str = "transition.comboSplit";

impl TransitionConfig<'_> {
    /// Starred transitions, as the user ordered them.
    pub fn favourites(&self) -> Vec<String> {
        transition_list(&self.config.str_at(FAVOURITES, ""))
    }

    /// Playlist entries the renderer understands ("combo" counts, it is
    /// expanded when it plays; unknown names are skipped).
    pub fn playlist(&self) -> Vec<String> {
        transition_list(&self.config.str_at(PLAYLIST, ""))
            .into_iter()
            .filter(|entry| playable(entry))
            .collect()
    }

    pub fn playlist_order(&self) -> DeckOrder {
        DeckOrder::parse(&self.config.str_at(PLAYLIST_ORDER, "in-order"))
    }

    /// The renderer name of the configured combination.
    pub fn combo(&self) -> String {
        transitions::combo_name(
            &self.config.str_at(COMBO_MODE, "then"),
            &self.config.str_at(COMBO_SPLIT, "lr"),
            &self.config.str_at(COMBO_FIRST, "burn"),
            &self.config.str_at(COMBO_SECOND, "ripple"),
        )
    }

    /// The transition to start now: "playlist" asks `next` for the next
    /// entry (an empty playlist plays "random"), "combo" becomes the
    /// configured combination, anything else is passed on.
    pub fn concrete_shader(
        &self,
        next: impl FnOnce(&[String], DeckOrder) -> Option<String>,
    ) -> String {
        let shader = self.shader();
        let picked = if shader == "playlist" {
            next(&self.playlist(), self.playlist_order()).unwrap_or_else(|| String::from("random"))
        } else {
            shader
        };
        if picked == "combo" { self.combo() } else { picked }
    }
}

fn playable(entry: &str) -> bool {
    entry == "combo"
        || entry == "random"
        || entry == "random-mix"
        || entry.starts_with(COMBO_PREFIX)
        || transitions::is_shader(entry)
}
