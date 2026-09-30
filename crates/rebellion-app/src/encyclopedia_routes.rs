//! Candidate encyclopedia caller routing and source-backed world admission.
//!
//! This module deliberately does not infer the original type-`0xf2` predicate
//! from port-only world fields. The caller supplies reviewed selected-view and
//! ancestry evidence, while this adapter contributes only current-world
//! instantiation and class-instance rebinding. E32 owns the production gate.

#![allow(
    dead_code,
    reason = "candidate/context route APIs remain deliberately gated until E32"
)]

use rebellion_core::ids::DatId;
use rebellion_core::ids::{
    CharacterKey, DefenseFacilityKey, FleetKey, ManufacturingFacilityKey, ProductionFacilityKey,
    SpecialForceKey, SystemKey, TroopKey,
};
use rebellion_core::world::GameWorld;
#[cfg(any(test, feature = "interface-test-fixtures"))]
use rebellion_core::{
    dat::{ExplorationStatus, Faction},
    world::{ControlKind, System},
};
use rebellion_data::encyclopedia::{
    AdmissionFact, AdmissionSnapshot, AdmittedBinding, BindingKey, CatalogBinding,
    EncyclopediaCatalog, EncyclopediaError, SystemSourceAncestry, ViewerFaction,
};
#[cfg(any(test, feature = "interface-test-fixtures"))]
use rebellion_render::{apply_encyclopedia_action, EncyclopediaAction, EncyclopediaView};
#[cfg(any(test, feature = "interface-test-fixtures"))]
use rebellion_render::{strategic_primary_controls, CockpitLayout, CockpitState};
use rebellion_render::{
    CockpitButton, EncyclopediaMode, EncyclopediaNavigationState, EncyclopediaSelection,
    NavigationOutcome,
};

#[cfg(any(test, feature = "interface-test-fixtures"))]
use std::collections::BTreeMap;
#[cfg(any(test, feature = "interface-test-fixtures"))]
use std::sync::Arc;

#[cfg(any(test, feature = "interface-test-fixtures"))]
use rebellion_render::inspect_encyclopedia_bytes;

#[cfg(any(test, feature = "interface-test-fixtures"))]
use crate::encyclopedia_presenter::{build_encyclopedia_view, EncyclopediaPresenter};
use crate::encyclopedia_session::EncyclopediaAvailability;
#[cfg(any(test, feature = "interface-test-fixtures"))]
use crate::encyclopedia_session::{
    prepare_encyclopedia_session, EncyclopediaBytes, EncyclopediaSession,
};

/// Original command-center command recovered for Encyclopedia/F7.
pub const ENCYCLOPEDIA_COMMAND_ID: u16 = CockpitButton::Encyclopedia.command_id();

const SYSTEM_FAMILY_PATH: &str = "$.admission.system_family";
const SYSTEM_FACTS_PATH: &str = "$.admission.systems";
#[cfg(any(test, feature = "interface-test-fixtures"))]
const SYNTHETIC_SYSTEM_FAMILY: &str = "systems_world_locations";

#[cfg(any(test, feature = "interface-test-fixtures"))]
const CANDIDATE_VALID_CATALOG: &[u8] =
    include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/catalog.json");
#[cfg(any(test, feature = "interface-test-fixtures"))]
const CANDIDATE_VALID_MANIFEST: &[u8] =
    include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/manifest.json");
#[cfg(any(test, feature = "interface-test-fixtures"))]
const CANDIDATE_VALID_IMAGE_1: &[u8] =
    include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.001");
#[cfg(any(test, feature = "interface-test-fixtures"))]
const CANDIDATE_VALID_IMAGE_2: &[u8] =
    include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.002");
#[cfg(any(test, feature = "interface-test-fixtures"))]
const CANDIDATE_VALID_IMAGE_3: &[u8] =
    include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.003");

/// Stable caller focus. No slotmap handle crosses a world replacement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncyclopediaFocusToken {
    CockpitCommand(u16),
    Binding(BindingKey),
}

/// Application-owned handoff from the modal encyclopedia surface back to its
/// stable caller. A return only queues the stable token. Focus transfers when
/// the matching live caller supplies its registered [`egui::Response`]; the
/// handoff never fabricates an unregistered widget ID.
#[derive(Debug, Default)]
pub struct EncyclopediaCallerFocusHandoff {
    pending: Option<EncyclopediaFocusToken>,
    restored: Option<(EncyclopediaFocusToken, egui_macroquad::egui::Id)>,
}

impl EncyclopediaCallerFocusHandoff {
    /// Queue focus for the matching caller's next registered frame.
    pub fn restore(&mut self, focus: EncyclopediaFocusToken) {
        self.pending = Some(focus);
        self.restored = None;
    }

    #[cfg(any(test, feature = "interface-test-fixtures"))]
    fn register(
        &mut self,
        ctx: &egui_macroquad::egui::Context,
        focus: &EncyclopediaFocusToken,
        response: &egui_macroquad::egui::Response,
    ) -> CandidateCallerRegistration {
        let restored_now = self.pending.as_ref() == Some(focus);
        if restored_now {
            response.request_focus();
            self.pending = None;
            self.restored = Some((focus.clone(), response.id));
        }
        let focused = self.focused_caller(ctx) == Some(focus);
        CandidateCallerRegistration {
            focused,
            restored_now,
        }
    }

    /// Returns the restored caller only while egui still owns that focus ID.
    #[must_use]
    pub fn focused_caller(
        &self,
        ctx: &egui_macroquad::egui::Context,
    ) -> Option<&EncyclopediaFocusToken> {
        let (focus, focus_id) = self.restored.as_ref()?;
        ctx.memory(|memory| (memory.focused() == Some(*focus_id)).then_some(focus))
    }

    /// Stable caller retained while restoration is pending or currently owned.
    #[must_use]
    pub fn caller(&self) -> Option<&EncyclopediaFocusToken> {
        self.pending
            .as_ref()
            .or_else(|| self.restored.as_ref().map(|(focus, _)| focus))
    }
}

#[cfg(any(test, feature = "interface-test-fixtures"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CandidateCallerRegistration {
    focused: bool,
    restored_now: bool,
}

/// Result from the real feature-only caller widget drawn by the application.
#[cfg(any(test, feature = "interface-test-fixtures"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CandidateCallerFrame {
    pub activated: bool,
    pub focused: bool,
    pub restored_now: bool,
}

/// Register and draw the actual candidate caller for this stable token.
///
/// Command callers use a focusable widget over the real recovered cockpit
/// control. Context callers use a persistent modeless test window so closing
/// Encyclopedia cannot close or replace its caller.
#[cfg(any(test, feature = "interface-test-fixtures"))]
pub fn draw_candidate_caller(
    ctx: &egui_macroquad::egui::Context,
    cockpit: &CockpitState,
    layout: CockpitLayout,
    focus: &EncyclopediaFocusToken,
    handoff: &mut EncyclopediaCallerFocusHandoff,
) -> CandidateCallerFrame {
    let response = match focus {
        EncyclopediaFocusToken::CockpitCommand(command_id) => {
            let button = strategic_primary_controls(cockpit.faction)
                .iter()
                .find(|control| control.button == CockpitButton::Encyclopedia)
                .filter(|control| control.command_id == *command_id)
                .expect("the recovered Encyclopedia command has its live cockpit control");
            let screen_rect = egui_macroquad::egui::Rect::from_min_size(
                egui_macroquad::egui::pos2(
                    layout.canvas.x + button.rect.x * layout.scale,
                    layout.canvas.y + button.rect.y * layout.scale,
                ),
                egui_macroquad::egui::vec2(
                    button.rect.width * layout.scale,
                    button.rect.height * layout.scale,
                ),
            );
            egui_macroquad::egui::Area::new(egui_macroquad::egui::Id::new((
                "candidate-encyclopedia-command-caller",
                command_id,
            )))
            .order(egui_macroquad::egui::Order::Background)
            .fixed_pos(screen_rect.min)
            .show(ctx, |ui| {
                let (_, response) = ui
                    .allocate_exact_size(screen_rect.size(), egui_macroquad::egui::Sense::click());
                response
            })
            .inner
        }
        EncyclopediaFocusToken::Binding(binding) => {
            let mut response = None;
            egui_macroquad::egui::Window::new("Synthetic contextual caller")
                .id(egui_macroquad::egui::Id::new((
                    "candidate-encyclopedia-context-caller",
                    binding.family.as_str(),
                    binding.dat_id,
                    binding.variant.as_str(),
                )))
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Synthetic test caller; not original mapping evidence");
                    response = Some(ui.button("Open Encyclopedia"));
                });
            response.expect("the persistent context caller window is open")
        }
    };

    let registration = handoff.register(ctx, focus, &response);
    let activated = response.clicked();
    if activated {
        ctx.input_mut(|input| {
            input.consume_key(
                egui_macroquad::egui::Modifiers::NONE,
                egui_macroquad::egui::Key::Enter,
            );
            input.consume_key(
                egui_macroquad::egui::Modifiers::NONE,
                egui_macroquad::egui::Key::Space,
            );
        });
    }
    CandidateCallerFrame {
        activated,
        focused: registration.focused,
        restored_now: registration.restored_now,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncyclopediaCaller {
    CommandCenter { focus: EncyclopediaFocusToken },
    Context { focus: EncyclopediaFocusToken },
}

impl EncyclopediaCaller {
    fn focus(&self) -> &EncyclopediaFocusToken {
        match self {
            Self::CommandCenter { focus } | Self::Context { focus } => focus,
        }
    }
}

/// One route request shared by command `0x131`, F7, and contextual callers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaOpenRequest {
    pub caller: EncyclopediaCaller,
    pub entity: Option<BindingKey>,
}

/// Feature/test-only caller exercised through the same application route.
///
/// These values describe how the bounded acceptance fixture enters the route;
/// they are not original-game availability facts and are absent from default
/// production artifacts.
#[cfg(any(test, feature = "interface-test-fixtures"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateFixtureMode {
    Command,
    Context,
}

