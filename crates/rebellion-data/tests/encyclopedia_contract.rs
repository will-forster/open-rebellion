use std::fs;
use std::path::{Path, PathBuf};

use rebellion_data::encyclopedia::{
    parse_catalog, parse_manifest, resolve_admitted_topics, resolve_localized,
    resolve_localized_label, resolve_topic, AdmissionFact, AdmissionSnapshot, AdmittedBinding,
    BaseImageIdField, BindingKey, LocalizedContent, SystemSourceAncestry, TopicId, ViewerFaction,
    CATALOG_JSON_BYTES_LIMIT, CATALOG_JSON_DEPTH_LIMIT, MANIFEST_JSON_BYTES_LIMIT,
    MANIFEST_JSON_DEPTH_LIMIT,
};
use serde::Deserialize;

fn corpus_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tests/fixtures/encyclopedia")
}

fn fixture(relative: &str) -> Vec<u8> {
    fs::read(corpus_root().join("fixtures").join(relative)).unwrap()
}

fn valid_catalog() -> rebellion_data::encyclopedia::EncyclopediaCatalog {
    parse_catalog(&fixture("bundles/valid/catalog.json")).unwrap()
}

fn admission_snapshot(keys: impl IntoIterator<Item = BindingKey>) -> AdmissionSnapshot {
    AdmissionSnapshot {
        world_epoch: 41,
        viewer: ViewerFaction::Alliance,
        admitted: keys
            .into_iter()
            .map(|key| AdmittedBinding {
                key,
                fact: AdmissionFact::DefinitionPresent,
            })
            .collect(),
    }
}

fn resolved_topic_ids<'a>(
    catalog: &'a rebellion_data::encyclopedia::EncyclopediaCatalog,
    membership: &'a [TopicId],
    admission: &'a AdmissionSnapshot,
    language: &str,
) -> Vec<&'a str> {
    resolve_admitted_topics(catalog, membership, Some(admission), language)
        .unwrap()
        .rows
        .into_iter()
        .map(|topic| topic.topic_id.0.as_str())
        .collect()
}

fn catalog_error(bytes: &[u8]) -> String {
    parse_catalog(bytes).unwrap_err().code().to_owned()
}

fn manifest_error(bytes: &[u8]) -> String {
    parse_manifest(bytes).unwrap_err().code().to_owned()
}

#[derive(Deserialize)]
struct FixtureInventory {
    cases: Vec<FixtureCase>,
}

#[derive(Deserialize)]
struct FixtureCase {
    id: String,
    validation_layer: String,
    fixture: Option<String>,
    consuming_validator: String,
    expect: FixtureExpectation,
}

#[derive(Deserialize)]
struct FixtureExpectation {
    accepted: bool,
    code: String,
}

fn fixture_inventory() -> FixtureInventory {
    serde_json::from_slice(&fs::read(corpus_root().join("cases.json")).unwrap()).unwrap()
}

#[test]
fn valid_catalog_and_manifest_preserve_typed_wire_fields() {
    let catalog = parse_catalog(&fixture("bundles/valid/catalog.json")).unwrap();
    assert_eq!(catalog.schema_version, 1);
    assert_eq!(catalog.default_language, "1033");
    assert_eq!(catalog.topic_sort.algorithm, "stable_display_title_v1");
    assert_eq!(catalog.index.command, "0x6f");
    assert_eq!(catalog.categories.len(), 6);
    assert_eq!(catalog.topics.len(), 7);
    assert_eq!(catalog.images.len(), 3);
    assert_eq!(catalog.bindings.len(), 7);

    let selector = &catalog.topics["original:60004"].localized["1033"];
    assert!(selector.image_selector.is_some());
    assert_eq!(selector.image_id, BaseImageIdField::Absent);
    let explicit_no_art = &catalog.topics["original:60006"].localized["1033"];
    assert_eq!(explicit_no_art.image_id, BaseImageIdField::Null);

    let manifest = parse_manifest(&fixture("bundles/valid/manifest.json")).unwrap();
    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.source_profile, "e37-synthetic-v1");
    assert!(manifest.files.contains_key("catalog.json"));
    assert!(!manifest.binding_sources.is_empty());
    assert!(!manifest.source_records.is_empty());
}

