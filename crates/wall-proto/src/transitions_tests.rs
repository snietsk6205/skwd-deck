use super::*;

#[test]
fn catalog_matches_paper_shaders() {
    let mut ours: Vec<&str> = super::TRANSITIONS
        .iter()
        .map(|spec| spec.key)
        .filter(|key| !PSEUDO_KEYS.contains(key))
        .collect();
    ours.sort_unstable();
    let mut shaders: Vec<&str> = paper_shaders::EFFECTS
        .iter()
        .map(|(key, _)| *key)
        .chain(paper_shaders::SAND_STYLES.iter().copied())
        .collect();
    shaders.sort_unstable();
    assert_eq!(ours, shaders);
}

#[test]
fn every_transition_has_a_sentence() {
    for spec in TRANSITIONS {
        assert!(spec.desc.ends_with('.'), "{}", spec.key);
        assert!(spec.desc.len() < 100, "{}", spec.key);
    }
}

#[test]
fn pseudo_keys_are_listed_but_not_shaders() {
    for key in PSEUDO_KEYS {
        assert!(spec(key).is_some(), "{key}");
        assert!(!is_shader(key), "{key}");
    }
    assert!(is_shader("burn") && is_shader("sand-galaxy"));
    assert!(is_shader("combo:then:burn:ripple"));
    assert!(!is_shader("no-such-thing"));
}

#[test]
fn combo_names_match_the_renderer() {
    let mut roll = || 0u32;
    for (mode, _) in COMBO_MODES {
        for (split, _) in COMBO_SPLITS {
            let name = combo_name(mode, split, "burn", "ripple");
            let plan = paper_shaders::plan::plan(&name, &mut roll);
            assert!(matches!(plan, paper_shaders::plan::Plan::Combo(_)), "{name}");
        }
    }
    assert_eq!(combo_name("split", "circle", "iris", "glitch"), "combo:split-circle:iris:glitch");
    assert_eq!(combo_name("nope", "", "playlist", "burn"), "combo:then:random:burn");
    assert_eq!(family_of("combo:then:burn:ripple"), Family::Combo);
}

#[test]
fn labels_read_as_words() {
    assert_eq!(label_of("polka-dots-curtain"), "Polka Dots Curtain");
    assert_eq!(label_of("crt"), "CRT");
    assert_eq!(label_of("combo:then:burn:page-curl"), "Burn then Page Curl");
    assert_eq!(label_of("combo:split-circle:iris:glitch"), "Iris | Glitch");
    assert_eq!(label_of("combo:mask:honeycomb:burn"), "Burn in Honeycomb shape");
}