#[cfg(any(test, feature = "interface-test-fixtures"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateRebindReason {
    Startup,
    NewCampaign,
    SavedWorldLoad,
    ManualReload,
}

#[cfg(any(test, feature = "interface-test-fixtures"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateActionResult {
    pub outcome: NavigationOutcome,
    pub restored_focus: Option<EncyclopediaFocusToken>,
}

/// Feature-only live evidence captured from the actual application state.
///
/// These digests compare one candidate journey within a process. They are not
/// save-file fingerprints and do not broaden the production route contract.
#[cfg(any(test, feature = "interface-test-fixtures"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateFixtureEvidence {
    pub world_fingerprint: String,
    pub rng_fingerprint: String,
    pub caller_window_open: bool,
    pub caller_window_count: usize,
    pub caller_window_system_dat_id: Option<u32>,
}

/// Bounded native/browser candidate fixture.
///
/// It owns a session prepared from exact E37 contributor-authored bytes and
/// supplies explicit synthetic source-equivalent facts to the real E31
/// controller. It never participates in a default build and must not be used
/// as evidence for the original type-`0xf2` mapping.
#[cfg(any(test, feature = "interface-test-fixtures"))]
pub struct CandidateRouteFixture {
    session: EncyclopediaSession,
    presenter: EncyclopediaPresenter,
    mode: CandidateFixtureMode,
    viewer: ViewerFaction,
    fixture_code: u32,
    last_rebind_reason: Option<CandidateRebindReason>,
}

#[cfg(any(test, feature = "interface-test-fixtures"))]
impl CandidateRouteFixture {
    pub fn synthetic(
        mode: CandidateFixtureMode,
        viewer: ViewerFaction,
    ) -> Result<Self, EncyclopediaError> {
        Ok(Self {
            session: synthetic_candidate_session()?,
            presenter: EncyclopediaPresenter::default(),
            mode,
            viewer,
            fixture_code: candidate_fixture_code(mode, viewer),
            last_rebind_reason: None,
        })
    }

    #[cfg(feature = "interface-test-fixtures")]
    pub fn from_request(request: CandidateFixtureRequest) -> Result<Self, EncyclopediaError> {
        let mut fixture = Self::synthetic(request.mode, request.viewer)?;
        fixture.fixture_code = request.code;
        Ok(fixture)
    }

    #[must_use]
    pub fn mode(&self) -> CandidateFixtureMode {
        self.mode
    }

    #[must_use]
    pub fn viewer(&self) -> ViewerFaction {
        self.viewer
    }

    #[must_use]
    pub fn new_controller(&self) -> EncyclopediaRouteController {
        EncyclopediaRouteController::from_catalog(self.session.effective_catalog())
    }

    #[must_use]
    pub fn source_facts(&self, world: &GameWorld, viewer: ViewerFaction) -> SourceAdmissionFacts {
        let systems = self
            .session
            .effective_catalog()
            .bindings
            .iter()
            .filter(|binding| binding.family == SYNTHETIC_SYSTEM_FAMILY)
            .filter_map(|binding| {
                let mut matching = world
                    .systems
                    .values()
                    .filter(|system| system.dat_id.index() == binding.dat_id);
                let system = matching.next()?;
                // Ambiguity is deliberately retained for the controller to
                // diagnose; do not silently choose one of multiple instances.
                Some((
                    binding.key(),
                    SystemAdmissionSourceFact {
                        source_dat_id: system.dat_id,
                        selected_view: viewer,
                        ancestry: SystemSourceAncestry::NoTypeF2,
                    },
                ))
            })
            .collect();
        SourceAdmissionFacts {
            system_family: SYNTHETIC_SYSTEM_FAMILY.to_owned(),
            systems,
        }
    }

    pub fn rebind(
        &mut self,
        routes: &mut EncyclopediaRouteController,
        world: &GameWorld,
        viewer: ViewerFaction,
        reason: CandidateRebindReason,
    ) -> Result<(), EncyclopediaError> {
        let facts = self.source_facts(world, viewer);
        let result = routes.rebind_world(world, viewer, Some(&facts));
        self.last_rebind_reason = Some(reason);
        result
    }

    pub fn open_requested(
        &self,
        routes: &mut EncyclopediaRouteController,
        world: &GameWorld,
    ) -> Result<(), EncyclopediaError> {
        match self.mode {
            CandidateFixtureMode::Command => routes.open_candidate(
                request_for_cockpit_button(CockpitButton::Encyclopedia)
                    .expect("the recovered Encyclopedia/F7 control has one route"),
            ),
            CandidateFixtureMode::Context => open_first_candidate_context(routes, world),
        }
    }

    pub fn build_view(
        &mut self,
        routes: &EncyclopediaRouteController,
    ) -> Result<EncyclopediaView, EncyclopediaError> {
        build_encyclopedia_view(
            &mut self.presenter,
            &self.session,
            routes.admission(),
            "1033",
            &routes.navigation.selection,
        )
    }

    pub fn apply_action(
        &mut self,
        routes: &mut EncyclopediaRouteController,
        view: &EncyclopediaView,
        action: EncyclopediaAction,
    ) -> CandidateActionResult {
        let outcome = apply_encyclopedia_action(&mut routes.navigation, view, action);
        let restored_focus = routes.finish_navigation(outcome.clone());
        CandidateActionResult {
            outcome,
            restored_focus,
        }
    }

    #[must_use]
    pub fn last_rebind_reason(&self) -> Option<CandidateRebindReason> {
        self.last_rebind_reason
    }

    #[must_use]
    pub fn source_profile(&self) -> &str {
        &self.session.base_manifest().source_profile
    }

    #[cfg(feature = "interface-test-fixtures")]
    fn telemetry_bytes(
        &self,
        status: &'static str,
        routes: &EncyclopediaRouteController,
        restored_focus: Option<&EncyclopediaFocusToken>,
        diagnostic: Option<&EncyclopediaError>,
        evidence: &CandidateFixtureEvidence,
    ) -> Result<Vec<u8>, serde_json::Error> {
        let telemetry = CandidateFixtureTelemetry {
            schema_version: 1,
            status,
            code: self.fixture_code,
            route: match self.mode {
                CandidateFixtureMode::Command => "command_0x131_f7",
                CandidateFixtureMode::Context => "contextual_binding",
            },
            viewer: match self.viewer {
                ViewerFaction::Alliance => "alliance",
                ViewerFaction::Empire => "empire",
            },
            source_facts: "synthetic_test_only_not_original_mapping",
            source_profile: self.source_profile(),
            world_epoch: routes.world_epoch(),
            is_open: routes.is_open(),
            selected_topic_id: routes.selected_topic_id(),
            restored_focus: restored_focus.map(|focus| format!("{focus:?}")),
            diagnostic: diagnostic.map(ToString::to_string),
            world_fingerprint: &evidence.world_fingerprint,
            rng_fingerprint: &evidence.rng_fingerprint,
            caller_window_open: evidence.caller_window_open,
            caller_window_count: evidence.caller_window_count,
            caller_window_system_dat_id: evidence.caller_window_system_dat_id,
            production_routes_enabled: production_routes_enabled(),
        };
        serde_json::to_vec(&telemetry)
    }

    #[cfg(feature = "interface-test-fixtures")]
    pub fn emit_status(
        &self,
        status: &'static str,
        routes: &EncyclopediaRouteController,
        restored_focus: Option<&EncyclopediaFocusToken>,
        diagnostic: Option<&EncyclopediaError>,
        evidence: &CandidateFixtureEvidence,
    ) {
        if let Ok(bytes) =
            self.telemetry_bytes(status, routes, restored_focus, diagnostic, evidence)
        {
            emit_candidate_fixture_bytes(&bytes);
        }
    }
}

#[cfg(feature = "interface-test-fixtures")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CandidateFixtureRequest {
    pub code: u32,
    pub mode: CandidateFixtureMode,
    pub viewer: ViewerFaction,
}

#[cfg(feature = "interface-test-fixtures")]
#[derive(serde::Serialize)]
struct CandidateFixtureTelemetry<'a> {
    schema_version: u8,
    status: &'a str,
    code: u32,
    route: &'a str,
    viewer: &'a str,
    source_facts: &'a str,
    source_profile: &'a str,
    world_epoch: u64,
    is_open: bool,
    selected_topic_id: Option<&'a str>,
    restored_focus: Option<String>,
    diagnostic: Option<String>,
    world_fingerprint: &'a str,
    rng_fingerprint: &'a str,
    caller_window_open: bool,
    caller_window_count: usize,
    caller_window_system_dat_id: Option<u32>,
    production_routes_enabled: bool,
}

#[cfg(any(test, feature = "interface-test-fixtures"))]
const CANDIDATE_FIXTURE_MAGIC: u32 = 0xe131_0000;

#[cfg(any(test, feature = "interface-test-fixtures"))]
const fn candidate_fixture_code(mode: CandidateFixtureMode, viewer: ViewerFaction) -> u32 {
    let case = match (mode, viewer) {
        (CandidateFixtureMode::Command, ViewerFaction::Alliance) => 1,
        (CandidateFixtureMode::Command, ViewerFaction::Empire) => 2,
        (CandidateFixtureMode::Context, ViewerFaction::Alliance) => 3,
        (CandidateFixtureMode::Context, ViewerFaction::Empire) => 4,
    };
    CANDIDATE_FIXTURE_MAGIC | case
}

#[cfg(feature = "interface-test-fixtures")]
fn decode_candidate_fixture(code: u32) -> Option<CandidateFixtureRequest> {
    if code & 0xffff_0000 != CANDIDATE_FIXTURE_MAGIC {
        return None;
    }
    let (mode, viewer) = match code & 0xffff {
        1 => (CandidateFixtureMode::Command, ViewerFaction::Alliance),
        2 => (CandidateFixtureMode::Command, ViewerFaction::Empire),
        3 => (CandidateFixtureMode::Context, ViewerFaction::Alliance),
        4 => (CandidateFixtureMode::Context, ViewerFaction::Empire),
        _ => return None,
    };
    Some(CandidateFixtureRequest { code, mode, viewer })
}

