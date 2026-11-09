#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    Random,
    Playlist,
    Combo,
    Fade,
    Wipe,
    Warp,
    Break,
    Light,
    Shapes,
    Sand,
}

impl Family {
    pub const fn key(self) -> &'static str {
        match self {
            Family::Random => "random",
            Family::Playlist => "playlist",
            Family::Combo => "combo",
            Family::Fade => "fade",
            Family::Wipe => "wipe",
            Family::Warp => "warp",
            Family::Break => "break",
            Family::Light => "light",
            Family::Shapes => "shapes",
            Family::Sand => "sand",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Family::Random => "Random",
            Family::Playlist => "Playlist",
            Family::Combo => "Combine two",
            Family::Fade => "Fade",
            Family::Wipe => "Wipe",
            Family::Warp => "Warp",
            Family::Break => "Break up",
            Family::Light => "Light",
            Family::Shapes => "Shapes",
            Family::Sand => "Sand",
        }
    }

    pub const fn desc(self) -> &'static str {
        match self {
            Family::Random => "A different transition every time.",
            Family::Playlist => {
                "Several transitions in turns: in order, shuffled or random without repeats."
            }
            Family::Combo => {
                "Two transitions at once: one after the other, blended, split screen, one as a mask, and more."
            }
            Family::Fade => "Dissolves and blends, no geometry.",
            Family::Wipe => "A shape or edge sweeps across the screen.",
            Family::Warp => "The image bends, stretches or ripples.",
            Family::Break => "The image shatters, pixelates or smears.",
            Family::Light => "Glows, flashes and beams of light.",
            Family::Shapes => "Tiles, dots and blades open in patterns.",
            Family::Sand => "Every pixel becomes a grain and flies into a shape.",
        }
    }
}

pub const FAMILIES: [Family; 10] = [
    Family::Random,
    Family::Playlist,
    Family::Combo,
    Family::Fade,
    Family::Wipe,
    Family::Warp,
    Family::Break,
    Family::Light,
    Family::Shapes,
    Family::Sand,
];

#[derive(Debug, Clone, Copy)]
pub struct TransitionSpec {
    pub key: &'static str,
    pub family: Family,
    /// Custom build (M3): one plain sentence for Settings and its search.
    pub desc: &'static str,
}

/// Custom build (M3): entries that are not a renderer shader. "random" and
/// "random-mix" are rolled by the renderer; the daemon turns "playlist" and
/// "combo" into a real transition name before it starts one.
pub const PSEUDO_KEYS: [&str; 4] = ["random", "random-mix", "playlist", "combo"];

/// Custom build (M3): prefix of a combination name, `combo:<mode>:<first>:<second>`.
pub const COMBO_PREFIX: &str = "combo:";

/// Custom build (M3): the ways to combine two transitions (combo mode key, label).
pub const COMBO_MODES: [(&str, &str); 7] = [
    ("then", "One after the other"),
    ("blend", "Blend"),
    ("split", "Split screen"),
    ("mask", "First is a mask"),
    ("sweep", "Sweep"),
    ("light", "Both, lighter wins"),
    ("dark", "Both, darker wins"),
];

/// Custom build (M3): split-screen shapes (renderer mode suffix, label).
pub const COMBO_SPLITS: [(&str, &str); 5] = [
    ("lr", "Left | right"),
    ("tb", "Top / bottom"),
    ("diagonal", "Diagonal"),
    ("circle", "Circle"),
    ("checker", "Checker"),
];

/// Custom build (M3): playlist orders (key, label).
pub const PLAYLIST_ORDERS: [(&str, &str); 3] =
    [("in-order", "In order"), ("shuffle", "Shuffled"), ("no-repeat", "Random, no repeats")];