#[test]
fn shared_e37_schema_cases_return_their_stable_diagnostics() {
    let inventory = fixture_inventory();
    let mut exercised = 0;
    for case in inventory.cases.iter().filter(|case| {
        case.consuming_validator.contains("E11 Rust")
            && case.validation_layer == "schema_structure"
            && case.fixture.is_some()
    }) {
        let bytes = fixture(case.fixture.as_deref().unwrap());
        let result = if case.id.starts_with("manifest-") {
            parse_manifest(&bytes).map(|_| ())
        } else {
            parse_catalog(&bytes).map(|_| ())
        };
        match (&case.expect.accepted, result) {
            (true, Ok(())) => {}
            (false, Err(error)) => assert_eq!(error.code(), case.expect.code, "{}", case.id),
            (expected, actual) => {
                panic!("{} expected accepted={expected}, got {actual:?}", case.id)
            }
        }
        exercised += 1;
    }
    assert_eq!(
        exercised, 27,
        "the approved E37 structural inventory changed"
    );
}

#[test]
fn duplicate_keys_are_rejected_before_nested_maps_can_overwrite_them() {
    let inventory = fixture_inventory();
    let cases: Vec<_> = inventory
        .cases
        .iter()
        .filter(|case| {
            case.consuming_validator.contains("E11 Rust")
                && case.validation_layer == "raw_parsing"
                && !case.expect.accepted
                && case.expect.code == "duplicate_key"
        })
        .collect();
    assert_eq!(cases.len(), 7);
    for case in cases {
        let bytes = fixture(case.fixture.as_deref().unwrap());
        let code = if case.id.starts_with("raw-manifest-") {
            manifest_error(&bytes)
        } else {
            catalog_error(&bytes)
        };
        assert_eq!(code, "duplicate_key", "{}", case.id);
    }
}

#[test]
fn raw_scanner_accepts_nested_values_and_repeated_keys_in_distinct_scopes() {
    for relative in [
        "raw/valid-nested-values.json",
        "raw/valid-repeated-keys-distinct-scopes.json",
    ] {
        let error = parse_catalog(&fixture(relative)).unwrap_err();
        assert_ne!(error.code(), "duplicate_key", "{relative}");
        assert_ne!(error.code(), "invalid_json", "{relative}");
        assert_ne!(error.code(), "invalid_utf8", "{relative}");
    }
}

#[test]
fn invalid_utf8_is_rejected_before_typed_deserialization() {
    let error = parse_catalog(&fixture("raw/catalog-invalid-utf8.json")).unwrap_err();
    assert_eq!(error.code(), "invalid_utf8");
    assert_eq!(error.source(), "catalog");
}

#[test]
fn required_nullable_and_exact_case_fields_preserve_presence() {
    let catalog = fixture("bundles/valid/catalog.json");
    let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();

    value["topics"]["original:60001"]["localized"]["1033"]
        .as_object_mut()
        .unwrap()
        .remove("body");
    let error = parse_catalog(&serde_json::to_vec(&value).unwrap()).unwrap_err();
    assert_eq!(error.code(), "missing_field");
    assert!(error.path().ends_with(".body"));
    assert_eq!(error.topic_id().unwrap().0, "original:60001");

    let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
    value["topics"]["original:60001"]["localized"]["1033"]["body"] = serde_json::Value::Null;
    assert_eq!(
        catalog_error(&serde_json::to_vec(&value).unwrap()),
        "invalid_type"
    );

    let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
    let localized = value["topics"]["original:60001"]["localized"]["1033"]
        .as_object_mut()
        .unwrap();
    let body = localized.remove("body").unwrap();
    localized.insert("Body".to_owned(), body);
    assert_eq!(
        catalog_error(&serde_json::to_vec(&value).unwrap()),
        "unknown_field"
    );

    let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
    value["topics"]["original:60004"]["localized"]["1033"]["image_id"] = serde_json::Value::Null;
    assert_eq!(
        catalog_error(&serde_json::to_vec(&value).unwrap()),
        "ambiguous_image_selector"
    );

    let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
    value["bindings"][0]
        .as_object_mut()
        .unwrap()
        .remove("dat_id");
    assert_eq!(
        catalog_error(&serde_json::to_vec(&value).unwrap()),
        "missing_field"
    );
}