#[cfg(all(feature = "interface-test-fixtures", target_arch = "wasm32"))]
extern "C" {
    fn open_rebellion_interface_fixture_code() -> u32;
    fn open_rebellion_interface_fixture_emit(ptr: *const u8, len: usize);
}

#[cfg(all(feature = "interface-test-fixtures", target_arch = "wasm32"))]
pub fn candidate_fixture_request() -> Option<CandidateFixtureRequest> {
    decode_candidate_fixture(unsafe { open_rebellion_interface_fixture_code() })
}

#[cfg(all(feature = "interface-test-fixtures", not(target_arch = "wasm32")))]
pub fn candidate_fixture_request() -> Option<CandidateFixtureRequest> {
    let value = std::env::var("REBELLION_ENCYCLOPEDIA_CALLER_ROUTE_FIXTURE").ok()?;
    let (mode, viewer) = match value.as_str() {
        "command-alliance" => (CandidateFixtureMode::Command, ViewerFaction::Alliance),
        "command-empire" => (CandidateFixtureMode::Command, ViewerFaction::Empire),
        "context-alliance" => (CandidateFixtureMode::Context, ViewerFaction::Alliance),
        "context-empire" => (CandidateFixtureMode::Context, ViewerFaction::Empire),
        _ => return None,
    };
    Some(CandidateFixtureRequest {
        code: candidate_fixture_code(mode, viewer),
        mode,
        viewer,
    })
}

#[cfg(all(feature = "interface-test-fixtures", target_arch = "wasm32"))]
fn emit_candidate_fixture_bytes(bytes: &[u8]) {
    unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
}

#[cfg(all(feature = "interface-test-fixtures", not(target_arch = "wasm32")))]
fn emit_candidate_fixture_bytes(bytes: &[u8]) {
    if let Ok(message) = std::str::from_utf8(bytes) {
        macroquad::logging::info!("[encyclopedia_candidate] {}", message);
    }
}

#[cfg(any(test, feature = "interface-test-fixtures"))]
fn synthetic_candidate_session() -> Result<EncyclopediaSession, EncyclopediaError> {
    let mut catalog: serde_json::Value =
        serde_json::from_slice(CANDIDATE_VALID_CATALOG).map_err(|error| {
            route_error(
                "candidate_fixture_catalog_failed",
                "$.route.fixture.catalog",
                error.to_string(),
            )
        })?;
    // The generic E37 corpus uses `system_locations`; the accepted owned
    // profile and the app's world adapter use `systems_world_locations`.
    // Rewrite only this contributor-authored in-memory test document so the
    // candidate route exercises the real family-qualified app boundary.
    catalog["bindings"][0]["family"] = serde_json::Value::from(SYNTHETIC_SYSTEM_FAMILY);
    let catalog_bytes = serde_json::to_vec(&catalog).map_err(|error| {
        route_error(
            "candidate_fixture_catalog_failed",
            "$.route.fixture.catalog",
            error.to_string(),
        )
    })?;
    let catalog_digest = inspect_encyclopedia_bytes(&catalog_bytes, None)
        .map_err(|detail| {
            route_error(
                "candidate_fixture_inspection_failed",
                "$.route.fixture.catalog",
                detail,
            )
        })?
        .sha256;
    let mut manifest: serde_json::Value = serde_json::from_slice(CANDIDATE_VALID_MANIFEST)
        .map_err(|error| {
            route_error(
                "candidate_fixture_manifest_failed",
                "$.route.fixture.manifest",
                error.to_string(),
            )
        })?;
    manifest["catalog_sha256"] = serde_json::Value::from(catalog_digest.clone());
    manifest["files"]["catalog.json"] = serde_json::Value::from(catalog_digest);
    let manifest_bytes = serde_json::to_vec(&manifest).map_err(|error| {
        route_error(
            "candidate_fixture_manifest_failed",
            "$.route.fixture.manifest",
            error.to_string(),
        )
    })?;
    let bytes: EncyclopediaBytes = BTreeMap::from([
        ("catalog.json".to_owned(), Arc::from(catalog_bytes)),
        ("manifest.json".to_owned(), Arc::from(manifest_bytes)),
        (
            "assets/EDATA.001".to_owned(),
            Arc::from(CANDIDATE_VALID_IMAGE_1),
        ),
        (
            "assets/EDATA.002".to_owned(),
            Arc::from(CANDIDATE_VALID_IMAGE_2),
        ),
        (
            "assets/EDATA.003".to_owned(),
            Arc::from(CANDIDATE_VALID_IMAGE_3),
        ),
    ]);
    let dats = BTreeMap::from([(
        "SYNTHETIC.DAT".to_owned(),
        "39fb2c329d34fbdd94bb2a2a596b694597eb00b8b402905ec4920f560210d322".to_owned(),
    )]);
    prepare_encyclopedia_session(bytes, &dats)
}

/// Minimal self-contained world for the bounded native/browser acceptance route.
///
/// Its identities and typed admission facts are synthetic test evidence only;
/// they do not assert that any port world field is equivalent to original
/// ancestry type `0xf2`.
#[cfg(any(test, feature = "interface-test-fixtures"))]
#[must_use]
pub fn candidate_fixture_world() -> GameWorld {
    let mut world = GameWorld::default();
    world.systems.insert(candidate_fixture_system(
        7,
        "Synthetic Alliance context",
        Faction::Alliance,
    ));
    world.systems.insert(candidate_fixture_system(
        8,
        "Synthetic Empire anchor",
        Faction::Empire,
    ));
    world
}

#[cfg(any(test, feature = "interface-test-fixtures"))]
fn candidate_fixture_system(dat_id: u32, name: &str, faction: Faction) -> System {
    System {
        dat_id: DatId::new(dat_id),
        name: name.to_owned(),
        sector: Default::default(),
        x: 0,
        y: 0,
        exploration_status: ExplorationStatus::Unexplored,
        popularity_alliance: 0.0,
        popularity_empire: 0.0,
        is_populated: false,
        total_energy: 0,
        raw_materials: 0,
        espionage_rating: 0.0,
        fleets: Vec::new(),
        ground_units: Vec::new(),
        special_forces: Vec::new(),
        defense_facilities: Vec::new(),
        manufacturing_facilities: Vec::new(),
        production_facilities: Vec::new(),
        is_headquarters: true,
        is_destroyed: false,
        control: ControlKind::Controlled(faction),
    }
}

#[cfg(any(test, feature = "interface-test-fixtures"))]
fn open_first_candidate_context(
    routes: &mut EncyclopediaRouteController,
    world: &GameWorld,
) -> Result<(), EncyclopediaError> {
    let mut entities = Vec::new();
    entities.extend(world.systems.keys().map(WorldEntityRef::System));
    for (fleet_key, fleet) in &world.fleets {
        entities.extend((0..fleet.capital_ships.len()).map(|ship_index| {
            WorldEntityRef::FleetCapitalShip {
                fleet: fleet_key,
                ship_index,
            }
        }));
        entities.extend((0..fleet.fighters.len()).map(|fighter_index| {
            WorldEntityRef::FleetFighter {
                fleet: fleet_key,
                fighter_index,
            }
        }));
    }
    entities.extend(world.troops.keys().map(WorldEntityRef::Troop));
    entities.extend(
        world
            .special_forces
            .keys()
            .map(WorldEntityRef::SpecialForce),
    );
    entities.extend(world.characters.keys().map(WorldEntityRef::Character));
    entities.extend(
        world
            .defense_facilities
            .keys()
            .map(WorldEntityRef::DefenseFacility),
    );
    entities.extend(
        world
            .manufacturing_facilities
            .keys()
            .map(WorldEntityRef::ManufacturingFacility),
    );
    entities.extend(
        world
            .production_facilities
            .keys()
            .map(WorldEntityRef::ProductionFacility),
    );

    let mut last_error = None;
    for entity in entities {
        let binding = resolve_world_entity(world, entity)?;
        let request = contextual_open_request(world, binding.clone(), entity)?;
        match routes.open_candidate(request) {
            Ok(()) => return Ok(()),
            Err(error) => last_error = Some(error),
        }
    }
    Err(route_error(
        "candidate_context_unavailable",
        "$.route.fixture.context",
        last_error.map_or_else(
            || "the current synthetic test world contains no contextual entity".to_owned(),
            |error| format!("no current entity resolves to an admitted fixture topic: {error}"),
        ),
    ))
}

/// Exact source-equivalent facts for one potential system topic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemAdmissionSourceFact {
    /// Full family-qualified `DatId` observed for the current instantiated system.
    pub source_dat_id: DatId,
    pub selected_view: ViewerFaction,
    pub ancestry: SystemSourceAncestry,
}

/// Reviewed system family plus complete facts for currently relevant candidates.
///
/// Each key stays family-qualified even though the first profile has one
/// system binding family. Facts for potential-but-absent systems are allowed;
/// absence in `GameWorld` still prevents admission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceAdmissionFacts {
    pub system_family: String,
    pub systems: Vec<(BindingKey, SystemAdmissionSourceFact)>,
}

/// Ephemeral world references accepted only long enough to derive a stable key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorldEntityRef {
    System(SystemKey),
    FleetCapitalShip {
        fleet: FleetKey,
        ship_index: usize,
    },
    FleetFighter {
        fleet: FleetKey,
        fighter_index: usize,
    },
    Troop(TroopKey),
    SpecialForce(SpecialForceKey),
    Character(CharacterKey),
    DefenseFacility(DefenseFacilityKey),
    ManufacturingFacility(ManufacturingFacilityKey),
    ProductionFacility(ProductionFacilityKey),
}

#[derive(Debug, Clone)]
struct RouteCatalog {
    bindings: Vec<CatalogBinding>,
}

impl RouteCatalog {
    fn from_catalog(catalog: &EncyclopediaCatalog) -> Self {
        Self {
            bindings: catalog.bindings.clone(),
        }
    }