pub const TRANSITIONS: [TransitionSpec; 81] = [
    TransitionSpec {
        key: "random",
        family: Family::Random,
        desc: "A different transition every time.",
    },
    TransitionSpec {
        key: "chromatic-bloom",
        family: Family::Light,
        desc: "Colour fringes bloom out of the middle and wash into the new picture.",
    },
    TransitionSpec {
        key: "circle-crop",
        family: Family::Wipe,
        desc: "A circle closes on the old picture and opens on the new one.",
    },
    TransitionSpec {
        key: "colour-distance",
        family: Family::Fade,
        desc: "Pixels change in order of how different their colours are.",
    },
    TransitionSpec { key: "crossfade", family: Family::Fade, desc: "A plain, smooth cross-fade." },
    TransitionSpec {
        key: "crosswarp",
        family: Family::Warp,
        desc: "The pictures stretch sideways into each other.",
    },
    TransitionSpec {
        key: "crosshatch",
        family: Family::Wipe,
        desc: "Criss-cross lines spread out from a point.",
    },
    TransitionSpec {
        key: "directional",
        family: Family::Wipe,
        desc: "The new picture pushes the old one off the screen.",
    },
    TransitionSpec {
        key: "directional-scaled",
        family: Family::Wipe,
        desc: "A push where the old picture shrinks away as it leaves.",
    },
    TransitionSpec {
        key: "directional-wipe",
        family: Family::Wipe,
        desc: "A soft straight edge sweeps across the screen.",
    },
    TransitionSpec {
        key: "fadecolor",
        family: Family::Fade,
        desc: "Fades through a solid colour.",
    },
    TransitionSpec {
        key: "glitch",
        family: Family::Break,
        desc: "Digital glitches tear the old picture apart.",
    },
    TransitionSpec {
        key: "glitch-displace",
        family: Family::Break,
        desc: "Blocks jump and slide like a broken signal.",
    },
    TransitionSpec {
        key: "heat-melt",
        family: Family::Warp,
        desc: "The old picture melts away like wax in heat.",
    },
    TransitionSpec {
        key: "ink-splash",
        family: Family::Break,
        desc: "A splash of ink spreads from a point.",
    },
    TransitionSpec {
        key: "inkwell-drop",
        family: Family::Break,
        desc: "A drop of ink falls and blooms across the screen.",
    },
    TransitionSpec { key: "iris", family: Family::Wipe, desc: "A round iris opens from a point." },
    TransitionSpec {
        key: "morph",
        family: Family::Warp,
        desc: "The two pictures flow into each other.",
    },
    TransitionSpec {
        key: "mosaic-tumble",
        family: Family::Shapes,
        desc: "Mosaic tiles tumble over one by one.",
    },
    TransitionSpec {
        key: "parametric-glitch",
        family: Family::Break,
        desc: "Bands of colour noise sweep across.",
    },
    TransitionSpec {
        key: "perlin",
        family: Family::Fade,
        desc: "A cloudy noise pattern dissolves the picture.",
    },
    TransitionSpec {
        key: "pixelate",
        family: Family::Break,
        desc: "The picture breaks into big pixels and rebuilds.",
    },
    TransitionSpec {
        key: "pixelfade-wave",
        family: Family::Fade,
        desc: "A wave of fading pixels rolls across.",
    },
    TransitionSpec {
        key: "plasma-flow",
        family: Family::Break,
        desc: "Swirling plasma eats away the old picture.",
    },
    TransitionSpec {
        key: "polka-dots-curtain",
        family: Family::Shapes,
        desc: "Growing polka dots cover the screen.",
    },
    TransitionSpec {
        key: "puzzle-right",
        family: Family::Shapes,
        desc: "Puzzle pieces slide in from the right.",
    },
    TransitionSpec {
        key: "randomsquares",
        family: Family::Shapes,
        desc: "Squares flip over in random order.",
    },
    TransitionSpec {
        key: "sand-bloom",
        family: Family::Sand,
        desc: "Grains burst out like a blooming flower.",
    },
    TransitionSpec { key: "sand-donut", family: Family::Sand, desc: "Grains spin round a ring." },
    TransitionSpec {
        key: "sand-galaxy",
        family: Family::Sand,
        desc: "Grains swirl into a spiral galaxy.",
    },
    TransitionSpec {
        key: "sand-globe",
        family: Family::Sand,
        desc: "Grains gather into a turning globe.",
    },
    TransitionSpec {
        key: "sand-helix",
        family: Family::Sand,
        desc: "Grains twist into a double helix.",
    },
    TransitionSpec {
        key: "sand-maelstrom",
        family: Family::Sand,
        desc: "Grains are pulled into a whirlpool.",
    },
    TransitionSpec {
        key: "sand-mobius",
        family: Family::Sand,
        desc: "Grains travel along a Mobius strip.",
    },
    TransitionSpec {
        key: "sand-murmuration",
        family: Family::Sand,
        desc: "Grains fly like a flock of starlings.",
    },
    TransitionSpec {
        key: "sand-tornado",
        family: Family::Sand,
        desc: "Grains are lifted into a tornado.",
    },
    TransitionSpec {
        key: "smoke",
        family: Family::Break,
        desc: "The old picture drifts away as smoke.",
    },
    TransitionSpec {
        key: "soft-warp-fade",
        family: Family::Fade,
        desc: "A gentle wobble while the pictures fade.",
    },
    TransitionSpec {
        key: "voronoi-shatter",
        family: Family::Break,
        desc: "The picture shatters into cells.",
    },
    TransitionSpec {
        key: "zoom-blur-pull",
        family: Family::Warp,
        desc: "A zoom blur pulls you into the new picture.",
    },
    // Custom build.
    TransitionSpec {
        key: "ripple",
        family: Family::Warp,
        desc: "Water rings spread from a point and carry the new picture in.",
    },
    TransitionSpec {
        key: "honeycomb",
        family: Family::Shapes,
        desc: "Hexagon tiles turn over in a wave from a point.",
    },
    TransitionSpec {
        key: "burn",
        family: Family::Break,
        desc: "The old picture burns away along glowing embers.",
    },
    TransitionSpec {
        key: "page-curl",
        family: Family::Warp,
        desc: "The old picture curls away like a page.",
    },
    TransitionSpec {
        key: "blinds",
        family: Family::Wipe,
        desc: "Slanted blinds open one after another.",
    },
    TransitionSpec {
        key: "slide",
        family: Family::Wipe,
        desc: "The new picture slides in over the old one.",
    },
    TransitionSpec {
        key: "vortex",
        family: Family::Warp,
        desc: "Swirls out of the old picture into the new one.",
    },
    // Custom build (M3).
    TransitionSpec {
        key: "light-leak",
        family: Family::Light,
        desc: "A warm film light leak sweeps across and leaves the new picture behind.",
    },
    TransitionSpec {
        key: "flashbulb",
        family: Family::Light,
        desc: "A camera flash, then the new picture with a fading afterimage.",
    },
    TransitionSpec {
        key: "aurora",
        family: Family::Light,
        desc: "Curtains of northern lights drift down and bring the new picture.",
    },
    TransitionSpec {
        key: "prism",
        family: Family::Light,
        desc: "The colours split apart like light through a prism, then meet again.",
    },
    TransitionSpec {
        key: "god-rays",
        family: Family::Light,
        desc: "Beams of light burst out of a point.",
    },
    TransitionSpec {
        key: "neon-edges",
        family: Family::Light,
        desc: "The outlines of the new picture light up like neon, then it fills in.",
    },
    TransitionSpec {
        key: "sunrise",
        family: Family::Light,
        desc: "Dusk falls, then a glowing horizon rises with the new picture.",
    },
    TransitionSpec {
        key: "diamonds",
        family: Family::Shapes,
        desc: "Diamond tiles pop open in a wave from a point.",
    },
    TransitionSpec {
        key: "triangles",
        family: Family::Shapes,
        desc: "Triangle tiles flip over from left to right.",
    },
    TransitionSpec {
        key: "checkerboard",
        family: Family::Shapes,
        desc: "The dark squares of a checkerboard open first, then the light ones.",
    },
    TransitionSpec {
        key: "stripes",
        family: Family::Shapes,
        desc: "Stripes slide up and down in turn and swap the pictures.",
    },
    TransitionSpec {
        key: "halftone",
        family: Family::Shapes,
        desc: "The new picture is printed as halftone dots, darkest parts first.",
    },
    TransitionSpec {
        key: "spiral-tiles",
        family: Family::Shapes,
        desc: "Square tiles turn up in a spiral from the middle.",
    },
    TransitionSpec {
        key: "shutter",
        family: Family::Shapes,
        desc: "Camera aperture blades close and open on the new picture.",
    },
    TransitionSpec {
        key: "kaleidoscope",
        family: Family::Shapes,
        desc: "The picture folds into a turning kaleidoscope and unfolds.",
    },
    TransitionSpec {
        key: "liquid",
        family: Family::Warp,
        desc: "The picture turns to liquid, flows and settles as the new one.",
    },
    TransitionSpec {
        key: "lens",
        family: Family::Warp,
        desc: "A magnifying lens grows from a point and shows the new picture.",
    },
    TransitionSpec {
        key: "flag",
        family: Family::Warp,
        desc: "The old picture waves like a flag and blows away.",
    },
    TransitionSpec {
        key: "tunnel",
        family: Family::Warp,
        desc: "The old picture falls down a tunnel and the new one rushes out.",
    },
    TransitionSpec {
        key: "accordion",
        family: Family::Warp,
        desc: "The old picture folds up like an accordion.",
    },
    TransitionSpec {
        key: "glass",
        family: Family::Break,
        desc: "Cracks run out from a point and the glass falls apart.",
    },
    TransitionSpec {
        key: "paper-tear",
        family: Family::Break,
        desc: "The old picture tears across like paper.",
    },
    TransitionSpec {
        key: "crt",
        family: Family::Break,
        desc: "An old TV switches off into a dot and back on with the new picture.",
    },
    TransitionSpec {
        key: "datamosh",
        family: Family::Break,
        desc: "Blocks of the new picture smear over the old like a broken video.",
    },
    TransitionSpec {
        key: "luma-fade",
        family: Family::Fade,
        desc: "Shadows first: the dark parts of the new picture appear before the bright ones.",
    },
    TransitionSpec {
        key: "dream-blur",
        family: Family::Fade,
        desc: "A soft dreamy blur, then the new picture comes into focus.",
    },
    TransitionSpec {
        key: "film-dissolve",
        family: Family::Fade,
        desc: "A grainy old-film dissolve with flicker and dust.",
    },
    TransitionSpec {
        key: "clock-wipe",
        family: Family::Wipe,
        desc: "A clock hand sweeps round a point.",
    },
    TransitionSpec {
        key: "barn-doors",
        family: Family::Wipe,
        desc: "The old picture splits and swings open like two doors.",
    },
    TransitionSpec {
        key: "curtain",
        family: Family::Wipe,
        desc: "The old picture gathers into folds like stage curtains and draws open.",
    },
    TransitionSpec {
        key: "wave-wipe",
        family: Family::Wipe,
        desc: "A wavy edge rolls across with a line of foam.",
    },
    // Custom build (M3): resolved before a transition starts.
    TransitionSpec {
        key: "playlist",
        family: Family::Playlist,
        desc: "Plays the transitions in your list in turns.",
    },
    TransitionSpec {
        key: "combo",
        family: Family::Combo,
        desc: "Your combination of two transitions (set it up below).",
    },
    TransitionSpec {
        key: "random-mix",
        family: Family::Combo,
        desc: "Two random transitions combined in a random way, every time.",
    },
];

