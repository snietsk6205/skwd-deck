//! "Sketch before transition" and transition start-point settings of the
//! custom build. The daemon passes them to every renderer in the
//! `SKWD_PAPER_SKETCH` and `SKWD_PAPER_TRANSITION_ORIGIN` environment values.

use serde_json::Value;

use super::TransitionConfig;

/// "Sketch before transition" settings (custom build; not in the upstream schema).
const SKETCH_ENABLED: &str = "transition.sketchEnabled";
const SKETCH_STYLE: &str = "transition.sketchStyle";
const SKETCH_DURATION_MS: &str = "transition.sketchDurationMs";
const SKETCH_VIDEOS: &str = "transition.sketchVideos";
const SKETCH_SCENES: &str = "transition.sketchScenes";
/// Where transitions start: default, centre, random or dense.
const ORIGIN: &str = "transition.origin";
/// Low-power mode (custom build) turns the sketch off.
const LOW_POWER: &str = "performance.lowPower";

const SKETCH_STYLES: [&str; 7] =
    ["pencil", "digital", "chalk", "blueprint", "ink-brush", "charcoal", "notebook"];
const ORIGINS: [&str; 4] = ["default", "centre", "random", "dense"];

impl TransitionConfig<'_> {
    /// The transition time including the sketch, when one is drawn first
    /// (`base_duration_ms` is the configured time without it).
    pub fn duration_ms(&self) -> u64 {
        let base = self.base_duration_ms();
        if base >= 10_000 { base } else { (base + self.sketch_ms()).min(10_000) }
    }

    fn flag_or(self, key: &str, default: bool) -> bool {
        self.config.get(key).and_then(Value::as_bool).unwrap_or(default)
    }

    pub fn sketch_style(&self) -> String {
        let style = self.config.str_at(SKETCH_STYLE, "pencil");
        if SKETCH_STYLES.contains(&style.as_str()) { style } else { String::from("pencil") }
    }

    /// Renderer environment value for the sketch (`SKWD_PAPER_SKETCH`), or an
    /// empty string when no sketch is drawn.
    pub fn sketch_env(&self) -> String {
        let sketch = self.sketch_ms();
        if sketch == 0 || self.base_duration_ms() >= 10_000 {
            return String::new();
        }
        format!(
            "style={};ms={sketch};base={};videos={};scenes={}",
            self.sketch_style(),
            self.base_duration_ms(),
            u8::from(self.flag_or(SKETCH_VIDEOS, true)),
            u8::from(self.flag_or(SKETCH_SCENES, true)),
        )
    }

    /// Transition start point (`SKWD_PAPER_TRANSITION_ORIGIN`).
    pub fn origin(&self) -> String {
        let origin = self.config.str_at(ORIGIN, "default");
        if ORIGINS.contains(&origin.as_str()) { origin } else { String::from("default") }
    }

    /// Extra time for the "sketch before transition" prelude (0 when off).
    /// The renderer draws the sketch during this time, then runs the chosen
    /// transition from the finished sketch. Sketch plus transition is capped
    /// at the 10 s the renderer protocol accepts.
    pub fn sketch_ms(&self) -> u64 {
        // Low-power mode skips the sketch (it is the costliest transition prelude).
        if !self.flag_or(SKETCH_ENABLED, false) || !self.active() || self.flag_or(LOW_POWER, false)
        {
            return 0;
        }
        self.config
            .get(SKETCH_DURATION_MS)
            .and_then(Value::as_f64)
            .filter(|number| number.is_finite())
            .map_or(6000, |number| number.clamp(1000.0, 20_000.0) as u64)
    }
}