    fn resolve_topic_id(&self, key: &BindingKey) -> Result<&str, EncyclopediaError> {
        let mut matching = self.bindings.iter().filter(|binding| binding.key() == *key);
        let first = matching.next().ok_or_else(|| {
            route_error(
                "unavailable_binding",
                binding_path(key),
                format_binding_detail("no catalog topic matches", key),
            )
        })?;
        if matching.next().is_some() {
            return Err(route_error(
                "ambiguous_binding",
                binding_path(key),
                format_binding_detail("multiple catalog topics match", key),
            ));
        }
        Ok(&first.topic_id.0)
    }
}

/// Candidate route state retained beside the world and never serialized.
#[derive(Debug, Clone)]
pub struct EncyclopediaRouteController {
    catalog: RouteCatalog,
    admission: Option<AdmissionSnapshot>,
    admission_error: Option<EncyclopediaError>,
    active_request: Option<EncyclopediaOpenRequest>,
    navigation: EncyclopediaNavigationState,
    world_epoch: u64,
    content_generation: Option<u64>,
}

impl EncyclopediaRouteController {
    #[must_use]
    pub fn from_catalog(catalog: &EncyclopediaCatalog) -> Self {
        Self {
            catalog: RouteCatalog::from_catalog(catalog),
            admission: None,
            admission_error: None,
            active_request: None,
            navigation: EncyclopediaNavigationState::default(),
            world_epoch: 0,
            content_generation: None,
        }
    }

    /// Constructs candidate routing only from a fully validated session.
    pub fn from_availability(
        availability: &EncyclopediaAvailability,
    ) -> Result<Self, EncyclopediaError> {
        match availability {
            EncyclopediaAvailability::Ready(session) => {
                Ok(Self::from_catalog(session.effective_catalog()))
            }
            EncyclopediaAvailability::Unavailable(detail) => Err(route_error(
                "encyclopedia_unavailable",
                "$.route.session",
                detail.clone(),
            )),
        }
    }

    /// Re-evaluates current-world membership. The prior snapshot is never used
    /// after a failed or successful world replacement.
    pub fn rebind_world(
        &mut self,
        world: &GameWorld,
        viewer: ViewerFaction,
        source: Option<&SourceAdmissionFacts>,
    ) -> Result<(), EncyclopediaError> {
        self.world_epoch = self.world_epoch.checked_add(1).ok_or_else(|| {
            route_error(
                "world_epoch_overflow",
                "$.admission.world_epoch",
                "world replacement epoch overflowed",
            )
        })?;
        let result = source.ok_or_else(|| {
            route_error(
                "missing_source_admission_facts",
                "$.admission",
                "reviewed selected-view and type-0xf2 ancestry facts are unavailable",
            )
        });
        let result = result.and_then(|source| {
            build_admission_snapshot_from_bindings(
                &self.catalog.bindings,
                world,
                self.world_epoch,
                viewer,
                source,
            )
        });

        match result {
            Ok(snapshot) => {
                self.admission = Some(snapshot);
                self.admission_error = None;
                self.rebind_active_selection()?;
                Ok(())
            }
            Err(error) => {
                self.admission = None;
                self.admission_error = Some(error.clone());
                self.active_request = None;
                self.navigation = EncyclopediaNavigationState::default();
                Err(error)
            }
        }
    }

    fn rebind_active_selection(&mut self) -> Result<(), EncyclopediaError> {
        let Some(request) = self.active_request.as_ref() else {
            self.navigation = EncyclopediaNavigationState::default();
            return Ok(());
        };
        let selected = match request.entity.as_ref() {
            Some(key) if self.binding_is_admitted(key) => {
                Some(self.catalog.resolve_topic_id(key)?.to_owned())
            }
            _ => None,
        };
        let mode = if selected.is_some() {
            EncyclopediaMode::Topic
        } else {
            EncyclopediaMode::Index
        };
        self.navigation = EncyclopediaNavigationState::new(
            EncyclopediaSelection {
                category_id: None,
                topic_id: selected,
            },
            mode,
        );
        Ok(())
    }

    fn binding_is_admitted(&self, key: &BindingKey) -> bool {
        self.admission.as_ref().is_some_and(|snapshot| {
            snapshot
                .admitted
                .iter()
                .any(|admitted| admitted.key == *key)
        })
    }

    /// Candidate/test entry. Production callers must use [`Self::open_production`].
    pub fn open_candidate(
        &mut self,
        request: EncyclopediaOpenRequest,
    ) -> Result<(), EncyclopediaError> {
        if self.admission.is_none() {
            return Err(self.admission_error.clone().unwrap_or_else(|| {
                route_error(
                    "missing_admission_facts",
                    "$.admission",
                    "world admission has not been evaluated",
                )
            }));
        }
        if let Some(key) = request.entity.as_ref() {
            if !self.binding_is_admitted(key) {
                return Err(route_error(
                    "entity_not_admitted",
                    binding_path(key),
                    format_binding_detail("current world/view does not admit", key),
                ));
            }
            self.catalog.resolve_topic_id(key)?;
        }
        self.active_request = Some(request);
        self.rebind_active_selection()
    }

    /// E32 is the only owner allowed to turn this into an opening route.
    pub fn open_production(
        &mut self,
        _request: EncyclopediaOpenRequest,
    ) -> Result<(), EncyclopediaError> {
        Err(route_error(
            "production_route_gate_closed",
            "$.route.production",
            "candidate caller routes remain disabled until the E32 evidence gate",
        ))
    }

    /// Closes only for renderer-forwarded source Return/close intentions and
    /// yields the caller's stable focus token to the app.
    pub fn finish_navigation(
        &mut self,
        outcome: NavigationOutcome,
    ) -> Option<EncyclopediaFocusToken> {
        if !matches!(
            outcome,
            NavigationOutcome::ReturnForwarded | NavigationOutcome::CloseRequested
        ) {
            return None;
        }
        let request = self.active_request.take()?;
        self.navigation = EncyclopediaNavigationState::default();
        Some(request.caller.focus().clone())
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        self.active_request.is_some()
    }

    #[must_use]
    pub fn active_caller_focus(&self) -> Option<&EncyclopediaFocusToken> {
        self.active_request
            .as_ref()
            .map(|request| request.caller.focus())
    }

    #[must_use]
    pub fn admission(&self) -> Option<&AdmissionSnapshot> {
        self.admission.as_ref()
    }

    #[must_use]
    pub fn navigation(&self) -> &EncyclopediaNavigationState {
        &self.navigation
    }

    #[cfg(feature = "interface-test-fixtures")]
    pub(crate) fn navigation_mut(&mut self) -> &mut EncyclopediaNavigationState {
        &mut self.navigation
    }

    #[must_use]
    pub fn selected_topic_id(&self) -> Option<&str> {
        self.navigation.selection.topic_id.as_deref()
    }

    #[must_use]
    pub const fn world_epoch(&self) -> u64 {
        self.world_epoch
    }

    /// E50 publishes immutable content separately. Bindings cannot be changed
    /// by overlays, so content generation changes retain stable route identity.
    pub fn observe_content_generation(&mut self, generation: Option<u64>) {
        self.content_generation = generation;
    }

    #[must_use]
    pub const fn content_generation(&self) -> Option<u64> {
        self.content_generation
    }
}

#[must_use]
pub const fn production_routes_enabled() -> bool {
    false
}

#[must_use]
pub fn command_center_open_request() -> EncyclopediaOpenRequest {
    EncyclopediaOpenRequest {
        caller: EncyclopediaCaller::CommandCenter {
            focus: EncyclopediaFocusToken::CockpitCommand(ENCYCLOPEDIA_COMMAND_ID),
        },
        entity: None,
    }
}

#[must_use]
pub fn request_for_cockpit_button(button: CockpitButton) -> Option<EncyclopediaOpenRequest> {
    (button == CockpitButton::Encyclopedia).then(command_center_open_request)
}

pub fn contextual_open_request(
    world: &GameWorld,
    caller_focus: BindingKey,
    entity: WorldEntityRef,
) -> Result<EncyclopediaOpenRequest, EncyclopediaError> {
    Ok(EncyclopediaOpenRequest {
        caller: EncyclopediaCaller::Context {
            focus: EncyclopediaFocusToken::Binding(caller_focus),
        },
        entity: Some(resolve_world_entity(world, entity)?),
    })
}

pub fn build_admission_snapshot(
    catalog: &EncyclopediaCatalog,
    world: &GameWorld,
    world_epoch: u64,
    viewer: ViewerFaction,
    source: &SourceAdmissionFacts,
) -> Result<AdmissionSnapshot, EncyclopediaError> {
    build_admission_snapshot_from_bindings(&catalog.bindings, world, world_epoch, viewer, source)
}