#[test]
fn manifest_nested_fields_reject_missing_null_and_wrong_types() {
    let manifest = fixture("bundles/valid/manifest.json");
    let mut value: serde_json::Value = serde_json::from_slice(&manifest).unwrap();
    let first = value["source_records"]
        .as_object_mut()
        .unwrap()
        .values_mut()
        .next()
        .unwrap();
    first.as_object_mut().unwrap().remove("raw_length");
    assert_eq!(
        manifest_error(&serde_json::to_vec(&value).unwrap()),
        "missing_field"
    );

    let mut value: serde_json::Value = serde_json::from_slice(&manifest).unwrap();
    let first = value["source_records"]
        .as_object_mut()
        .unwrap()
        .values_mut()
        .next()
        .unwrap();
    first["language_id"] = serde_json::Value::Null;
    assert_eq!(
        manifest_error(&serde_json::to_vec(&value).unwrap()),
        "invalid_type"
    );

    let mut value: serde_json::Value = serde_json::from_slice(&manifest).unwrap();
    let first = value["source_records"]
        .as_object_mut()
        .unwrap()
        .values_mut()
        .next()
        .unwrap();
    first["resource_id"]["value"] = serde_json::Value::Null;
    assert_eq!(
        manifest_error(&serde_json::to_vec(&value).unwrap()),
        "invalid_type"
    );
}

#[test]
fn runtime_only_and_alias_fields_are_not_wire_fields() {
    let catalog = fixture("bundles/valid/catalog.json");
    for field in ["aliases", "effective_images", "retained_bytes", "mod_name"] {
        let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
        value[field] = serde_json::json!({});
        let error = parse_catalog(&serde_json::to_vec(&value).unwrap()).unwrap_err();
        assert_eq!(error.code(), "unknown_field", "{field}");
        assert_eq!(error.source(), "catalog");
        assert_eq!(error.path(), format!("$.{field}"));
    }
}

#[test]
fn schema_valid_png_descriptors_remain_structural_while_base_policy_is_deferred() {
    let mut value: serde_json::Value =
        serde_json::from_slice(&fixture("bundles/valid/catalog.json")).unwrap();
    value["images"]["edata:1"]["format"] = serde_json::Value::String("png".to_owned());
    parse_catalog(&serde_json::to_vec(&value).unwrap()).unwrap();
}

fn runtime_asset_path_with_total_len(total_len: usize) -> String {
    let final_segment_len = total_len - 171;
    format!(
        "assets/{}/{}/{}/{}/{}",
        "a".repeat(40),
        "b".repeat(40),
        "c".repeat(40),
        "d".repeat(40),
        "e".repeat(final_segment_len),
    )
}

#[test]
fn runtime_asset_paths_accept_256_bytes_and_reject_257_bytes() {
    let exact = runtime_asset_path_with_total_len(256);
    let above = runtime_asset_path_with_total_len(257);
    assert_eq!(exact.len(), 256);
    assert_eq!(above.len(), 257);

    let catalog = fixture("bundles/valid/catalog.json");
    for (path, accepted) in [(&exact, true), (&above, false)] {
        let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
        value["images"]["edata:1"]["path"] = serde_json::Value::String(path.clone());
        let result = parse_catalog(&serde_json::to_vec(&value).unwrap());
        if accepted {
            result.unwrap();
        } else {
            assert_eq!(result.unwrap_err().code(), "unsafe_asset_path");
        }
    }

    let manifest = fixture("bundles/valid/manifest.json");
    for (path, accepted) in [(&exact, true), (&above, false)] {
        let mut value: serde_json::Value = serde_json::from_slice(&manifest).unwrap();
        value["files"]
            .as_object_mut()
            .unwrap()
            .insert(path.clone(), serde_json::Value::String("0".repeat(64)));
        let result = parse_manifest(&serde_json::to_vec(&value).unwrap());
        if accepted {
            result.unwrap();
        } else {
            assert_eq!(result.unwrap_err().code(), "unsafe_asset_path");
        }
    }
}

#[test]
fn runtime_asset_path_segments_use_distinct_directory_and_final_limits() {
    let catalog = fixture("bundles/valid/catalog.json");
    for (path, accepted) in [
        (format!("assets/{}/x", "a".repeat(127)), true),
        (format!("assets/{}/x", "a".repeat(128)), false),
        (format!("assets/{}", "z".repeat(128)), true),
        (format!("assets/{}", "z".repeat(129)), false),
    ] {
        let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
        value["images"]["edata:1"]["path"] = serde_json::Value::String(path);
        let result = parse_catalog(&serde_json::to_vec(&value).unwrap());
        if accepted {
            result.unwrap();
        } else {
            assert_eq!(result.unwrap_err().code(), "unsafe_asset_path");
        }
    }
}