pub fn family_of(key: &str) -> Family {
    if let Some(spec) = TRANSITIONS.iter().find(|spec| spec.key == key) {
        return spec.family;
    }
    if key.starts_with("sand-") {
        return Family::Sand;
    }
    if key.starts_with(COMBO_PREFIX) {
        return Family::Combo;
    }
    Family::Break
}

pub fn keys_in_family(family: Family) -> impl Iterator<Item = &'static str> {
    TRANSITIONS.iter().filter(move |spec| spec.family == family).map(|spec| spec.key)
}

/// Custom build (M3): the catalogue entry of a key.
pub fn spec(key: &str) -> Option<&'static TransitionSpec> {
    TRANSITIONS.iter().find(|spec| spec.key == key)
}

/// Custom build (M3): whether a key is a real renderer transition (not
/// "random", "playlist", ...). Combination names count.
pub fn is_shader(key: &str) -> bool {
    if key.starts_with(COMBO_PREFIX) {
        return true;
    }
    !PSEUDO_KEYS.contains(&key) && TRANSITIONS.iter().any(|spec| spec.key == key)
}

/// Custom build (M3): the renderer name of a combination. `mode` is a
/// `COMBO_MODES` key; `split` (a `COMBO_SPLITS` key) is used by "split".
pub fn combo_name(mode: &str, split: &str, first: &str, second: &str) -> String {
    let mode = if COMBO_MODES.iter().any(|(key, _)| *key == mode) { mode } else { "then" };
    let mode = if mode == "split" {
        let split = if COMBO_SPLITS.iter().any(|(key, _)| *key == split) { split } else { "lr" };
        format!("split-{split}")
    } else {
        mode.to_string()
    };
    let part = |key: &str| {
        if is_shader(key) && !key.starts_with(COMBO_PREFIX) || key == "random" {
            key.to_string()
        } else {
            String::from("random")
        }
    };
    format!("{COMBO_PREFIX}{mode}:{}:{}", part(first), part(second))
}