fn build_admission_snapshot_from_bindings(
    bindings: &[CatalogBinding],
    world: &GameWorld,
    world_epoch: u64,
    viewer: ViewerFaction,
    source: &SourceAdmissionFacts,
) -> Result<AdmissionSnapshot, EncyclopediaError> {
    if !bindings
        .iter()
        .any(|binding| binding.family == source.system_family)
    {
        return Err(route_error(
            "unknown_system_binding_family",
            SYSTEM_FAMILY_PATH,
            format!(
                "reviewed system family {:?} is absent from the validated catalog",
                source.system_family
            ),
        ));
    }

    for (index, (key, fact)) in source.systems.iter().enumerate() {
        if source.systems[..index]
            .iter()
            .any(|(prior_key, _)| prior_key == key)
        {
            return Err(route_error(
                "ambiguous_system_admission_fact",
                binding_path(key),
                format_binding_detail("multiple source facts identify", key),
            ));
        }
        if key.family != source.system_family
            || !bindings.iter().any(|binding| binding.key() == *key)
        {
            return Err(route_error(
                "unknown_system_admission_fact",
                binding_path(key),
                format_binding_detail("source fact does not identify a catalog system", key),
            ));
        }
        if fact.source_dat_id.index() != key.dat_id {
            return Err(route_error(
                "source_identity_mismatch",
                binding_path(key),
                format!(
                    "source DatId {} has index {}, not binding DatId {}",
                    fact.source_dat_id,
                    fact.source_dat_id.index(),
                    key.dat_id
                ),
            ));
        }
    }

    let mut admitted = Vec::with_capacity(bindings.len());
    for binding in bindings {
        let key = binding.key();
        if binding.family != source.system_family {
            admitted.push(AdmittedBinding {
                key,
                fact: AdmissionFact::DefinitionPresent,
            });
            continue;
        }

        let mut current = world
            .systems
            .values()
            .filter(|system| system.dat_id.index() == binding.dat_id);
        let Some(system) = current.next() else {
            continue;
        };
        if current.next().is_some() {
            return Err(route_error(
                "ambiguous_world_system",
                binding_path(&key),
                format_binding_detail("multiple current systems match", &key),
            ));
        }
        let mut matching_facts = source
            .systems
            .iter()
            .filter(|(source_key, _)| source_key == &key);
        let fact = matching_facts.next().map(|(_, fact)| fact).ok_or_else(|| {
            route_error(
                "missing_system_admission_fact",
                binding_path(&key),
                format_binding_detail(
                    "instantiated system lacks selected-view/type-0xf2 evidence for",
                    &key,
                ),
            )
        })?;
        if matching_facts.next().is_some() {
            return Err(route_error(
                "ambiguous_system_admission_fact",
                binding_path(&key),
                format_binding_detail("multiple source facts identify", &key),
            ));
        }
        if system.dat_id != fact.source_dat_id {
            return Err(route_error(
                "source_identity_mismatch",
                binding_path(&key),
                format!(
                    "current world has {} but reviewed fact names {}",
                    system.dat_id, fact.source_dat_id
                ),
            ));
        }
        if fact.selected_view == viewer && fact.ancestry == SystemSourceAncestry::NoTypeF2 {
            admitted.push(AdmittedBinding {
                key,
                fact: AdmissionFact::InstantiatedSystem {
                    selected_view: fact.selected_view,
                    ancestry: fact.ancestry,
                },
            });
        }
    }

    Ok(AdmissionSnapshot {
        world_epoch,
        viewer,
        admitted,
    })
}

pub fn resolve_world_entity(
    world: &GameWorld,
    entity: WorldEntityRef,
) -> Result<BindingKey, EncyclopediaError> {
    let (family, dat_id) = match entity {
        WorldEntityRef::System(key) => (
            "systems_world_locations",
            world
                .systems
                .get(key)
                .map(|system| system.dat_id)
                .ok_or_else(|| missing_world_entity("system"))?,
        ),
        WorldEntityRef::FleetCapitalShip { fleet, ship_index } => {
            let fleet = world
                .fleets
                .get(fleet)
                .ok_or_else(|| missing_world_entity("fleet"))?;
            let ship = fleet
                .capital_ships
                .get(ship_index)
                .ok_or_else(|| missing_world_entity("fleet capital-ship instance"))?;
            (
                "capital_ship_classes",
                world
                    .capital_ship_classes
                    .get(ship.class)
                    .map(|class| class.dat_id)
                    .ok_or_else(|| missing_world_entity("capital-ship class"))?,
            )
        }
        WorldEntityRef::FleetFighter {
            fleet,
            fighter_index,
        } => {
            let fleet = world
                .fleets
                .get(fleet)
                .ok_or_else(|| missing_world_entity("fleet"))?;
            let fighter = fleet
                .fighters
                .get(fighter_index)
                .ok_or_else(|| missing_world_entity("fleet fighter instance"))?;
            (
                "fighter_classes",
                world
                    .fighter_classes
                    .get(fighter.class)
                    .map(|class| class.dat_id)
                    .ok_or_else(|| missing_world_entity("fighter class"))?,
            )
        }
        WorldEntityRef::Troop(key) => (
            "troop_classes",
            world
                .troops
                .get(key)
                .map(|unit| unit.class_dat_id)
                .ok_or_else(|| missing_world_entity("troop instance"))?,
        ),
        WorldEntityRef::SpecialForce(key) => (
            "special_force_classes",
            world
                .special_forces
                .get(key)
                .map(|unit| unit.class_dat_id)
                .ok_or_else(|| missing_world_entity("special-force instance"))?,
        ),
        WorldEntityRef::Character(key) => {
            let character = world
                .characters
                .get(key)
                .ok_or_else(|| missing_world_entity("character"))?;
            (
                if character.is_major {
                    "major_characters"
                } else {
                    "minor_characters"
                },
                character.dat_id,
            )
        }
        WorldEntityRef::DefenseFacility(key) => (
            "defense_facilities",
            world
                .defense_facilities
                .get(key)
                .map(|facility| facility.class_dat_id)
                .ok_or_else(|| missing_world_entity("defense-facility instance"))?,
        ),
        WorldEntityRef::ManufacturingFacility(key) => (
            "manufacturing_facilities",
            world
                .manufacturing_facilities
                .get(key)
                .map(|facility| facility.class_dat_id)
                .ok_or_else(|| missing_world_entity("manufacturing-facility instance"))?,
        ),
        WorldEntityRef::ProductionFacility(key) => (
            "production_facilities",
            world
                .production_facilities
                .get(key)
                .map(|facility| facility.class_dat_id)
                .ok_or_else(|| missing_world_entity("production-facility instance"))?,
        ),
    };
    Ok(BindingKey {
        family: family.to_owned(),
        dat_id: dat_id.index(),
        variant: "default".to_owned(),
    })
}

fn missing_world_entity(kind: &str) -> EncyclopediaError {
    route_error(
        "missing_world_entity",
        "$.request.entity",
        format!("current world no longer contains the requested {kind}"),
    )
}

fn route_error(
    code: &'static str,
    path: impl Into<String>,
    detail: impl Into<String>,
) -> EncyclopediaError {
    EncyclopediaError::for_session(code, path, detail)
}

fn binding_path(key: &BindingKey) -> String {
    format!(
        "{SYSTEM_FACTS_PATH}[{:?}, {}, {:?}]",
        key.family, key.dat_id, key.variant
    )
}