#[test]
fn localized_text_limits_count_utf8_bytes_at_exact_and_above_boundaries() {
    let catalog = fixture("bundles/valid/catalog.json");

    for (field, exact, code) in [
        ("title", "t".repeat(65_536), "resource_limit:title_bytes"),
        ("body", "b".repeat(1_048_576), "resource_limit:body_bytes"),
    ] {
        let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
        value["topics"]["original:60001"]["localized"]["1033"][field] =
            serde_json::Value::String(exact.clone());
        parse_catalog(&serde_json::to_vec(&value).unwrap()).unwrap();
        value["topics"]["original:60001"]["localized"]["1033"][field] =
            serde_json::Value::String(format!("{exact}x"));
        assert_eq!(catalog_error(&serde_json::to_vec(&value).unwrap()), code);
    }

    let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
    value["topics"]["original:60001"]["localized"]["1033"]["body"] =
        serde_json::Value::String("é".repeat(524_289));
    assert_eq!(
        catalog_error(&serde_json::to_vec(&value).unwrap()),
        "resource_limit:body_bytes"
    );

    let mut value: serde_json::Value = serde_json::from_slice(&catalog).unwrap();
    value["index"]["labels"]["1033"] = serde_json::Value::String("l".repeat(65_536));
    parse_catalog(&serde_json::to_vec(&value).unwrap()).unwrap();
    value["index"]["labels"]["1033"] = serde_json::Value::String("l".repeat(65_537));
    assert_eq!(
        catalog_error(&serde_json::to_vec(&value).unwrap()),
        "resource_limit:label_bytes"
    );
}

#[test]
fn topic_count_accepts_ten_thousand_and_rejects_one_more() {
    let mut value: serde_json::Value =
        serde_json::from_slice(&fixture("bundles/valid/catalog.json")).unwrap();
    let template = value["topics"]["original:60001"].clone();
    let topics = value["topics"].as_object_mut().unwrap();
    topics.clear();
    for id in 1..=10_000 {
        topics.insert(format!("original:{id}"), template.clone());
    }
    parse_catalog(&serde_json::to_vec(&value).unwrap()).unwrap();
    value["topics"]
        .as_object_mut()
        .unwrap()
        .insert("original:10001".to_owned(), template);
    assert_eq!(
        catalog_error(&serde_json::to_vec(&value).unwrap()),
        "resource_limit:topics"
    );
}

fn nested_array(depth: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(depth * 2 + 1);
    bytes.extend(std::iter::repeat_n(b'[', depth));
    bytes.push(b'0');
    bytes.extend(std::iter::repeat_n(b']', depth));
    bytes
}