/// Custom build (M3): a readable name for a key or a combination name.
pub fn label_of(key: &str) -> String {
    if let Some(rest) = key.strip_prefix(COMBO_PREFIX) {
        let mut fields = rest.splitn(3, ':');
        let mode = fields.next().unwrap_or("");
        let first = pretty(fields.next().unwrap_or(""));
        let second = pretty(fields.next().unwrap_or(""));
        let how = if mode.starts_with("split") {
            "split"
        } else {
            COMBO_MODES.iter().find(|(key, _)| *key == mode).map_or("then", |(key, _)| key)
        };
        if how == "mask" {
            return format!("{second} in {first} shape");
        }
        let joint = match how {
            "then" => "then",
            "blend" => "blends into",
            "split" => "|",
            "sweep" => "sweeps to",
            "light" => "+",
            _ => "×",
        };
        return format!("{first} {joint} {second}");
    }
    pretty(key)
}

/// Custom build (M3): "polka-dots-curtain" → "Polka Dots Curtain".
pub fn pretty(key: &str) -> String {
    let mut out = String::with_capacity(key.len());
    for (index, word) in key.split('-').enumerate() {
        if index > 0 {
            out.push(' ');
        }
        if word == "crt" {
            out.push_str("CRT");
            continue;
        }
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
        }
    }
    out
}

#[cfg(test)]
#[path = "transitions_tests.rs"]
mod tests;