fn format_binding_detail(prefix: &str, key: &BindingKey) -> String {
    format!(
        "{prefix} family {:?}, DatId {}, variant {:?}",
        key.family, key.dat_id, key.variant
    )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::Arc;

    use rand::{RngCore, SeedableRng};
    use rand_xoshiro::Xoshiro256PlusPlus;
    use rebellion_core::dat::ExplorationStatus;
    use rebellion_core::ids::DatId;
    use rebellion_core::world::{
        CapitalShipClass, ControlKind, Fleet, GameWorld, ShipInstance, System,
    };
    use rebellion_data::encyclopedia::{
        parse_catalog, AdmissionFact, BindingKey, SystemSourceAncestry, ViewerFaction,
    };
    use rebellion_render::inspect_encyclopedia_bytes;
    use rebellion_render::{CockpitButton, EncyclopediaAction, NavigationOutcome};
    use serde_json::Value;

    #[cfg(feature = "interface-test-fixtures")]
    use super::decode_candidate_fixture;
    #[cfg(feature = "interface-test-fixtures")]
    use super::CandidateFixtureEvidence;
    use super::{
        build_admission_snapshot, candidate_fixture_world, command_center_open_request,
        contextual_open_request, draw_candidate_caller, production_routes_enabled,
        request_for_cockpit_button, resolve_world_entity, CandidateFixtureMode,
        CandidateRebindReason, CandidateRouteFixture, EncyclopediaCaller,
        EncyclopediaCallerFocusHandoff, EncyclopediaFocusToken, EncyclopediaRouteController,
        SourceAdmissionFacts, SystemAdmissionSourceFact, WorldEntityRef, ENCYCLOPEDIA_COMMAND_ID,
    };
    use crate::encyclopedia_presenter::{build_encyclopedia_view, EncyclopediaPresenter};
    use crate::encyclopedia_session::{
        prepare_encyclopedia_session, EncyclopediaAvailability, EncyclopediaBytes,
        EncyclopediaSession,
    };

    const VALID_CATALOG: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/catalog.json");
    const VALID_MANIFEST: &[u8] =
        include_bytes!("../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/manifest.json");
    const VALID_IMAGE_1: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.001"
    );

    #[cfg(feature = "interface-test-fixtures")]
    #[test]
    fn candidate_telemetry_reports_real_caller_window_and_live_fingerprints() {
        let fixture = CandidateRouteFixture::synthetic(
            CandidateFixtureMode::Context,
            ViewerFaction::Alliance,
        )
        .unwrap();
        let routes = fixture.new_controller();
        let evidence = CandidateFixtureEvidence {
            world_fingerprint: "fnv1a64:1111111111111111".to_owned(),
            rng_fingerprint: "fnv1a64:2222222222222222".to_owned(),
            caller_window_open: true,
            caller_window_count: 1,
            caller_window_system_dat_id: Some(7),
        };

        let bytes = fixture
            .telemetry_bytes("candidate_ready", &routes, None, None, &evidence)
            .unwrap();
        let value: Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(value["world_fingerprint"], "fnv1a64:1111111111111111");
        assert_eq!(value["rng_fingerprint"], "fnv1a64:2222222222222222");
        assert_eq!(value["caller_window_open"], true);
        assert_eq!(value["caller_window_count"], 1);
        assert_eq!(value["caller_window_system_dat_id"], 7);
        assert_eq!(value["production_routes_enabled"], false);
    }
    const VALID_IMAGE_2: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.002"
    );
    const VALID_IMAGE_3: &[u8] = include_bytes!(
        "../../../tests/fixtures/encyclopedia/fixtures/bundles/valid/assets/EDATA.003"
    );

    fn catalog() -> rebellion_data::encyclopedia::EncyclopediaCatalog {
        parse_catalog(VALID_CATALOG).unwrap()
    }

    fn session() -> EncyclopediaSession {
        let catalog_bytes = VALID_CATALOG.to_vec();
        let catalog_digest = inspect_encyclopedia_bytes(&catalog_bytes, None)
            .unwrap()
            .sha256;
        let mut manifest: Value = serde_json::from_slice(VALID_MANIFEST).unwrap();
        manifest["catalog_sha256"] = Value::from(catalog_digest.clone());
        manifest["files"]["catalog.json"] = Value::from(catalog_digest);
        let bytes: EncyclopediaBytes = BTreeMap::from([
            ("catalog.json".to_owned(), Arc::from(catalog_bytes)),
            (
                "manifest.json".to_owned(),
                Arc::from(serde_json::to_vec(&manifest).unwrap()),
            ),
            ("assets/EDATA.001".to_owned(), Arc::from(VALID_IMAGE_1)),
            ("assets/EDATA.002".to_owned(), Arc::from(VALID_IMAGE_2)),
            ("assets/EDATA.003".to_owned(), Arc::from(VALID_IMAGE_3)),
        ]);
        let dats = BTreeMap::from([(
            "SYNTHETIC.DAT".to_owned(),
            "39fb2c329d34fbdd94bb2a2a596b694597eb00b8b402905ec4920f560210d322".to_owned(),
        )]);
        prepare_encyclopedia_session(bytes, &dats).unwrap()
    }

    fn system(dat_id: u32) -> System {
        System {
            dat_id: DatId::new(dat_id),
            name: "Synthetic system".to_owned(),
            sector: Default::default(),
            x: 0,
            y: 0,
            exploration_status: ExplorationStatus::Unexplored,
            popularity_alliance: 0.0,
            popularity_empire: 0.0,
            is_populated: false,
            total_energy: 0,
            raw_materials: 0,
            espionage_rating: 0.0,
            fleets: Vec::new(),
            ground_units: Vec::new(),
            special_forces: Vec::new(),
            defense_facilities: Vec::new(),
            manufacturing_facilities: Vec::new(),
            production_facilities: Vec::new(),
            is_headquarters: false,
            is_destroyed: false,
            control: ControlKind::Uncontrolled,
        }
    }

    fn system_binding() -> BindingKey {
        BindingKey {
            family: "system_locations".to_owned(),
            dat_id: 7,
            variant: "default".to_owned(),
        }
    }

    fn facts(selected_view: ViewerFaction, ancestry: SystemSourceAncestry) -> SourceAdmissionFacts {
        SourceAdmissionFacts {
            system_family: "system_locations".to_owned(),
            systems: vec![(
                system_binding(),
                SystemAdmissionSourceFact {
                    source_dat_id: DatId::new(0x9000_0007),
                    selected_view,
                    ancestry,
                },
            )],
        }
    }

    #[test]
    fn complete_source_facts_admit_current_system_and_definitions_for_both_viewers() {
        let catalog = catalog();
        let mut world = GameWorld::default();
        world.systems.insert(system(0x9000_0007));

        for viewer in [ViewerFaction::Alliance, ViewerFaction::Empire] {
            let snapshot = build_admission_snapshot(
                &catalog,
                &world,
                7,
                viewer,
                &facts(viewer, SystemSourceAncestry::NoTypeF2),
            )
            .unwrap();

            assert_eq!(snapshot.world_epoch, 7);
            assert_eq!(snapshot.viewer, viewer);
            assert_eq!(snapshot.admitted.len(), catalog.bindings.len());
            let admitted_system = snapshot
                .admitted
                .iter()
                .find(|admitted| admitted.key == system_binding())
                .unwrap();
            assert_eq!(
                admitted_system.fact,
                AdmissionFact::InstantiatedSystem {
                    selected_view: viewer,
                    ancestry: SystemSourceAncestry::NoTypeF2,
                }
            );
        }
    }

    #[test]
    fn absent_wrong_view_and_type_f2_systems_are_not_admitted() {
        let catalog = catalog();
        let empty = GameWorld::default();
        let absent = build_admission_snapshot(
            &catalog,
            &empty,
            1,
            ViewerFaction::Alliance,
            &facts(ViewerFaction::Alliance, SystemSourceAncestry::NoTypeF2),
        )
        .unwrap();
        assert!(!absent
            .admitted
            .iter()
            .any(|admitted| admitted.key == system_binding()));

        let mut world = GameWorld::default();
        world.systems.insert(system(0x9000_0007));
        for source in [
            facts(ViewerFaction::Empire, SystemSourceAncestry::NoTypeF2),
            facts(
                ViewerFaction::Alliance,
                SystemSourceAncestry::ContainsTypeF2,
            ),
        ] {
            let excluded =
                build_admission_snapshot(&catalog, &world, 2, ViewerFaction::Alliance, &source)
                    .unwrap();
            assert!(!excluded
                .admitted
                .iter()
                .any(|admitted| admitted.key == system_binding()));
        }
    }

    #[test]
    fn instantiated_system_without_source_equivalent_facts_is_unavailable() {
        let catalog = catalog();
        let mut world = GameWorld::default();
        let key = world.systems.insert(system(0x9000_0007));
        world.systems[key].exploration_status = ExplorationStatus::Explored;
        world.systems[key].is_populated = true;

        let error = build_admission_snapshot(
            &catalog,
            &world,
            3,
            ViewerFaction::Alliance,
            &SourceAdmissionFacts {
                system_family: "system_locations".to_owned(),
                systems: Vec::new(),
            },
        )
        .unwrap_err();

        assert_eq!(error.code(), "missing_system_admission_fact");
        assert!(error.to_string().contains("system_locations"));
        assert!(error.to_string().contains("DatId 7"));
    }

    #[test]
    fn source_fact_identity_and_uniqueness_are_checked_before_admission() {
        let catalog = catalog();
        let mut world = GameWorld::default();
        world.systems.insert(system(0x9000_0007));
        let mut wrong_identity = facts(ViewerFaction::Alliance, SystemSourceAncestry::NoTypeF2);
        wrong_identity.systems[0].1.source_dat_id = DatId::new(0x9200_0007);
        assert_eq!(
            build_admission_snapshot(
                &catalog,
                &world,
                1,
                ViewerFaction::Alliance,
                &wrong_identity,
            )
            .unwrap_err()
            .code(),
            "source_identity_mismatch"
        );

        let mut duplicate = facts(ViewerFaction::Alliance, SystemSourceAncestry::NoTypeF2);
        duplicate.systems.push(duplicate.systems[0].clone());
        assert_eq!(
            build_admission_snapshot(&catalog, &world, 1, ViewerFaction::Alliance, &duplicate,)
                .unwrap_err()
                .code(),
            "ambiguous_system_admission_fact"
        );

        // Duplicate evidence is invalid even when the potential system is not
        // instantiated, so validation cannot depend on the world scan below.
        assert_eq!(
            build_admission_snapshot(
                &catalog,
                &GameWorld::default(),
                1,
                ViewerFaction::Alliance,
                &duplicate,
            )
            .unwrap_err()
            .code(),
            "ambiguous_system_admission_fact"
        );
    }

    #[test]
    fn source_fact_family_and_catalog_membership_are_both_required() {
        let catalog = catalog();
        let world = GameWorld::default();
        let unknown_family = SourceAdmissionFacts {
            system_family: "unreviewed_systems".to_owned(),
            systems: Vec::new(),
        };
        assert_eq!(
            build_admission_snapshot(
                &catalog,
                &world,
                1,
                ViewerFaction::Alliance,
                &unknown_family,
            )
            .unwrap_err()
            .code(),
            "unknown_system_binding_family"
        );

        let unknown_member = SourceAdmissionFacts {
            system_family: "system_locations".to_owned(),
            systems: vec![(
                BindingKey {
                    family: "system_locations".to_owned(),
                    dat_id: 999,
                    variant: "default".to_owned(),
                },
                SystemAdmissionSourceFact {
                    source_dat_id: DatId::new(0x9000_03e7),
                    selected_view: ViewerFaction::Alliance,
                    ancestry: SystemSourceAncestry::NoTypeF2,
                },
            )],
        };
        assert_eq!(
            build_admission_snapshot(
                &catalog,
                &world,
                1,
                ViewerFaction::Alliance,
                &unknown_member,
            )
            .unwrap_err()
            .code(),
            "unknown_system_admission_fact"
        );

        let mut wrong_variant = facts(ViewerFaction::Alliance, SystemSourceAncestry::NoTypeF2);
        wrong_variant.systems[0].0.variant = "viewer_faction".to_owned();
        assert_eq!(
            build_admission_snapshot(&catalog, &world, 1, ViewerFaction::Alliance, &wrong_variant,)
                .unwrap_err()
                .code(),
            "unknown_system_admission_fact"
        );
    }

    #[test]
    fn cockpit_command_and_f7_button_share_the_same_index_request() {
        let direct = command_center_open_request();
        let from_button = request_for_cockpit_button(CockpitButton::Encyclopedia).unwrap();

        assert_eq!(direct, from_button);
        assert_eq!(direct.entity, None);
        assert_eq!(
            direct.caller,
            EncyclopediaCaller::CommandCenter {
                focus: EncyclopediaFocusToken::CockpitCommand(ENCYCLOPEDIA_COMMAND_ID),
            }
        );
        assert!(request_for_cockpit_button(CockpitButton::FleetFinder).is_none());
    }

    #[test]
    fn fleet_ship_context_resolves_the_current_class_without_retaining_world_handles() {
        let mut world = GameWorld::default();
        let mut class = CapitalShipClass::default();
        class.dat_id = DatId::new(0x1000_0007);
        let class_key = world.capital_ship_classes.insert(class);
        let fleet_key = world.fleets.insert(Fleet {
            location: Default::default(),
            capital_ships: vec![ShipInstance::new(class_key, 10, true)],
            fighters: Vec::new(),
            characters: Vec::new(),
            is_alliance: true,
            has_death_star: false,
        });
        let caller_focus = BindingKey {
            family: "fleet_definitions".to_owned(),
            dat_id: 4,
            variant: "viewer_faction".to_owned(),
        };

        let request = contextual_open_request(
            &world,
            caller_focus.clone(),
            WorldEntityRef::FleetCapitalShip {
                fleet: fleet_key,
                ship_index: 0,
            },
        )
        .unwrap();
        assert_eq!(
            request.entity,
            Some(BindingKey {
                family: "capital_ship_classes".to_owned(),
                dat_id: 7,
                variant: "default".to_owned(),
            })
        );
        assert_eq!(
            request.caller,
            EncyclopediaCaller::Context {
                focus: EncyclopediaFocusToken::Binding(caller_focus),
            }
        );

        world.fleets.remove(fleet_key);
        let error = resolve_world_entity(
            &world,
            WorldEntityRef::FleetCapitalShip {
                fleet: fleet_key,
                ship_index: 0,
            },
        )
        .unwrap_err();
        assert_eq!(error.code(), "missing_world_entity");
    }

    #[test]
    fn world_replacement_recomputes_epoch_and_drops_deleted_stable_selection() {
        let catalog = catalog();
        let mut original = GameWorld::default();
        let old_handle = original.systems.insert(system(0x9000_0007));
        let mut routes = EncyclopediaRouteController::from_catalog(&catalog);
        routes
            .rebind_world(
                &original,
                ViewerFaction::Alliance,
                Some(&facts(
                    ViewerFaction::Alliance,
                    SystemSourceAncestry::NoTypeF2,
                )),
            )
            .unwrap();
        let request = super::EncyclopediaOpenRequest {
            caller: EncyclopediaCaller::Context {
                focus: EncyclopediaFocusToken::Binding(system_binding()),
            },
            entity: Some(system_binding()),
        };
        routes.open_candidate(request).unwrap();
        assert_eq!(routes.selected_topic_id(), Some("original:60001"));
        assert_eq!(routes.world_epoch(), 1);

        let mut replacement = GameWorld::default();
        let replacement_handle = replacement.systems.insert(system(0x9000_0008));
        routes
            .rebind_world(
                &replacement,
                ViewerFaction::Alliance,
                Some(&SourceAdmissionFacts {
                    system_family: "system_locations".to_owned(),
                    systems: Vec::new(),
                }),
            )
            .unwrap();

        assert_eq!(routes.world_epoch(), 2);
        assert_eq!(routes.selected_topic_id(), None);
        // A replacement world can reuse the same generational slot. The old
        // handle would now name the new system, which is exactly why route
        // state retains the stable binding and re-resolves it instead.
        assert_eq!(old_handle, replacement_handle);
        assert_eq!(
            resolve_world_entity(&replacement, WorldEntityRef::System(old_handle))
                .unwrap()
                .dat_id,
            8
        );
        assert_eq!(
            resolve_world_entity(&replacement, WorldEntityRef::System(replacement_handle))
                .unwrap()
                .dat_id,
            8
        );
    }

    #[test]
    fn candidate_route_supplies_e29_presenter_with_one_shared_selection_model() {
        let session = session();
        let mut world = GameWorld::default();
        world.systems.insert(system(0x9000_0007));
        let mut routes = EncyclopediaRouteController::from_catalog(session.effective_catalog());
        routes
            .rebind_world(
                &world,
                ViewerFaction::Alliance,
                Some(&facts(
                    ViewerFaction::Alliance,
                    SystemSourceAncestry::NoTypeF2,
                )),
            )
            .unwrap();
        routes
            .open_candidate(super::EncyclopediaOpenRequest {
                caller: EncyclopediaCaller::Context {
                    focus: EncyclopediaFocusToken::Binding(system_binding()),
                },
                entity: Some(system_binding()),
            })
            .unwrap();

        let view = build_encyclopedia_view(
            &mut EncyclopediaPresenter::default(),
            &session,
            routes.admission(),
            "1033",
            &routes.navigation().selection,
        )
        .unwrap();
        assert_eq!(
            view.active_topic
                .as_ref()
                .map(|topic| topic.topic_id.as_str()),
            Some("original:60001")
        );
        assert_eq!(view.navigation.world_epoch, routes.world_epoch());
    }

    #[test]
    fn content_generation_observation_keeps_world_identity_and_selection_stable() {
        let catalog = catalog();
        let world = GameWorld::default();
        let mut routes = EncyclopediaRouteController::from_catalog(&catalog);
        routes
            .rebind_world(
                &world,
                ViewerFaction::Empire,
                Some(&SourceAdmissionFacts {
                    system_family: "system_locations".to_owned(),
                    systems: Vec::new(),
                }),
            )
            .unwrap();
        routes
            .open_candidate(command_center_open_request())
            .unwrap();
        let epoch = routes.world_epoch();

        routes.observe_content_generation(Some(2));
        routes.observe_content_generation(Some(2));

        assert_eq!(routes.content_generation(), Some(2));
        assert_eq!(routes.world_epoch(), epoch);
        assert!(routes.is_open());
        assert_eq!(routes.selected_topic_id(), None);
    }

    #[test]
    fn failed_world_rebind_closes_candidate_without_retaining_stale_focus() {
        let catalog = catalog();
        let world = GameWorld::default();
        let mut routes = EncyclopediaRouteController::from_catalog(&catalog);
        routes
            .rebind_world(
                &world,
                ViewerFaction::Alliance,
                Some(&SourceAdmissionFacts {
                    system_family: "system_locations".to_owned(),
                    systems: Vec::new(),
                }),
            )
            .unwrap();
        routes
            .open_candidate(command_center_open_request())
            .unwrap();

        let error = routes
            .rebind_world(&world, ViewerFaction::Empire, None)
            .unwrap_err();

        assert_eq!(error.code(), "missing_source_admission_facts");
        assert!(!routes.is_open());
        assert!(routes.admission().is_none());
    }

    #[test]
    fn return_and_close_restore_stable_caller_without_world_or_rng_effects() {
        let catalog = catalog();
        let world = GameWorld::default();
        let world_before = serde_json::to_vec(&world).unwrap();
        let mut expected_rng = Xoshiro256PlusPlus::seed_from_u64(42);
        let mut actual_rng = expected_rng.clone();
        let mut routes = EncyclopediaRouteController::from_catalog(&catalog);
        routes
            .rebind_world(
                &world,
                ViewerFaction::Alliance,
                Some(&SourceAdmissionFacts {
                    system_family: "system_locations".to_owned(),
                    systems: Vec::new(),
                }),
            )
            .unwrap();

        for outcome in [
            NavigationOutcome::ReturnForwarded,
            NavigationOutcome::CloseRequested,
        ] {
            routes
                .open_candidate(command_center_open_request())
                .unwrap();
            assert_eq!(
                routes.finish_navigation(outcome),
                Some(EncyclopediaFocusToken::CockpitCommand(
                    ENCYCLOPEDIA_COMMAND_ID
                ))
            );
            assert!(!routes.is_open());
        }

        assert_eq!(serde_json::to_vec(&world).unwrap(), world_before);
        assert_eq!(actual_rng.next_u64(), expected_rng.next_u64());
    }

    #[test]
    fn production_route_stays_closed_even_when_candidate_is_ready() {
        let catalog = catalog();
        let world = GameWorld::default();
        let mut routes = EncyclopediaRouteController::from_catalog(&catalog);
        routes
            .rebind_world(
                &world,
                ViewerFaction::Alliance,
                Some(&SourceAdmissionFacts {
                    system_family: "system_locations".to_owned(),
                    systems: Vec::new(),
                }),
            )
            .unwrap();

        assert!(!production_routes_enabled());
        let error = routes
            .open_production(command_center_open_request())
            .unwrap_err();
        assert_eq!(error.code(), "production_route_gate_closed");
        assert!(!routes.is_open());
    }

    #[test]
    fn unavailable_or_namespace_free_session_never_constructs_a_route() {
        let unavailable = EncyclopediaAvailability::Unavailable(
            "old runtime pack contains no encyclopedia namespace".to_owned(),
        );

        let error = EncyclopediaRouteController::from_availability(&unavailable).unwrap_err();

        assert_eq!(error.code(), "encyclopedia_unavailable");
        assert!(error.to_string().contains("no encyclopedia namespace"));
        assert!(!production_routes_enabled());
    }

    #[test]
    fn bounded_candidate_fixture_drives_command_and_context_through_the_real_presenter_actions() {
        let mut world = GameWorld::default();
        world.systems.insert(system(0x9000_0007));

        for viewer in [ViewerFaction::Alliance, ViewerFaction::Empire] {
            for mode in [CandidateFixtureMode::Command, CandidateFixtureMode::Context] {
                let mut fixture = CandidateRouteFixture::synthetic(mode, viewer).unwrap();
                let mut routes = fixture.new_controller();
                fixture
                    .rebind(&mut routes, &world, viewer, CandidateRebindReason::Startup)
                    .unwrap();
                fixture.open_requested(&mut routes, &world).unwrap();

                let view = fixture.build_view(&routes).unwrap();
                assert_eq!(view.navigation.world_epoch, routes.world_epoch());
                assert_eq!(
                    view.navigation.selected_topic_id.as_deref(),
                    routes.selected_topic_id()
                );
                if mode == CandidateFixtureMode::Context {
                    assert_eq!(
                        view.active_topic
                            .as_ref()
                            .map(|topic| topic.topic_id.as_str()),
                        Some("original:60001")
                    );
                } else {
                    assert!(view.active_topic.is_none());
                }

                let world_before = serde_json::to_vec(&world).unwrap();
                let mut expected_rng = Xoshiro256PlusPlus::seed_from_u64(0x131);
                let mut actual_rng = expected_rng.clone();
                let action = if mode == CandidateFixtureMode::Context {
                    EncyclopediaAction::Return
                } else {
                    EncyclopediaAction::Close
                };
                let result = fixture.apply_action(&mut routes, &view, action);

                assert!(matches!(
                    result.outcome,
                    NavigationOutcome::ReturnForwarded | NavigationOutcome::CloseRequested
                ));
                assert!(result.restored_focus.is_some());
                assert!(!routes.is_open());
                assert_eq!(serde_json::to_vec(&world).unwrap(), world_before);
                assert_eq!(actual_rng.next_u64(), expected_rng.next_u64());
            }
        }
    }

    #[test]
    fn candidate_return_and_close_restore_a_live_caller_across_frames_and_consume_input() {
        let world = candidate_fixture_world();
        let system = world.systems.keys().next().unwrap();
        for (viewer, cockpit_faction) in [
            (
                ViewerFaction::Alliance,
                rebellion_render::CockpitFaction::Alliance,
            ),
            (
                ViewerFaction::Empire,
                rebellion_render::CockpitFaction::Empire,
            ),
        ] {
            let cockpit = rebellion_render::CockpitState::new(cockpit_faction);
            let mut windows = rebellion_render::SystemWindowState::default();
            assert!(windows.open(
                &world,
                system,
                (80, 60),
                cockpit_faction,
                cockpit.layout_for(640.0, 480.0),
            ));
            let modeless_windows_before = windows.window_count();

            for mode in [CandidateFixtureMode::Command, CandidateFixtureMode::Context] {
                let mut fixture = CandidateRouteFixture::synthetic(mode, viewer).unwrap();
                let mut routes = fixture.new_controller();
                fixture
                    .rebind(&mut routes, &world, viewer, CandidateRebindReason::Startup)
                    .unwrap();
                fixture.open_requested(&mut routes, &world).unwrap();
                let view = fixture.build_view(&routes).unwrap();
                let result = fixture.apply_action(
                    &mut routes,
                    &view,
                    if mode == CandidateFixtureMode::Context {
                        EncyclopediaAction::Return
                    } else {
                        EncyclopediaAction::Close
                    },
                );
                let expected = result.restored_focus.unwrap();
                let ctx = egui_macroquad::egui::Context::default();
                let encyclopedia_child = egui_macroquad::egui::Id::new((
                    "active-encyclopedia-child",
                    mode == CandidateFixtureMode::Context,
                ));
                ctx.memory_mut(|memory| memory.request_focus(encyclopedia_child));
                assert_eq!(
                    ctx.memory(|memory| memory.focused()),
                    Some(encyclopedia_child)
                );

                let mut handoff = EncyclopediaCallerFocusHandoff::default();
                let layout = cockpit.layout_for(640.0, 480.0);
                let mut initial = None;
                let _ = ctx.run(egui_macroquad::egui::RawInput::default(), |ctx| {
                    initial = Some(draw_candidate_caller(
                        ctx,
                        &cockpit,
                        layout,
                        &expected,
                        &mut handoff,
                    ));
                });
                assert!(!initial.unwrap().focused);

                handoff.restore(expected.clone());
                let mut restored = None;
                let _ = ctx.run(egui_macroquad::egui::RawInput::default(), |ctx| {
                    restored = Some(draw_candidate_caller(
                        ctx,
                        &cockpit,
                        layout,
                        &expected,
                        &mut handoff,
                    ));
                });
                assert_eq!(
                    restored,
                    Some(super::CandidateCallerFrame {
                        activated: false,
                        focused: true,
                        restored_now: true,
                    })
                );
                assert_eq!(handoff.focused_caller(&ctx), Some(&expected));

                let mut caller_focused = false;
                let mut caller_activated = false;
                let mut enter_remained = true;
                let _ = ctx.run(
                    egui_macroquad::egui::RawInput {
                        events: vec![egui_macroquad::egui::Event::Key {
                            key: egui_macroquad::egui::Key::Enter,
                            physical_key: None,
                            pressed: true,
                            repeat: false,
                            modifiers: egui_macroquad::egui::Modifiers::NONE,
                        }],
                        ..Default::default()
                    },
                    |ctx| {
                        let frame =
                            draw_candidate_caller(ctx, &cockpit, layout, &expected, &mut handoff);
                        caller_focused = frame.focused;
                        caller_activated = frame.activated;
                        enter_remained =
                            ctx.input(|input| input.key_pressed(egui_macroquad::egui::Key::Enter));
                    },
                );

                assert!(
                    caller_focused,
                    "the registered caller must own focus on the frame after Return/Close"
                );
                assert!(caller_activated, "the focused caller must consume Enter");
                assert!(!enter_remained, "the caller-owned Enter must not leak");
                assert_eq!(windows.window_count(), modeless_windows_before);

                fixture.open_requested(&mut routes, &world).unwrap();
                assert!(routes.is_open(), "the live caller reopens the real route");
            }
        }
    }

    #[test]
    fn candidate_application_rebinds_cover_lifecycle_failure_deletion_and_reused_slots() {
        let mut fixture = CandidateRouteFixture::synthetic(
            CandidateFixtureMode::Context,
            ViewerFaction::Alliance,
        )
        .unwrap();
        let mut routes = fixture.new_controller();
        let mut original = GameWorld::default();
        let old_handle = original.systems.insert(system(0x9000_0007));

        for reason in [
            CandidateRebindReason::Startup,
            CandidateRebindReason::NewCampaign,
            CandidateRebindReason::SavedWorldLoad,
            CandidateRebindReason::ManualReload,
        ] {
            fixture
                .rebind(&mut routes, &original, ViewerFaction::Alliance, reason)
                .unwrap();
        }
        assert_eq!(routes.world_epoch(), 4);
        fixture.open_requested(&mut routes, &original).unwrap();
        assert_eq!(routes.selected_topic_id(), Some("original:60001"));

        let mut replacement = GameWorld::default();
        let replacement_handle = replacement.systems.insert(system(0x9100_0008));
        assert_eq!(old_handle, replacement_handle);
        fixture
            .rebind(
                &mut routes,
                &replacement,
                ViewerFaction::Alliance,
                CandidateRebindReason::SavedWorldLoad,
            )
            .unwrap();
        assert_eq!(routes.world_epoch(), 5);
        assert_eq!(routes.selected_topic_id(), None);

        let error = fixture
            .open_requested(&mut routes, &replacement)
            .unwrap_err();
        assert_eq!(error.code(), "candidate_context_unavailable");

        let error = routes
            .rebind_world(&replacement, ViewerFaction::Alliance, None)
            .unwrap_err();
        assert_eq!(error.code(), "missing_source_admission_facts");
        assert!(!routes.is_open());

        // A separately loaded world reuses the old numeric slot for DatId 7.
        // The fixture recomputes an exact typed source fact from that world
        // instead of retaining the old handle or the prior full DatId.
        let mut reused_world = GameWorld::default();
        let reused = reused_world.systems.insert(system(0x9200_0007));
        assert_eq!(reused, old_handle);
        fixture
            .rebind(
                &mut routes,
                &reused_world,
                ViewerFaction::Alliance,
                CandidateRebindReason::ManualReload,
            )
            .unwrap();
        fixture.open_requested(&mut routes, &reused_world).unwrap();
        assert_eq!(routes.selected_topic_id(), Some("original:60001"));
    }

    #[cfg(feature = "interface-test-fixtures")]
    #[test]
    fn candidate_fixture_codes_are_disjoint_from_every_accepted_fixture_namespace() {
        // Accepted E47 contract: interface_test_fixture.rs Scenario 38/39/40/41,
        // encoded as (scenario + 1) | (faction << 8). E21 pins 0x012a and
        // 0x022a for PackedEncyclopedia. These are reviewed identities, not an
        // inferred extension of E31's stale local scenario count.
        const E47_ACCEPTED_ENCYCLOPEDIA_CODES: [u32; 8] = [
            0x0127, 0x0227, // artwork
            0x0128, 0x0228, // message index shell
            0x0129, 0x0229, // encyclopedia index shell
            0x012a, 0x022a, // packed encyclopedia
        ];
        // Accepted E17 contract: encyclopedia_loose.rs BROWSER_PROBE_MAGIC
        // with seven status-aware transport cases.
        const E17_PROBE_CODES: [u32; 7] = [
            0xe117_0001,
            0xe117_0002,
            0xe117_0003,
            0xe117_0004,
            0xe117_0005,
            0xe117_0006,
            0xe117_0007,
        ];

        for (code, mode, viewer) in [
            (
                0xe131_0001,
                CandidateFixtureMode::Command,
                ViewerFaction::Alliance,
            ),
            (
                0xe131_0002,
                CandidateFixtureMode::Command,
                ViewerFaction::Empire,
            ),
            (
                0xe131_0003,
                CandidateFixtureMode::Context,
                ViewerFaction::Alliance,
            ),
            (
                0xe131_0004,
                CandidateFixtureMode::Context,
                ViewerFaction::Empire,
            ),
        ] {
            assert_eq!(super::candidate_fixture_code(mode, viewer), code);
            let request = decode_candidate_fixture(code).unwrap();
            assert_eq!(request.code, code);
            assert_eq!(request.mode, mode);
            assert_eq!(request.viewer, viewer);
            assert!(!accepted_e47_fixture_contract(code));
            assert!(!accepted_e17_probe_contract(code));
        }
        for code in E47_ACCEPTED_ENCYCLOPEDIA_CODES {
            assert!(accepted_e47_fixture_contract(code));
            assert!(decode_candidate_fixture(code).is_none());
        }
        for code in E17_PROBE_CODES {
            assert!(accepted_e17_probe_contract(code));
            assert!(decode_candidate_fixture(code).is_none());
        }
        assert!(decode_candidate_fixture(0).is_none());
        assert!(decode_candidate_fixture(42).is_none());
        assert!(decode_candidate_fixture(0x01_012a).is_none());
        assert!(decode_candidate_fixture(300).is_none());
    }

    #[cfg(feature = "interface-test-fixtures")]
    fn accepted_e47_fixture_contract(code: u32) -> bool {
        code >> 16 == 0 && matches!((code >> 8) & 0xff, 1 | 2) && matches!(code & 0xff, 1..=42)
    }

    #[cfg(feature = "interface-test-fixtures")]
    fn accepted_e17_probe_contract(code: u32) -> bool {
        code & 0xffff_0000 == 0xe117_0000 && matches!(code & 0xffff, 1..=7)
    }

    #[test]
    fn candidate_fixture_world_is_self_contained_and_keeps_the_context_identity_explicit() {
        let world = candidate_fixture_world();
        assert_eq!(world.systems.len(), 2);
        assert!(world
            .systems
            .values()
            .any(|system| system.dat_id == DatId::new(7)));
        assert!(world
            .systems
            .values()
            .any(|system| system.dat_id == DatId::new(8)));
    }
}
