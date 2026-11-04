use serde_json::Value;

use super::Config;

/// "Sketch before transition" settings (custom build; not in the upstream schema).
pub const SKETCH_ENABLED: &str = "transition.sketchEnabled";
pub const SKETCH_STYLE: &str = "transition.sketchStyle";
pub const SKETCH_DURATION_MS: &str = "transition.sketchDurationMs";
pub const SKETCH_VIDEOS: &str = "transition.sketchVideos";
pub const SKETCH_SCENES: &str = "transition.sketchScenes";
/// Where transitions start: default, centre, random or dense.
pub const ORIGIN: &str = "transition.origin";

pub const SKETCH_STYLES: [&str; 7] =
    ["pencil", "digital", "chalk", "blueprint", "ink-brush", "charcoal", "notebook"];
pub const ORIGINS: [&str; 4] = ["default", "centre", "random", "dense"];

#[derive(Clone, Copy)]
pub struct TransitionConfig<'a> {
    config: &'a Config,
}

impl<'a> TransitionConfig<'a> {
    pub(super) fn new(config: &'a Config) -> Self {
        Self { config }
    }

    fn root(&self) -> &Value {
        self.config.root()
    }

    skwd_config::getters! {
        enabled: on_unless_off(skwd_config::keys::transition::ENABLED);
        sand_sharp: bool(skwd_config::keys::transition::SAND_SHARP, false);
    }

    pub fn sand_quality(&self) -> String {
        let quality = self.config.str_at(skwd_config::keys::transition::SAND_QUALITY, "auto");
        if quality == "full" || quality == "low" { quality } else { String::from("auto") }
    }

    pub fn sand_scope(&self) -> String {
        let scope = self.config.str_at(skwd_config::keys::transition::SAND_SCOPE, "all");
        if scope == "primary" { scope } else { String::from("all") }
    }

    pub fn scope(&self, shader: &str) -> String {
        let scope = self
            .config
            .get(skwd_config::keys::transition::SHADER_SCOPES)
            .and_then(Value::as_object)
            .and_then(|scopes| scopes.get(shader))
            .and_then(Value::as_str);
        match scope {
            Some("primary") => String::from("primary"),
            Some("all") => String::from("all"),
            _ if shader.starts_with("sand-") => self.sand_scope(),
            _ => String::from("all"),
        }
    }

    pub fn sand_primary(&self) -> String {
        self.config.str_at(skwd_config::keys::transition::SAND_PRIMARY, "")
    }

    pub fn fps(&self) -> u32 {
        skwd_config::schema::read_number(self.root(), skwd_config::keys::transition::FPS)
            .unwrap_or(0.0) as u32
    }

    pub fn output_fps(&self, refresh_mhz: i32) -> u32 {
        let fps = self.fps();
        let limit = if fps == 0 { if refresh_mhz > 0 { 1000 } else { 60 } } else { fps };
        crate::outputs::effective_fps(limit, refresh_mhz)
    }

    pub fn sand_fps(&self) -> String {
        let fps = self
            .config
            .get(skwd_config::keys::transition::SAND_FPS)
            .and_then(Value::as_f64)
            .map_or(0, |number| number.max(0.0) as u32);
        if fps > 0 { fps.to_string() } else { String::from("auto") }
    }

    pub fn shader(&self) -> String {
        let shader = self.config.str_at(skwd_config::keys::transition::SHADER, "random");
        if shader.is_empty() { "random".to_string() } else { shader }
    }

    /// The transition time including the sketch, when one is drawn first.
    pub fn duration_ms(&self) -> u64 {
        let base = self.base_duration_ms();
        if base >= 10_000 { base } else { (base + self.sketch_ms()).min(10_000) }
    }

    /// The configured transition time, without the sketch.
    pub fn base_duration_ms(&self) -> u64 {
        self.config
            .get(skwd_config::keys::transition::DURATION_MS)
            .and_then(Value::as_f64)
            .map_or(600, |number| number.clamp(50.0, 10000.0) as u64)
    }

    fn flag_or(&self, key: &str, default: bool) -> bool {
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
        if !self.flag_or(SKETCH_ENABLED, false) || !self.active() {
            return 0;
        }
        self.config
            .get(SKETCH_DURATION_MS)
            .and_then(Value::as_f64)
            .filter(|number| number.is_finite())
            .map_or(6000, |number| number.clamp(1000.0, 20_000.0) as u64)
    }

    pub fn active(&self) -> bool {
        self.enabled() && !self.config.renderer().performance_mode()
    }
}