fn nested_object(depth: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(depth * 6 + 1);
    for _ in 0..depth {
        bytes.extend_from_slice(br#"{"a":"#);
    }
    bytes.push(b'0');
    bytes.extend(std::iter::repeat_n(b'}', depth));
    bytes
}

#[test]
fn json_depth_limits_accept_the_boundary_before_structural_rejection() {
    let catalog_exact = parse_catalog(&nested_array(CATALOG_JSON_DEPTH_LIMIT)).unwrap_err();
    assert_ne!(catalog_exact.code(), "resource_limit:json_depth");
    assert_eq!(
        catalog_error(&nested_array(CATALOG_JSON_DEPTH_LIMIT + 1)),
        "resource_limit:json_depth"
    );

    let manifest_exact = parse_manifest(&nested_array(MANIFEST_JSON_DEPTH_LIMIT)).unwrap_err();
    assert_ne!(manifest_exact.code(), "resource_limit:json_depth");
    assert_eq!(
        manifest_error(&nested_array(MANIFEST_JSON_DEPTH_LIMIT + 1)),
        "resource_limit:json_depth"
    );

    let object_exact = parse_catalog(&nested_object(CATALOG_JSON_DEPTH_LIMIT)).unwrap_err();
    assert_ne!(object_exact.code(), "resource_limit:json_depth");
    assert_eq!(
        catalog_error(&nested_object(CATALOG_JSON_DEPTH_LIMIT + 1)),
        "resource_limit:json_depth"
    );
}

#[test]
fn json_byte_limits_accept_the_boundary_before_syntax_rejection() {
    let catalog_exact = vec![b' '; CATALOG_JSON_BYTES_LIMIT];
    assert_ne!(
        parse_catalog(&catalog_exact).unwrap_err().code(),
        "resource_limit:json_bytes"
    );
    let catalog_above = vec![b' '; CATALOG_JSON_BYTES_LIMIT + 1];
    assert_eq!(catalog_error(&catalog_above), "resource_limit:json_bytes");

    let manifest_exact = vec![b' '; MANIFEST_JSON_BYTES_LIMIT];
    assert_ne!(
        parse_manifest(&manifest_exact).unwrap_err().code(),
        "resource_limit:json_bytes"
    );
    let manifest_above = vec![b' '; MANIFEST_JSON_BYTES_LIMIT + 1];
    assert_eq!(manifest_error(&manifest_above), "resource_limit:json_bytes");
}

#[test]
fn topic_resolution_uses_the_complete_original_binding_identity() {
    let mut catalog = valid_catalog();

    let system = BindingKey {
        family: "system_locations".to_owned(),
        dat_id: 7,
        variant: "default".to_owned(),
    };
    let capital_ship = BindingKey {
        family: "capital_ship_classes".to_owned(),
        dat_id: 7,
        variant: "default".to_owned(),
    };
    let viewer_faction = BindingKey {
        family: "missions".to_owned(),
        dat_id: 21,
        variant: "viewer_faction".to_owned(),
    };

    assert_eq!(
        resolve_topic(&catalog, &system).unwrap().0,
        "original:60001"
    );
    assert_eq!(
        resolve_topic(&catalog, &capital_ship).unwrap().0,
        "original:60002"
    );
    assert_eq!(
        resolve_topic(&catalog, &viewer_faction).unwrap().0,
        "original:60004"
    );

    let unknown_variant = BindingKey {
        variant: "alternate".to_owned(),
        ..viewer_faction
    };
    assert!(resolve_topic(&catalog, &unknown_variant).is_none());
    assert!(resolve_topic(
        &catalog,
        &BindingKey {
            family: "unknown_family".to_owned(),
            dat_id: 7,
            variant: "default".to_owned(),
        }
    )
    .is_none());

    let mut duplicate = catalog.bindings[0].clone();
    duplicate.topic_id = TopicId("original:60002".to_owned());
    catalog.bindings.push(duplicate);
    assert!(resolve_topic(&catalog, &system).is_none());
}

#[test]
fn localized_resolution_selects_one_requested_or_default_record_without_mixing() {
    let mut catalog = valid_catalog();
    let topic_id = TopicId("original:60001".to_owned());

    let requested = resolve_localized(&catalog, &topic_id, "1036").unwrap();
    assert_eq!(requested.title, "Système ambre");
    assert_eq!(
        requested.body,
        "Description synthétique rédigée pour ce test."
    );
    assert_eq!(
        requested.image_id,
        BaseImageIdField::Value(rebellion_data::encyclopedia::BaseImageId(
            "edata:1".to_owned()
        ))
    );

    let fallback = resolve_localized(&catalog, &topic_id, "1041").unwrap();
    assert_eq!(fallback.title, "Amber system");
    assert_eq!(fallback.body, "Contributor-written system description.");

    catalog.topics.get_mut(&topic_id).unwrap().localized.insert(
        "1041".to_owned(),
        LocalizedContent {
            title: "Requested title".to_owned(),
            body: "Requested body".to_owned(),
            image_id: BaseImageIdField::Null,
            image_selector: None,
        },
    );
    let requested = resolve_localized(&catalog, &topic_id, "1041").unwrap();
    assert_eq!(requested.title, "Requested title");
    assert_eq!(requested.body, "Requested body");
    assert_eq!(requested.image_id, BaseImageIdField::Null);

    let default = catalog
        .topics
        .get_mut(&topic_id)
        .unwrap()
        .localized
        .get_mut("1033")
        .unwrap();
    default.title = "Changed default title".to_owned();
    default.body = "Changed default body".to_owned();
    default.image_id = BaseImageIdField::Value(rebellion_data::encyclopedia::BaseImageId(
        "edata:2".to_owned(),
    ));
    let requested = resolve_localized(&catalog, &topic_id, "1041").unwrap();
    assert_eq!(requested.title, "Requested title");
    assert_eq!(requested.body, "Requested body");
    assert_eq!(requested.image_id, BaseImageIdField::Null);

    catalog
        .topics
        .get_mut(&topic_id)
        .unwrap()
        .localized
        .remove("1041");
    let restored_fallback = resolve_localized(&catalog, &topic_id, "1041").unwrap();
    assert_eq!(restored_fallback.title, "Changed default title");
    assert_eq!(restored_fallback.body, "Changed default body");

    catalog.topics.get_mut(&topic_id).unwrap().localized.clear();
    let error = resolve_localized(&catalog, &topic_id, "1041").unwrap_err();
    assert_eq!(error.code(), "missing_localized_record");
    assert_eq!(error.topic_id(), Some(&topic_id));
}

#[test]
fn localized_labels_preserve_present_empty_values_and_never_invent_internal_keys() {
    let mut catalog = valid_catalog();
    let labels = &mut catalog.categories[0].labels;
    assert_eq!(
        resolve_localized_label(labels, &catalog.default_language, "1041").unwrap(),
        "Synthetic systems"
    );
    labels.insert("1041".to_owned(), String::new());

    assert_eq!(
        resolve_localized_label(labels, &catalog.default_language, "1041").unwrap(),
        ""
    );

    labels.clear();
    let error = resolve_localized_label(labels, &catalog.default_language, "1041").unwrap_err();
    assert_eq!(error.code(), "missing_localized_label");
    assert!(!error.to_string().contains(&catalog.categories[0].id));
}

#[test]
fn admitted_resolution_requires_supplied_facts_and_preserves_them_without_world_inference() {
    let catalog = valid_catalog();
    let missing = resolve_admitted_topics(
        &catalog,
        &catalog.index.topic_ids,
        None,
        &catalog.default_language,
    )
    .unwrap_err();
    assert_eq!(missing.code(), "missing_admission_facts");

    let unknown_key = BindingKey {
        family: "system_locations".to_owned(),
        dat_id: 999_999,
        variant: "default".to_owned(),
    };
    let unavailable = admission_snapshot([unknown_key]);
    let error = resolve_admitted_topics(
        &catalog,
        &catalog.index.topic_ids,
        Some(&unavailable),
        &catalog.default_language,
    )
    .unwrap_err();
    assert_eq!(error.code(), "unavailable_binding");

    let duplicate_key = catalog.bindings[0].key();
    let ambiguous = admission_snapshot([duplicate_key.clone(), duplicate_key]);
    let error = resolve_admitted_topics(
        &catalog,
        &catalog.index.topic_ids[..1],
        Some(&ambiguous),
        &catalog.default_language,
    )
    .unwrap_err();
    assert_eq!(error.code(), "ambiguous_admission");

    let binding = catalog.bindings[0].key();
    let admission = AdmissionSnapshot {
        world_epoch: 77,
        viewer: ViewerFaction::Empire,
        admitted: vec![AdmittedBinding {
            key: binding.clone(),
            fact: AdmissionFact::InstantiatedSystem {
                selected_view: ViewerFaction::Empire,
                ancestry: SystemSourceAncestry::NoTypeF2,
            },
        }],
    };
    let view = resolve_admitted_topics(
        &catalog,
        &catalog.index.topic_ids,
        Some(&admission),
        &catalog.default_language,
    )
    .unwrap();
    assert!(view.diagnostics.is_empty());
    assert_eq!(view.rows.len(), 1);
    assert_eq!(view.rows[0].binding, &binding);
    assert_eq!(view.rows[0].admission_fact, &admission.admitted[0].fact);
    assert_eq!(view.rows[0].topic_id.0, "original:60001");
    assert_eq!(view.rows[0].image_id.unwrap().0, "edata:1");
}

#[test]
fn unexpected_topic_lookup_failure_keeps_the_whole_view_unavailable() {
    let mut catalog = valid_catalog();
    let missing_topic = TopicId("original:999999".to_owned());
    catalog.bindings[0].topic_id = missing_topic.clone();
    let admission = admission_snapshot([catalog.bindings[0].key()]);

    let error = resolve_admitted_topics(
        &catalog,
        std::slice::from_ref(&missing_topic),
        Some(&admission),
        "1041",
    )
    .unwrap_err();

    assert_eq!(error.code(), "unknown_topic");
    assert_eq!(error.topic_id(), Some(&missing_topic));
}

#[test]
fn one_missing_localized_record_does_not_discard_other_resolved_topics() {
    let mut catalog = valid_catalog();
    let disabled_topic = TopicId("original:60001".to_owned());
    catalog
        .topics
        .get_mut(&disabled_topic)
        .unwrap()
        .localized
        .remove("1033");
    let membership = catalog.index.topic_ids[..2].to_vec();
    let admission = admission_snapshot(catalog.bindings[..2].iter().map(|binding| binding.key()));

    let view = resolve_admitted_topics(&catalog, &membership, Some(&admission), "1041")
        .expect("a topic-local language miss must not make the whole snapshot unavailable");

    assert_eq!(view.rows.len(), 1);
    assert_eq!(view.rows[0].topic_id.0, "original:60002");
    assert_eq!(view.diagnostics.len(), 1);
    assert_eq!(view.diagnostics[0].topic_id, &disabled_topic);
    assert_eq!(view.diagnostics[0].error.code(), "missing_localized_record");
    assert_eq!(view.diagnostics[0].error.topic_id(), Some(&disabled_topic));
}

#[test]
fn all_missing_localized_records_return_an_available_empty_view_with_diagnostics() {
    let mut catalog = valid_catalog();
    let first = TopicId("original:60001".to_owned());
    let second = TopicId("original:60002".to_owned());
    catalog
        .topics
        .get_mut(&first)
        .unwrap()
        .localized
        .remove("1033");
    let second_french = catalog
        .topics
        .get_mut(&second)
        .unwrap()
        .localized
        .remove("1033")
        .unwrap();
    catalog
        .topics
        .get_mut(&second)
        .unwrap()
        .localized
        .insert("1036".to_owned(), second_french);
    let membership = catalog.index.topic_ids[..2].to_vec();
    let admission = admission_snapshot(catalog.bindings[..2].iter().map(|binding| binding.key()));

    let view = resolve_admitted_topics(&catalog, &membership, Some(&admission), "1041").unwrap();

    assert!(view.rows.is_empty());
    assert_eq!(
        view.diagnostics
            .iter()
            .map(|diagnostic| diagnostic.topic_id.0.as_str())
            .collect::<Vec<_>>(),
        vec!["original:60001", "original:60002"]
    );
    assert!(view
        .diagnostics
        .iter()
        .all(|diagnostic| diagnostic.error.code() == "missing_localized_record"));
}

#[test]
fn surviving_rows_keep_effective_title_order_when_another_topic_is_disabled() {
    let mut catalog = valid_catalog();
    let first = TopicId("original:60001".to_owned());
    let disabled = TopicId("original:60002".to_owned());
    let third = TopicId("original:60003".to_owned());
    catalog
        .topics
        .get_mut(&first)
        .unwrap()
        .localized
        .get_mut("1033")
        .unwrap()
        .title = "Zulu surviving".to_owned();
    let disabled_french = catalog
        .topics
        .get_mut(&disabled)
        .unwrap()
        .localized
        .remove("1033")
        .unwrap();
    catalog
        .topics
        .get_mut(&disabled)
        .unwrap()
        .localized
        .insert("1036".to_owned(), disabled_french);
    catalog
        .topics
        .get_mut(&third)
        .unwrap()
        .localized
        .get_mut("1033")
        .unwrap()
        .title = "Alpha surviving".to_owned();
    let membership = catalog.index.topic_ids[..3].to_vec();
    let admission = admission_snapshot(
        catalog.bindings[..3]
            .iter()
            .rev()
            .map(|binding| binding.key()),
    );

    let view = resolve_admitted_topics(&catalog, &membership, Some(&admission), "1041").unwrap();

    assert_eq!(
        view.rows
            .iter()
            .map(|row| row.topic_id.0.as_str())
            .collect::<Vec<_>>(),
        vec!["original:60003", "original:60001"]
    );
    assert_eq!(view.diagnostics.len(), 1);
    assert_eq!(view.diagnostics[0].topic_id, &disabled);
}

#[test]
fn no_art_topics_remain_artless_after_whole_record_resolution() {
    let catalog = valid_catalog();
    let keys = [catalog.bindings[4].key(), catalog.bindings[5].key()];
    let admission = admission_snapshot(keys);
    let membership = [
        TopicId("original:60005".to_owned()),
        TopicId("original:60006".to_owned()),
    ];

    let view = resolve_admitted_topics(
        &catalog,
        &membership,
        Some(&admission),
        &catalog.default_language,
    )
    .unwrap();
    assert!(view.diagnostics.is_empty());
    assert_eq!(view.rows.len(), 2);
    assert!(view.rows.iter().all(|topic| topic.image_id.is_none()));
}

#[test]
fn viewer_faction_topics_resolve_only_the_selected_side_art() {
    let catalog = valid_catalog();
    let membership = [TopicId("original:60004".to_owned())];
    let admitted = vec![AdmittedBinding {
        key: catalog.bindings[3].key(),
        fact: AdmissionFact::DefinitionPresent,
    }];

    for (viewer, expected_image) in [
        (ViewerFaction::Alliance, "edata:2"),
        (ViewerFaction::Empire, "edata:3"),
    ] {
        let admission = AdmissionSnapshot {
            world_epoch: 41,
            viewer,
            admitted: admitted.clone(),
        };
        let view =
            resolve_admitted_topics(&catalog, &membership, Some(&admission), "1033").unwrap();
        assert!(view.diagnostics.is_empty());
        assert_eq!(view.rows[0].image_id.unwrap().0, expected_image);
    }
}

#[test]
fn effective_language_records_and_renames_determine_display_order() {
    let mut catalog = valid_catalog();
    let admission = admission_snapshot(catalog.bindings.iter().rev().map(|binding| binding.key()));

    assert_eq!(
        resolved_topic_ids(
            &catalog,
            &catalog.index.topic_ids,
            &admission,
            &catalog.default_language,
        ),
        vec![
            "original:60001",
            "original:60002",
            "original:60003",
            "original:60004",
            "original:60005",
            "original:60006",
            "original:60007",
        ]
    );

    let second = TopicId("original:60002".to_owned());
    catalog.topics.get_mut(&second).unwrap().localized.insert(
        "1036".to_owned(),
        LocalizedContent {
            title: "Zulu traduit".to_owned(),
            body: "Corps traduit synthétique.".to_owned(),
            image_id: BaseImageIdField::Value(rebellion_data::encyclopedia::BaseImageId(
                "edata:2".to_owned(),
            )),
            image_selector: None,
        },
    );
    let first_two = &catalog.index.topic_ids[..2];
    assert_eq!(
        resolved_topic_ids(&catalog, first_two, &admission, "1036"),
        vec!["original:60001", "original:60002"]
    );

    catalog
        .topics
        .get_mut(&second)
        .unwrap()
        .localized
        .remove("1036");
    assert_eq!(
        resolved_topic_ids(&catalog, first_two, &admission, "1036"),
        vec!["original:60002", "original:60001"]
    );

    catalog
        .topics
        .get_mut(&second)
        .unwrap()
        .localized
        .get_mut("1033")
        .unwrap()
        .title = "Aardvark vessel".to_owned();
    assert_eq!(
        resolved_topic_ids(&catalog, first_two, &admission, "1033"),
        vec!["original:60002", "original:60001"]
    );
}

#[test]
fn equal_folded_titles_retain_registry_order_not_admission_order() {
    let mut catalog = valid_catalog();
    let first = TopicId("original:60001".to_owned());
    let second = TopicId("original:60002".to_owned());
    catalog
        .topics
        .get_mut(&first)
        .unwrap()
        .localized
        .get_mut("1033")
        .unwrap()
        .title = "SAME".to_owned();
    catalog
        .topics
        .get_mut(&second)
        .unwrap()
        .localized
        .get_mut("1033")
        .unwrap()
        .title = "same".to_owned();
    let admission = admission_snapshot([catalog.bindings[1].key(), catalog.bindings[0].key()]);

    assert_eq!(
        resolved_topic_ids(&catalog, &catalog.index.topic_ids[..2], &admission, "1033"),
        vec!["original:60001", "original:60002"]
    );
}

#[test]
fn title_sorting_uses_strict_cp1252_then_pinned_scalar_unicode_lowercase() {
    let mut catalog = valid_catalog();
    let titles = [
        ("original:60001", "Éclair"),
        ("original:60002", "zebra"),
        ("original:60003", "Ωmega"),
        ("original:60004", "ΟΣ"),
        ("original:60005", "οσ"),
    ];
    for (topic_id, title) in titles {
        catalog
            .topics
            .get_mut(topic_id)
            .unwrap()
            .localized
            .get_mut("1033")
            .unwrap()
            .title = title.to_owned();
    }
    let admission = admission_snapshot(
        catalog.bindings[..5]
            .iter()
            .rev()
            .map(|binding| binding.key()),
    );

    assert_eq!(
        resolved_topic_ids(&catalog, &catalog.index.topic_ids[..5], &admission, "1033"),
        vec![
            "original:60002",
            "original:60001",
            "original:60004",
            "original:60005",
            "original:60003",
        ]
    );
}
