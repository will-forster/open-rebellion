use std::collections::{BTreeMap, BTreeSet, HashMap};

use rebellion_render::inspect_encyclopedia_bytes;

use crate::encyclopedia_session::{prepare_encyclopedia_session, EncyclopediaAvailability};
use crate::runtime_pack::take_encyclopedia_namespace;

/// Prepares packed encyclopedia content before any global cache installation.
///
/// A wholly absent namespace is compatible with an old pack. Once any member
/// is present, the candidate must be complete, source-paired, and semantically
/// valid or the caller receives an integrity error.
pub fn prepare_packed_encyclopedia(
    game_files: &mut HashMap<String, Vec<u8>>,
) -> Result<EncyclopediaAvailability, String> {
    let Some(bytes) = take_encyclopedia_namespace(game_files)? else {
        return Ok(EncyclopediaAvailability::Unavailable(
            "namespace_absent: runtime pack contains no encyclopedia/ namespace".to_owned(),
        ));
    };

    let dat_hashes = observed_dat_hashes(game_files)?;
    prepare_encyclopedia_session(bytes, &dat_hashes)
        .map(EncyclopediaAvailability::Ready)
        .map_err(|error| format!("invalid_encyclopedia_bundle:{error}"))
}

fn observed_dat_hashes(
    game_files: &HashMap<String, Vec<u8>>,
) -> Result<BTreeMap<String, String>, String> {
    let mut observed = BTreeMap::new();
    let mut folded_names = BTreeSet::new();
    let mut dats: Vec<_> = game_files
        .iter()
        .filter(|(name, _)| name.to_ascii_uppercase().ends_with(".DAT"))
        .collect();
    dats.sort_unstable_by(|left, right| left.0.cmp(right.0));

    for (name, bytes) in dats {
        if name.contains(['/', '\\']) {
            return Err(format!(
                "invalid_encyclopedia_bundle:binding_source_mismatch: DAT cache key {name:?} is not a flat basename"
            ));
        }
        if !folded_names.insert(name.to_ascii_lowercase()) {
            return Err(format!(
                "invalid_encyclopedia_bundle:binding_source_mismatch: DAT basename {name:?} is ambiguous case-insensitively"
            ));
        }
        let inspected = inspect_encyclopedia_bytes(bytes, None).map_err(|detail| {
            format!(
                "invalid_encyclopedia_bundle:binding_source_mismatch: cannot inspect DAT {name:?}: {detail}"
            )
        })?;
        observed.insert(name.clone(), inspected.sha256);
    }
    Ok(observed)
}

#[cfg(test)]
mod tests {
    use super::*;
    const VALID_CATALOG: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/catalog.json");
    const VALID_MANIFEST: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/manifest.json");
    const VALID_IMAGE_1: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.001"
    );
    const VALID_IMAGE_2: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.002"
    );
    const VALID_IMAGE_3: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.003"
    );
    const VALID_DAT: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/sources/SYNTHETIC.DAT"
    );

    fn valid_game_files() -> HashMap<String, Vec<u8>> {
        HashMap::from([
            ("SYNTHETIC.DAT".to_owned(), VALID_DAT.to_vec()),
            ("textstra.json".to_owned(), b"{}".to_vec()),
            (
                "encyclopedia/catalog.json".to_owned(),
                VALID_CATALOG.to_vec(),
            ),
            (
                "encyclopedia/manifest.json".to_owned(),
                VALID_MANIFEST.to_vec(),
            ),
            (
                "encyclopedia/assets/EDATA.001".to_owned(),
                VALID_IMAGE_1.to_vec(),
            ),
            (
                "encyclopedia/assets/EDATA.002".to_owned(),
                VALID_IMAGE_2.to_vec(),
            ),
            (
                "encyclopedia/assets/EDATA.003".to_owned(),
                VALID_IMAGE_3.to_vec(),
            ),
        ])
    }

    #[test]
    fn absent_namespace_is_old_pack_unavailable_and_preserves_game_files() {
        let mut game_files = HashMap::from([
            ("SYNTHETIC.DAT".to_owned(), VALID_DAT.to_vec()),
            ("textstra.json".to_owned(), b"{}".to_vec()),
        ]);
        let original = game_files.clone();

        let availability = prepare_packed_encyclopedia(&mut game_files).unwrap();

        assert!(matches!(
            availability,
            EncyclopediaAvailability::Unavailable(ref diagnostic)
                if diagnostic.contains("namespace_absent")
        ));
        assert_eq!(game_files, original);
    }

    #[test]
    fn valid_namespace_is_removed_and_prepared_from_exact_selected_dat_bytes() {
        let mut game_files = valid_game_files();

        let availability = prepare_packed_encyclopedia(&mut game_files).unwrap();

        let EncyclopediaAvailability::Ready(session) = availability else {
            panic!("valid packed namespace must publish a ready session");
        };
        assert_eq!(session.base_bytes()["catalog.json"].as_ref(), VALID_CATALOG);
        assert_eq!(
            session.base_bytes()["manifest.json"].as_ref(),
            VALID_MANIFEST
        );
        assert!(game_files
            .keys()
            .all(|key| !key.starts_with("encyclopedia/")));
        assert_eq!(game_files["SYNTHETIC.DAT"], VALID_DAT);
        assert_eq!(game_files["textstra.json"], b"{}");
    }

    #[test]
    fn partial_or_corrupt_present_namespace_is_an_integrity_error_not_unavailable() {
        for mutation in ["partial", "corrupt"] {
            let mut game_files = valid_game_files();
            match mutation {
                "partial" => {
                    game_files.remove("encyclopedia/manifest.json");
                }
                "corrupt" => {
                    game_files.insert("encyclopedia/catalog.json".to_owned(), b"not-json".to_vec());
                }
                _ => unreachable!(),
            }

            let error = prepare_packed_encyclopedia(&mut game_files).unwrap_err();

            assert!(error.contains("invalid_encyclopedia_bundle"), "{error}");
            assert!(game_files
                .keys()
                .all(|key| !key.starts_with("encyclopedia/")));
            assert_eq!(game_files["SYNTHETIC.DAT"], VALID_DAT);
        }
    }

    #[test]
    fn wrong_dat_pairing_is_rejected_before_any_candidate_can_be_published() {
        let mut game_files = valid_game_files();
        game_files.insert("SYNTHETIC.DAT".to_owned(), b"wrong installation".to_vec());

        let error = prepare_packed_encyclopedia(&mut game_files).unwrap_err();

        assert!(error.contains("binding_source_mismatch"), "{error}");
    }

    #[test]
    fn case_ambiguous_namespace_members_fail_without_mutating_the_candidate() {
        let mut game_files = valid_game_files();
        game_files.insert(
            "encyclopedia/CATALOG.JSON".to_owned(),
            VALID_CATALOG.to_vec(),
        );
        let original = game_files.clone();

        let error = take_encyclopedia_namespace(&mut game_files).unwrap_err();

        assert!(error.contains("namespace_collision"), "{error}");
        assert_eq!(game_files, original);
    }

    #[test]
    fn malformed_namespace_spelling_cannot_leak_into_the_dat_cache() {
        let mut game_files = valid_game_files();
        game_files.insert(
            "Encyclopedia/catalog.json".to_owned(),
            VALID_CATALOG.to_vec(),
        );
        let original = game_files.clone();

        let error = take_encyclopedia_namespace(&mut game_files).unwrap_err();

        assert!(error.contains("namespace_collision"), "{error}");
        assert_eq!(game_files, original);
    }
}
