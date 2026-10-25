---
title: "Roadmap"
description: "Audit-driven milestones M0-M5 and the v1.0 release, addons, and known technical debt"
category: "agent-docs"
created: 2026-03-11
updated: 2026-10-25
tags: [roadmap, planning, milestones, parity]
---

# Roadmap

<!-- interface-parity-status:start -->
Required interface families: 43. Complete: 2. Partial: 15. Failing: 26. Blocked: 0. Strictly accepted cells: 28/627.
The current CMD-02 GID catalog has 38 scenarios (29 baseline cells and nine additional native filter variants) and 152 faction/viewport executions. Its strict original-evidence and cross-browser gate remains open.
<!-- interface-parity-status:end -->

The delivery notes before “Audit-Driven Parity” are historical implementation
records. They do not establish current parity or release acceptance; the active
audit-driven milestones are the source of truth.

The bounded strategic-windowing backend sequence SW-00–SW-14 is reconciled at
the accepted r29 candidate; see the [SW-14 evidence record](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-25-strategic-windowing-acceptance.md).
This closes the backend implementation/evidence graph only; it does not itself
promote strict interface parity. Current strict family and cell counts are
reported by the audit snapshot below. Original-runtime comparisons,
native-Windows proof, A0 bitmap/layout/text coverage, and the missing System
Finder remain open.

PR #11 checkpoint (2026-09-14): [cockpit routing is corrected](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-cockpit-routing-correction.md) for commands `0x131` through `0x133`, both factions, and F1/F7. GID uses its authentic bottom control and can open and close its original menu. Encyclopedia was fail-closed at that checkpoint and is activated by E32 below. The [2026-09-26 Game Options review checkpoint](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-26-game-options-review.md) restores the shared bitmap surface and save/audio/display wiring; PRE-03 strict acceptance remains open.

P62 transports all 187 validated original EDATA images through native and
browser builds. A source cross-check corrects P63 as Message Index evidence.
P64 separately reconstructs both authentic Encyclopedia index shells, seven
category states, right-rail states, and native clipping in a bounded exact
browser gate. P65 initially inventories all 356 source records; P68 corrects
that set to 346 visible topics and ten excluded mission records. Seven category
filters, stable selection, and scrolling do not change saves
or exposing the unfinished production route. P66A strictly extracts 348 topic
texts and 191 image mappings, binds 346 complete topics per faction, and keeps
ten source-empty mission records explicit. W2 adds one immutable content
session with strict catalog/binding and original-BMP validation, atomic
replacement, rollback, deterministic logical and texture generations, and no
campaign-state ownership. W3 adds a pure presenter with exact session order,
both faction bindings, bounded navigation, explicit unavailable-source topics,
and immutable close/return origins across complete logical journeys. Browser
reader parity and production routing were then separated. W4 connects that presenter
to the authentic topic/index shell behind fixture compilation, owns one
generation-keyed selected-topic texture, and passes ten two-faction browser A1
cases at native and scaled viewports. W5A atomically publishes the canonical
catalog, sidecar, and exact referenced artwork through ORPK plus a
development-only loose generation, then proves native, packed-browser, and
loose-reader convergence on one 346/346-visible-topic W2 session fingerprint,
with ten source-hidden mission records excluded, while
preserving the four-request production startup budget. W6 adds opt-in native
faithful-HD selection at the immutable-session boundary: only manifest-reviewed,
source-matched, digest-valid exact-4x PNG output can replace a rendered view,
and every failure retains the original. Browser and strict acceptance remain
original-only. W7 adds deterministic, presence-aware native overlays with
atomic rollback and exact base restoration. E32 then pairs strict new-package
requirements with production cockpit and contextual routes, exact returns,
both-faction packaged journeys, old-pack unavailable compatibility, and native
foreground contextual evidence. E34 verifies the current-main assembled
candidate across workspace, package, native/packed/loose, replay, save/load,
production browser, mutation-identity, and isolation gates, leaving A0
comparison and strict `OBJ-01` cells to E35. E35 then corrects the faction-specific
surface origins, headings, production fonts, initial selection, hover tooltip,
and modal order. Its expanded final-package browser journey passes twelve
applicable lossless comparisons against the original executable under Wine;
the plan-defined source-unavailable state has no honest original A0 and passes
from its functional evidence. By explicit user decision on 2026-10-07, the
Wine evidence is sufficient for this delivery and all thirteen canonical
`OBJ-01` cells pass without claiming native-Windows parity. See
the [P66A evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-01-encyclopedia-topic-source-bindings.md),
[W2 evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-content-session.md),
[W3 evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-presenter.md),
[W4 evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-topic-surface.md),
[W5A evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-canonical-publication.md),
[W6 evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-hd-selection.md),
[W7 evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-07-encyclopedia-mod-overlays.md),
[E32 evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-07-encyclopedia-production-activation.md),
[E34 technical acceptance](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-07-encyclopedia-technical-acceptance.md),
and [E35 compatibility evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-07-encyclopedia-a0-compatibility.md).

The 2026-10-06 strategic-interface batch restores native cockpit accelerators,
the two-sector-window cap, Message Index interaction, Personnel and Troop
Finders, per-area production destinations, absolute completion days, fleet and
ship Rename, and en-route marks. Both-faction bounded browser gates pass. This
raises the planning estimate to approximately 25%, but accepts no strict cells:
the native Manufacturing and Build Selection windows, complete Status-window
coverage, A0 evidence, and complete state matrices remain open. The recovered production-window
contract is indexed in [the Manufacturing and Build Selection Ghidra note](../ghidra/notes/manufacturing-build-selection.md).

## Galaxy Viewer: historical implementation tranche delivered
*Interactive galaxy data viewer -- native + WASM*

Commits: `4e258bb` (scaffold) > `5905497` (dat-dumper + types) > `fc968ec` (renderer + WASM) > `22b61a6` (review fixes)

Delivered:
- Cargo workspace with 5 crates
- dat-dumper: 22/22 documented DAT files parsed, 3 structural patterns, round-trip byte validated
- rebellion-core: DatId newtype, 11 slotmap keys, dat/ enums, world/ structs, GameWorld root
- rebellion-data: DAT to GameWorld loader (sectors > systems > ships > fighters > characters)
- rebellion-render: macroquad galaxy map with pan/zoom/click-to-select, egui info panel + status bar
- WASM build: 2.9MB browser artifact via scripts/build-wasm.sh

## Living Galaxy: historical implementation tranche delivered
*Automated galaxy simulation — shipped 2026-03-13 by Knesset Athirat*

All 11 deliverables shipped. ~8.7K LOC, 105+ unit tests. See `docs/reports/2026-03-13-knesset-athirat-swarm-report.md`.

Delivered:
- TEXTSTRA.DLL string extraction via pelite (real entity names)
- GameClock with pause/1x/2x/4x speed controls
- 9 seed table loaders + fleet instantiation
- All 51/51 DAT parsers with round-trip validation (29 new)
- Manufacturing system with production queues
- Mission system (diplomacy, recruitment) -- ported from rebellion2's Mission.cs
- Event system (conditional triggers, chaining, deterministic rng)
- AI manager (officer assignment, production priority, fleet deployment)
- Message log UI (egui bottom panel, 6 color-coded categories)
- Mod loader: TOML manifests, RFC 7396 merge patch, semver, hot reload
- Main loop integration (all systems wired)

## War Room: historical implementation tranche delivered
*Full strategy game without combat*

Committed: `6d47a10` (integration wiring — fog/fleet/panels/encyclopedia/audio into main loop)

Delivered:
- Player faction selection + 5 UI panels (officers, fleets, manufacturing, missions)
- Fleet movement on galaxy map (diamond icons, dashed routes, ETA labels)
- Fog of war with monotonic reveal + advance intel at 50% transit
- Encyclopedia viewer with 4 tabs + BMP texture cache from EData/
- Audio system via quad-snd (CoreAudio/ALSA/WebAudio)
- Main loop integration: all draw calls + event hooks wired

## War Machine: historical implementation tranche delivered
*Major strategy systems implemented*

1. Space combat: auto-resolve first, then tactical 2D view
2. Ground combat: regiment engagement, orbital bombardment
3. Blockade mechanics, Death Star construction/destruction
4. Asymmetric victory conditions (control Coruscant; destroy and occupy the mobile Alliance HQ; preserve Standard leader requirements)
5. 3D tactical models: nano-banana-pro concept art → Hunyuan3D Pro/Meshy → Blender sprite sheets. Pipeline proven in World War Watcher (14 models). See `agent_docs/assets.md`.

Ghidra RE of REBEXE.EXE has extensive simulation coverage: the combat call
chain, bombardment formula, 111 GNPRTB parameters, and the C++ class hierarchy
are mapped. Interface RE remains active because 2,790 of 4,934 canonical text
exports are empty. Use the saved project and the
[interface RE ledger](../docs/qa/2026-09-10-interface-parity-audit/reverse-engineering-ledger.md)
for UI work.

## Full Parity: historical implementation tranche delivered
*Named delivery tranche shipped 2026-03-16 by Knesset Shapash; final parity remains open*

Delivered (v0.5.0):
- 4 scripted story chains (Luke Dagobah, Final Battle, Bounty Hunters, Jabba's Palace)
- Han Solo speed bonus, betrayal mechanics, decoy system, escape system
- 10 new Character fields promoted from DAT, mission state flags
- 6 new EventConditions + 6 new EventActions, 15 RE event ID constants
- All 9 mission types with world-mutating effects (sabotage, assassination, rescue, abduction, incite uprising)

## Mod Workshop: historical implementation tranche delivered
*Release candidate — Knesset Elat (v0.6.0)*

Delivered:
- Sensor-radius fog (detection field on CapitalShipClass)
- Captivity state tracking (is_captive, captured_by, capture_tick)
- Research pure advance() contract (caller applies level-ups)
- Save format v4 with mod metadata + FNV-1a hash, migration framework
- ModRuntime: discovery, enable/disable, structured errors, config persistence
- Mod Manager egui panel (discover, toggle, reload)
- ESCAPETB per-tick escape check wired into main loop

## Release: historical packaging tranche delivered
*Release packaging work by Knesset Hokhmah (v0.7.0); release acceptance remains open*

Delivered:
- 8 new story event chains (0x1e1, 0x1e5, 0x390-0x39A)
- 4 new EventCondition variants + 3 new EventAction variants
- Release packaging scripts (macOS + web)
- Example mod (star-destroyer-rebalance)
- GitHub Actions CI (test + WASM build)
- Vendored web/gl.js
- Release profile (LTO, strip, codegen-units=1)

Remaining:
- HD asset pack bulk execution
- Distribution: itch.io (web), Homebrew (macOS), GitHub Releases

## AI Overhaul: historical implementation tranche delivered
*v0.13.0 — AI balance, observability, autoresearch*

Delivered:
- Distance-based fleet transit (Euclidean, DISTANCE_SCALE=2, MIN_TRANSIT_TICKS=10)
- Starting force distribution (Empire 10 systems, Alliance 3)
- Garrison strength scoring (ships + troops + facilities)
- Galaxy-wide strategic bucketing (7 categories)
- Per-fleet attack targeting with deconfliction + battle repeat penalty
- Two-pass deployment (original parity: primary assign + secondary redistribute)
- Faction-asymmetric doctrine (Empire→HQ strike, Alliance→guerrilla)
- Role-based character AI (Jedi reserved, diplomats prioritized)
- Production doctrine (capital ships > fighters > yards)
- Build completion wiring (manufactured items added to GameWorld)
- HQ garrison defense (first fleet defends HQ)
- ControlKind state machine (Uncontrolled, Controlled, Contested, Uprising)
- Human-readable JSONL event payloads (system/character names)
- Enhanced --summary (galaxy control, transit state, combat diagnostics)
- --jsonl streaming flag for stdout
- REPL commands: systems, transit, events N
- eval_game_quality.py (8 sub-metrics, degenerate detection)
- autoresearch_loop.py (Karpathy self-improvement loop)
- AI parity tracker (maps 6 original functions to our implementation)
- Death Star fleet exemption (always targets enemy HQ)

Campaign results: VICTORY at tick 1188, 211 battles, eval score 0.59

## UI Rebuild — IN PROGRESS
*v0.16.0 — Player-facing UI rebuild following `docs/plans/archive/2026-03-22-001-feat-complete-game-ui-rebuild-plan.md`*

### Completed (2026-03-23/24)
- Phase 1.1/1.2 prototype: custom text main menu and separate setup screen. These are now superseded by the authentic cockpit gate below.
- Phase 1.4: TEXTSTRA.DLL string extraction (1,347 entity names)
- Phase 2.4: Star Wars egui theme (dark space + gold/amber accents, Liberation Sans)
- Phase 3.3: System Context Menu (right-click: control, popularity, garrison, actions)
- Phase 3.4: Fleet Context Menu (right-click: composition, commander, transit, actions)
- Phase 4.1: Research Panel (3 tech tree tabs, progress bars, character assignment)
- Phase 4.2: Jedi Training Panel (Force roster, XP bars, training controls)
- Phase 4.3: Fleet Composition Editor (assign/remove officers, merge fleets)
- Phase 4.4: Bombardment Targeting Panel (fleet selection, damage forecast, fire button)
- Phase 4.5: Death Star Control Panel (construction progress, superlaser targeting, movement)
- Phase 4.6: Loyalty & Uprising Dashboard (per-system danger, betrayal risk)
- Phase 4.7: Expanded System Detail Panel (fleets, troops, facilities, popularity bars)
- Phase 4.8: Character Detail View (skills, Force progression, location, assignment)

### Completed (Knesset Demiurge, 2026-03-24)
- Phase 2.1-2.2: Cockpit Frame + Control Buttons (BmpCache, faction chrome, 9 buttons)
- Phase 3.1: Facility Status Icons (colored squares by type, zoom-scaled)
- Phase 3.2: Sector Boundaries (Graham scan convex hull, 3 SectorGroup colors)
- Phase 3.5: Blockade Visual Indicators (red ring + fill on blockaded systems)
- Phase 5: Tactical Combat View (2D arena, phased combat, targeting, retreat, ground)
- Phase 6: Audio Integration (quad-snd soundtrack, 285 voice lines, SFX, context music)
- Phase 7: Entity Portraits (GOKRES.DLL 61 portraits + 57 ship status views)

### Remaining UI — HD Visual Polish (IN PROGRESS)

- [x] Reconstruct the browser's original 640x480 shuttle-cockpit composition, exact pointer regions, visible settings, direct faction start, and single-loop menu music. Browser review R3 passed 11/11 after finding two R2 defects ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-cockpit.md)).
- [x] Complete the cockpit endpoints and lifecycle: Credits, explicit M4 multiplayer status, gain/mute, return-to-menu, clean second-campaign reset, exact `MDATA.300` menu cue, and original `COMMON.DLL` button sounds. All 500 tests pass; completion evidence is recorded in [F-016C](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-completion.md). Native interactive visual acceptance and semantic canvas accessibility remain release-hardening work.
- [x] Expose all 14 authentic controls through clipped browser semantics while preserving the bitmap-only presentation, exact Rust action/SFX paths, and four-request startup. All 502 tests pass; independent review verified 84/84 focus transitions and all browser gates in [F-016D](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-semantics.md). The then-open native visual gate closes in F-016E below.
- [x] Complete F-016E and P03: add a clearly documented, non-original 30×22 music-only cockpit control using Fable-reviewed Jiff Gorda/SWG Project Thorn cues; preserve SFX while muted; complete native visual acceptance; and pass 10/10 browser checks across four viewports, exact hit geometry, semantics, audio, navigation, and error gates. All 504 tests pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-music-toggle.md)).

The April 2026 model shootout remains comparison evidence, but its universal
UltraSharp conclusion is superseded by the
[faithful-HD plan](../docs/plans/2026-09-10-faithful-hd-pipeline/). Original
pixels are now the default parity profile. Optional HD uses reviewed manifests
and family-specific routing.

```bash
uv run scripts/faithful_hd_pipeline.py generate # Deterministic candidate generation
uv run scripts/faithful_hd_pipeline.py verify   # Hash and reconstruction gates
python3 scripts/model-shootout.py --html-only    # Regenerate comparison at data/hd/shootout/comparison.html
```

| Item | Status |
|------|--------|
| Historical model shootout (8 models × 20 BMPs) | COMPLETE; preference evidence only |
| Explicit profiles and manifest foundation | COMPLETE under P45A; [evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-faithful-hd-foundation.md) |
| Complete-family learned-model comparison | PENDING under P45B |
| EData encyclopedia images (187 owned originals) | TRANSPORT AND EXACT DISPLAY COMPLETE; AUTHENTIC WINDOWS OPEN |

### Knesset Kothar wa Khasis. Historical implementation (2026-04-07)
*U2 plus the first C1 attempt from Knesset Resheph. P44 supersedes the advisor result.*

Delivered:
- **U2 — Native video playback for decoded WebM cutscenes**: new `crates/rebellion-render/src/video_player.rs` with `VideoPlayer::open/advance/current_frame/is_finished/stop`. Runtime path is PNG frame sequences + WAV sidecars (no ffmpeg/libvpx/gstreamer runtime deps). `scripts/decode-cutscenes.sh` is a one-time local ffmpeg decode that produces `assets/references/cutscene-frames/<name>/frame-*.png` + `metadata.json` + `<name>.wav` (ignored by git). `GameMode::Cutscene` added to `rebellion-app/src/main.rs`, plays `000.webm` on startup before MainMenu and `201.webm`/`202.webm` on victory/defeat. ESC/SPACE skip handling. Graceful `VideoError::NotDecoded` path logs a skip message and proceeds when decoded assets are missing. WASM build unaffected via a cfg-gated no-op stub. New dep: `quad-snd` on rebellion-render (player owns its own audio context).
- **C1 first attempt**: a cascading BIN parser and sorted-frame fallback were added, but later interface review showed that this did not prove original motion or even transport the custom type-302 pixels. P44 replaces the visible idle path with the exact indexed anchors and cumulative type-302 deltas for both factions. `RE-ADV-02` still owns authored action, cadence, preemption, and voice mapping.

Known gaps left for future work:
- Only ~24% of advisor BINs (183 of ~752 per faction) match the simple `u16 count + u16 ids` shape. The other ~76% declare inconsistent lengths and are silently skipped — the game almost certainly uses one or more additional header variants (timing bytes, loop counts, control flags) that have not been decoded. The 24% that do parse now drive authored sequence length/variation; the rest fall through to the legacy path.
- Victory/defeat cutscenes return directly to `MainMenu` rather than chaining through the existing egui victory modal.
- Cutscene audio plays at fixed volume through the player's own quad-snd context rather than through `AudioVolumeState`.
- Task #132 report in `docs/plans/archive/2026-04-03-001-feat-knesset-resheph-final-sprint-plan.md` tracks these.

### Game Seeding Parity — CRITICAL GAP (~12% parity)

*Discovered 2026-03-24 via audit against TheArchitect2018/Deep-Dive-into-SW-Rebellion-PC-Game-Internals/initial_game_seeding_logic*

ExecPlan: `docs/plans/completed/2026-03-24-003-game-seeding-parity-execplan.md`

**What works**: 9 seed table DAT parsing + dispatch, Coruscant fleet/garrison/facilities, Yavin garrison, army table distribution.

**Fixed (M1-M4, 2026-03-24):**
1. ~~Character stat rolling + placement~~ — DONE. SkillPair rolled to concrete stats, named characters placed (Luke/Leia/Han at Yavin, Vader/Palpatine at Coruscant, Mon Mothma at random HQ). 8 tests.
2. ~~Alliance HQ randomization~~ — DONE. Random rim system selected as Rebel HQ, deterministic with RNG seed.
3. ~~Fleet distribution model~~ — DONE. 3-system model (Coruscant/Yavin/Rebel HQ) replaces proximity spread.
4. ~~Difficulty/galaxy size affecting seeds~~ — DONE. SeedOptions threaded through to apply_seeds.
5. ~~Energy/raw materials fields~~ — DONE. System struct extended with is_populated, total_energy, raw_materials.

**Fixed (M5-M8, 2026-03-25):**
6. ~~Support/popularity initialization~~ — DONE. Control buckets (core strong/weak/neutral), support 20-100 for controlled, ~41-59 for neutral.
7. ~~Procedural galaxy generation~~ — DONE. Core/rim bucket percentages from SDPRTB 7680/7681. Energy/raw materials from GNPRTB 7721-7727. Facility generation from SYFCCRTB/SYFCRMTB.
8. ~~Maintenance-budget common unit seeding~~ — DONE. Budget from SDPRTB 5168/5169/5170, bundle rolling from CMUNALTB/CMUNEMTB, low-support garrison pass.

**Seeding parity: COMPLETE.** Verified against TheArchitect2018 seed.js — rim exclusion from maintenance budget is parity-correct.

### Combat Review Fixes — DONE (2026-03-25)

All 10 review findings from Knesset Ma'at resolved:
- Shield absorption double-rounding → single i64 integer division
- `recall_fighters` grounded squad loss → `originally_launched` parameter
- Hardcoded `difficulty: 2` → `GameWorld.difficulty_index` threaded everywhere (space, ground, bombardment)
- TroopClassDef fallback warning → `eprintln!` on missing class
- Tactical fighter shield bypass → shield absorption added
- Tactical weapon type from hull_max → actual `CapitalShipClass` weapon stats
- Dead code (`ButtonSprite::disabled`, `atk_class_defense`) → removed
- Deprecated API (`Frame::none()`) → `Frame::NONE`

This tranche corrected the listed combat defects. Formula, tactical/automatic,
and full combat acceptance remain open under P23–P26.

## Knesset Resheph: historical implementation tranche delivered
*Final parity sprint — 10 delivered tasks across combat, AI, WASM, UI, and eval*

Plan: `docs/plans/archive/2026-04-03-001-feat-knesset-resheph-final-sprint-plan.md`

Delivered:
- Combat: per-weapon-type damage in `phase_weapon_fire()` now multiplies turbolaser, ion cannon, and laser cannon arcs by their class-specific `*_attack_strength` scalars using i64 math throughout. Zero attack strength falls back to raw arc count. 3 tests added.
- Death Star docs: replaced "placeholder" / "not yet decompiled" wording with "best available approximation" after exhaustive GNPRTB search confirmed the current values are not parameterized.
- AI parity: `FUN_00506ea0` reads per-faction deployment budgets at `+0xc4` (Alliance) and `+0xc8` (Empire); their values are not recovered, and `AiConfig` carries no budget.
- AI parity: dispatch validation advanced from 4/18 to 10/18 decoded gates via new `can_dispatch_to_system()` and `can_dispatch_fleet()` helpers, including strength comparison, loyalty gating, faction checks, and empty-fleet rejection.
- AI behavior: `evaluate_uprising_prevention()` now sends diplomats to low-support controlled systems before they flip.
- AI behavior: `evaluate_ds_escort()` now routes the nearest available fleet to the Death Star as an escort.
- WASM save/load: `rebellion-data` replaced browser stubs with a real `localStorage` backend using manual base64 encode/decode, plus save/load/list/delete support and separate metadata keys for fast `list_saves()`.
- WASM UI assets: `BmpCache` gained cfg-gated path rebasing for browser texture loads and a named DLL resource ID catalog grouped by source (`common`, `strategy`, `tactical`, `gokres`).
- WASM audio: `audio_base_path()` and `AUDIO_PREFIX` now normalize browser audio asset paths.
- Eval parity: `scripts/eval_parity.py` was rewritten into a golden-value oracle backed by `scripts/golden_values.json`, covering all mapped 111 GNPRTB bindings plus combat, economy, research, AI, movement, and victory constants with pass/fail reporting.

## Knesset Shamash-Bet Dabora 3: historical implementation tranche delivered
*Story events + betrayal telemetry sprint — 8 R-tasks delivered*

Plan: `docs/plans/archive/2026-04-08-001-feat-knesset-shamash-bet-story-events-cutscene-plan.md`

Delivered:
- R1: EVT_HAN_RESCUE (0x200) — telemetry twin of 0x383, fires same-tick via EventFired chaining. Silent (no actions). +19 lines.
- R2: EVT_JABBA_PRISONERS (0x231) — consolidator for palace captures. Three OR-branch variants (one per capture ID: 0x385 Leia, 0x387 Chewie, 0x399 Luke). Self-guard via EventNotFired ensures exactly one fires per game. +36 lines.
- R3: EVT_HAN_PERMANENT_FREEZE (0x39B) — 5-stage carbonite escape countdown (FAIL_1 through FAIL_5, ticks 145→205) using existing primitives only (EventFired + TickAtLeast + EventNotFired + CharacterIsCaptive). Terminal state when countdown completes without rescue or self-escape. 7 new constants in events.rs. +81 lines.
- R4: Final Battle heritage gate — single 0x220 EVT_FINAL_BATTLE (no event split). 0x396 action flips `heritage_known` via SetHeritageKnown. Nothing reads the flag; the story chains' port is `or-port-story-research-c80`.
- R5: Bounty Hunters real spawn — EVT_BOUNTY_ATTACK (0x212) now includes SpawnSpecialForce { at_character: han } + CharacterAssignedToFleet precondition (SF-#7). Test helper assign_han_to_fleet() added; 10 existing tests updated with fleet setup.
- R9: EVT_TRAITOR_REVEALED (0x361) — emitted BEFORE faction flip in integrator with original_faction payload.
- R10: EVT_SIDE_CHANGE (0x386) — emitted AFTER faction flip with DI-H2-compliant payload (char_name, not CharacterKey).
- R13: Stale "notifications" removed from section comment.
- 446 tests (347 core + 50 data + 46 render + 3 doc), zero warnings.

## Knesset Tammuz: historical implementation tranche delivered
*Full parity sprint — 7 phases across combat, telemetry, cutscenes, AI, and advisor*

Plan: `docs/plans/completed/2026-04-12-001-feat-knesset-tammuz-full-parity-sprint-plan.md`

Delivered:
- Phase 1: SpecialForceSpawned arena wiring (A2 — creates SpecialForceUnit in world). A1/A3-A6 already shipped in prior sprints.
- Phase 2: Mission telemetry emissions — EVT_INFORMANT_INTEL (R6), EVT_SABOTEUR_DETECTED (R7), EVT_CHARACTER_HEALTH (R8), EVT_CHARACTER_KILLED from assassination (R11). Death Star kill path was already done.
- Phase 3: Emperor Palpatine 1.5× combat modifier (R12) — scales weapon fire damage when Emperor is co-located.
- Phase 4: Cutscene state machine — `GameMode::Cutscene { kind: CutsceneKind }` struct variant with Intro/Victory/Defeat/Story(u32) kinds. Post-cutscene transitions (Story→Galaxy, others→MainMenu). `GameMode::VictoryModal` with egui overlay. Save lock-out during cutscenes/victory. 8 story cutscene triggers (101–108) mapped to event IDs (0x221, 0x210, 0x212, 0x383, 0x220, 0x393, 0x396, 0x397).
- Phase 5: Advisor BIN v2 cascading decoder — parse rate 12%→99%. Four binary formats discovered (v1 explicit, v2 sequential range, v3 BMP-mapped range, v4 BMP single). ~42% of sequences now use direct BMP resource ID lookup instead of modulo. 11 new tests.
- Phase 6: AI parity closeout — dispatch validators 10/18→15/18, troop deployment with frontline awareness, Death Star multi-target selection + retreat logic, reconnaissance mission dispatch. 7 new tests.
- Phase 7: Code hygiene — 5 stale `controlling_faction` doc comments fixed, unused imports removed.
- 465 tests (355 core + 50 data + 57 render + 3 doc), zero failures.

## AI Parity Status (as of 2026-04-12)

Based on 3-agent review + 23-function GhidraMCP session (2026-03-23) + TheArchitect2018 wiki cross-reference + Knesset Tammuz D1-D5. See `agent_docs/systems/ai-parity-tracker.md` for full matrix.

### Core Pipeline (6 Functions)

| # | Original | Status | Gap |
|---|----------|--------|-----|
| 1 | FUN_00519d00 Galaxy evaluation | DONE | 7 buckets + control_ratio + aggression scaling |
| 2 | FUN_00537180 Primary deployment | AUGMENTED | Per-fleet scoring (4-factor). Original was per-system with capacity check only. **Our model is superior.** |
| 3 | FUN_005385f0 Secondary deployment | AUGMENTED | Original uses FUN_0052e970 (capacity check) + FUN_00506ea0 (faction evaluator). Our aggression model is more nuanced. |
| 4 | FUN_00502020 Garrison strength | DONE | Ships + troops + facilities |
| 5 | FUN_00508250 Dispatch validation | DONE | **All 18 checks resolved.** 15 directly ported, #2/#3/#4 CLOSED — per-cycle caps replace C++ allocation budget tracking. |
| 6 | FUN_00520580 Movement orders | DONE | **Decoded: 2-field struct setter** (not transit calc). 9 lines. |

### Remaining Gaps (P2) — ALL CLOSED (2026-04-14)

5. **Defense facility construction** — CLOSED. FUN_00508660 is an entity-type dispatcher, not a priority function. Current order is FAITHFUL.
7. **AI resource rebalancing** — CLOSED. FUN_00559ee0 looks up RESRCTB (table 0x2b) with a random draw; the resource perturbation is minor. Negligible gameplay impact.

---

## Roadmap: AI Parity Completion (v0.14.0)

### Phase A: Config Externalization — COMPLETE
*Unblocks autoresearch. ~160 LOC.*

- [x] Created `crates/rebellion-core/src/tuning.rs` — `GameConfig` with 4 sub-configs (`AiConfig`, `MovementConfig`, `ProductionConfig`, `ScoringConfig`), 16 tunable parameters
- [x] Externalized all AI/movement/production constants — threaded `&GameConfig` through `AISystem::advance()`, `score_attack_target()`, `fleet_transit_ticks_with_config()`
- [x] Added `--config <path>` to playtest binary (JSON with `#[serde(default)]`)
- [x] Default config at `configs/autoresearch/default.json`
- [x] Parity/augmentation split documented per-field in tuning.rs
- [x] Updated `autoresearch_loop.py` to write and pass config files

### Phase B: Research Dispatch (P0 Gap #1) — COMPLETE
*AI tech tree progression. ~60 LOC.*

- [x] Added `evaluate_research()` to AISystem — assigns idle characters to Ship/Troop/Facility trees
- [x] Characters matched by primary skill (ship_design ≥ 30, troop_training ≥ 30, facility_design ≥ 30)
- [x] `AIAction::DispatchResearch` variant + telemetry event
- [x] Wired into simulation tick (both factions) and interactive app

### Phase C: Ratio-Based Galaxy Evaluation (P0 Gap #2) — COMPLETE
*Aggression scaling. ~30 LOC.*

- [x] Ported FUN_0053e190: `control_ratio` and `aggression` fields on GalaxyState
- [x] Galaxy control % → aggression: 10% → 0.18, 50% → 0.5, 90% → 0.82
- [x] `max_fronts` scaled by aggression (weak faction opens fewer attack fronts)
- [x] Pass 2 behavior: high aggression → pile onto attacks; low → reinforce

### Phase D: Proportional Redistribution (P0 Gap #3) — COMPLETE
*Replace Pass 2. ~25 LOC.*

- [x] Defensive reinforcement distributes across ALL undefended systems (not just first)
- [x] Round-robin by fewest incoming reinforcements (even distribution)
- [x] Falls back to all controlled systems when no undefended remain

### Phase E: Combat Spread — COMPLETE
*Territory expansion via diplomacy. ~20 LOC.*

- [x] Diplomacy-to-control transition: popularity ≥ 0.6 with ≥ 0.1 lead flips ControlKind
- [x] Existing diplomacy targeting already covers neutral/enemy systems
- [x] More controlled territory → more attack targets → diverse battle locations

### Phase F: Autoresearch Execution — COMPLETE
*Run 001: 20 iterations × 3 seeds.*

- [x] Release binary built (1.5MB, LTO)
- [x] Run 20 iterations × 3 seeds × 5000 ticks
- [x] Baseline: 0.5449 → Final: 0.6483 (+19% improvement)
- [x] 2 accepted mutations: distance_scale 2→3, mfg_target 50→55
- [x] Results at `autoresearch/run-001/`, best config at `configs/autoresearch/best_game_config.json`
- [x] Finding: parameter space is narrow — most mutations cause degenerate games

---
Where the project stands is in the [full-functionality audit](../docs/qa/2026-09-08-full-functionality-audit/README.md):
its feature ledger and evidence files, and in `audit-report.md` beside it
"Where work left off", the evidence contract and the definition of done. Its
sister [original-interface audit](../docs/qa/2026-09-10-interface-parity-audit/README.md)
owns visible surface, bitmap, geometry, hotspot, navigation, animation and
audio acceptance. Their JSON ledgers hold the finding, feature and cell IDs
cited below; each ticked item names its evidence, and each open item keeps its
acceptance criteria.

Delivery notes from March and April 2026 are in
[archive/roadmap-history-2026-03-to-04.md](../archive/roadmap-history-2026-03-to-04.md);
they establish no current parity.
## Audit-Driven Parity, Browser, and Multiplayer Roadmap

### M0: Truth and Critical Integration (1 week)

- [x] F-001 Save, Load, Delete and the main-menu save picker: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-save-delete.md)).
- [ ] Surface browser corruption/quota errors and move production saves to asynchronous IndexedDB.
- [x] F-014A native HD root, `TROOPSD.DAT` prefetch and the deterministic `runtime.orpk`: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-runtime-pack.md)).
- [x] F-010B fleet miniatures for all 38 fighter and capital-ship records: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-fleet-miniatures.md)).
- [x] F-011A save-state fingerprint: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-state-fingerprints.md)).
- [x] F-011B1 save continuation envelope: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-state-continuation.md)).
- [x] F-016A cockpit command-to-animation table: pass ([reference](main-menu-parity.md); [evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-cockpit.md)).
- [x] F-016B cockpit faction, difficulty, galaxy size and game type persist in saves: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-game-setup-propagation.md)).
- [x] F-016C / P03-P04 Credits, Multiplayer, audio controls and campaign reset: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-completion.md)).
- [x] F-016D main-menu browser semantics: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-semantics.md)).
- [x] F-016E / P03 final presentation and the music-only extension: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-main-menu-music-toggle.md)).
- [ ] Make a clean packaged browser artifact boot in CI.
- [x] README parity claims link to current acceptance evidence: done (`README.md`).
- [x] F-011B2 replay format v1: pass ([contract](deterministic-replay.md); [evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-contract.md)).
- [x] F-011B3 replay recorder and executor: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-execution.md)).
- [x] F-011B4 native/WASM replay equivalence: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-replay-wasm-equivalence.md)).

### M1: Simulation Correctness and Determinism (2–3 weeks)

- [x] F-007A in-transit fleet redispatch prevented: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-fleet-redispatch.md)).
- [x] F-007B explicit fleet position, no production onto fleets in transit: pass; its arrival merge is retired as unsourced (`ghidra/notes/fleet-join-split.md`) ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-fleet-position-consolidation.md)).
- [ ] Wire ordinary player fleet dispatch through the validated authoritative
  departure path and pass independent bitmap acceptance for both factions
  ([F-007C evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-09-player-fleet-dispatch.md)).
  Reopened 2026-10-01: the egui context menu that set the destination lost
  its caller in 23d15da, so the player cannot move a fleet, and the fleet
  panel's destination banner and Dispatch button are unreachable. The
  original Move order (below) restores it: menu Move, Confirmed Move, and the
  system window drag pass a two-sided browser gate (2026-10-02), and the
  blockade bit follows the original. Regiments load through the Fleet window
  (2026-10-04, below). A native check remains.
- [x] F-007D one bounded engagement per system: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-system-combat-resolution.md)).
- [ ] Extend stable ordering and versioned random streams through five 5,000-tick campaign seeds.
- [x] F-007E logistics checkpoint (AI production control, transit, HQ defender, troop classes, repair in saves): pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-ai-campaign-logistics.md)).
- [x] F-007E conquest checkpoint (transport, landing, surface battles, occupation): pass; capture stays open below ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-troop-transport-occupation.md)).
- [ ] Add player troop selection to the bitmap fleet workflow with live
  capacity, authoritative embarkation, transit, and automatic landing
  ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-player-troop-dispatch.md)).
  Reopened 2026-10-02: the fleet panel's troop picker lost its caller in
  23d15da and was deleted in F-007C phase 6a, so no player path loads
  regiments onto a fleet. Restored 2026-10-04 through the original order: a
  regiment's Move released on the Fleet window's fleet, opened from the
  sector window's fleet icon; the `fleet-window.mjs` gate passes both sides
  (load, hold, move, land, full-fleet refusal). A native check remains.
- [x] F-007E cited campaign-history baseline: done ([reference](../docs/reference/campaign-history/)).
- [x] F-007E victory semantics (Coruscant, mobile HQ, Standard conjunctions, nonterminal Death Star loss): pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-victory-contract.md)).
- [ ] Replace deterministic mass capture with validated capture/evasion
  behavior.
- [ ] Establish faction liveness and the full cited campaign loop, including
  production, diplomacy, recruitment, intelligence, research, blockade
  follow-up, uprisings, bombardment, HQ relocation, and principal missions.
  Do not require symmetric attack counts from an asymmetric game.
- [ ] Enable AI Death Star construction/fire/cleanup and repair parity oracles.
  `ghidra/notes/death-star-strategic.md` recovers the original: an ordinary
  shipyard build of CAPSHPSD record 136, one per system; fire is order
  `0x223` (gate `FUN_00582440`, no side or owner test); the destroy cascade
  leaves fleets standing; sabotage has no build-delay rule (F-017). The
  orders, the gate and the per-hit run are ported
  (`docs/plans/2026-10-16-feat-orbital-strike-and-bombardment.md`). Open:
  `or-port-destroy-system-cascade-imrv` (P1; an Alliance-player native
  Destroy System and the browser probes) and whether the AI fires it,
  `or-q-ai-death-star-use-z9ow`.
- [ ] Wire the recovered rules the 2026-09-25 test-pruning pass found uncalled:
  research gating of buildable ships (F-018), the decoy roll and the SUBDMSTB
  Subdue success check (F-019; both now run, and its remaining phases are
  tracked below), mod dependency errors (F-020), and blockade troop
  destruction (F-021; tracking now persists in save v15, browser pass
  pending).
- [ ] Correct or re-source the code behaviors the 2026-09-26 citation audit
  found without a binary source: the economy's KDY production modifier
  (F-023) and the Emperor's 1.5x battle bonus (F-024). Death Star detection
  keys on capital-ship family `0x18` as the original does; its shield's
  battle role waits on the TAC-05 cells (F-025,
  `ghidra/notes/death-star-strategic.md`). The revolt lifecycle, uprising
  incident, and disaster now follow recovered code (F-026, browser pass
  pending).
- [ ] Move the remaining probability-table consumers to the recovered step
  lookup (F-028) and the informant and resource incidents to their recovered
  timers (F-029).
- [x] F-030 en-route delivery of manufactured objects: ported (save v16); browser pass pending (audit F-030).
- [ ] Port the mission phases, decoys, and per-member success roll (F-019).
  The rules are in `ghidra/notes/decoy-roll.md`, `mission-lifecycle.md`,
  `ai-mission-planning.md`, and `mission-dialog.md`. Phases 0-6 have landed:
  the lifecycle, detection, the per-member roll, the recruit pool, sabotage
  targets, legality, the AI team and decoy selectors, the original mission
  dialog, dispatch telemetry, and the dialog's browser gate. Phase 7 ports the
  original entry (right click, Mission, targeting cursor, target), removes the
  invented entry points, and traces refusals to the advisor; the entry passes
  its browser gate for both sides. A native check remains.
- [ ] Port the Move and Confirmed Move orders (`0x201`/`0x202`): the pop-up
  menu items, targeting with the `+0x70` hit test, and the list drag
  (`FUN_006083c0` notification `0x29a`). This restores player fleet movement
  (F-007C). Retire, the Command submenu, and Status share the menu. Phases
  0-8 have landed and pass the `fleet-move.mjs` and `fleet-window.mjs` gates
  for both sides; a native check remains.
- [x] F-007C system blockade bit follows `FUN_0050b8e0`: ported (`ghidra/notes/blockade-bit.md`).
- [x] F-007E regiment loading through the Fleet window: ported, `fleet-window.mjs` passes both sides; native check under F-007C (`ghidra/notes/fleet-window.md`).
- [x] F-007C sector window quadrant icons and the System, System Defenses and Missions windows: ported, `sector-quadrants.mjs` passes both sides; native check open (`ghidra/notes/sector-quadrants.md`).
- [x] F-007E unloading a regiment by hand and regiment travel: ported, `fleet-window.mjs` passes both sides; native check open (`ghidra/notes/regiment-unload.md`).
- [x] F-007C joining and splitting fleets: ported, `fleet-window.mjs` passes both sides; native check open (`ghidra/notes/fleet-join-split.md`).
- [x] F-007C Fleet Finder: ported, `fleet-finder.mjs` passes 18/18 on both sides; transport-limited native journeys open ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-10-05-f007c-native-interface-acceptance.md)).
- [ ] Balance one-ship fleets in the AI. With the arrival merge retired, the
  AI's own fleets no longer combine where they meet: in seed 42's 1,500-tick
  dual-AI run the fleet count at its six snapshots runs 5, 6, 11, 11, 17, 17
  where it ran 4, 4, 9, 9, 15, 15. Teach the AI to avoid lone-ship fleets,
  or join them by condition, and measure against that run (Tom,
  2026-10-05).
- [ ] Source or retire the port's automatic landing (F-007E: cargo lands
  when its side alone orbits a system). The unloading trace finds no
  original path that lands cargo without an order
  (`ghidra/notes/regiment-unload.md`, "What happens to the rest of the
  cargo"); it stays until traced (Tom, 2026-10-04).
- [ ] Port targeting of characters and objects (`+0x68` hit test, the walk up
  from a team member) so the dialog offers Rescue, Assassination, and
  Abduction (F-019).
- [ ] Trace the galaxy view's right click on a system or fleet, then port
  what it opens.
- [ ] Sort a refused order's status by its first word as the advisor does
  (P34; `FUN_004c3bb0`, `FUN_004c19f0`; `ghidra/notes/mission-dialog.md`,
  "Refusal"). Every refusal (missions, fleet, regiment and squadron moves)
  already reaches `AdvisorState::refuse` and files no text; any status but a
  repeat takes the generic reaction.
- [ ] Add the original mission kinds the port lacks: Reconnaissance, Palace,
  Bounty, Jedi Training, and the leisure and movement kinds (F-040).
- [x] F-031 provenance gate on uncited simulation rules: in place, and its baseline may only shrink (`tools/provenance-scan`).
- [x] F-032 fleet transit time and bombardment: ported from `FUN_0055d8c0` and orders `0x220`-`0x223`; fog open (audit F-032).
- [ ] Replace the invented rules found by the invention review, one system at
  a time: fog (F-032), strategic combat (F-033), missions
  (F-034), manufacturing and repair (F-035), AI (F-036), economy and research
  (F-037), character timers and story events (F-038), and false notes (F-039).
- [ ] Pass five reproducibly configured 5,000-tick seeds: transit ≤10%, orders
  ≤1.5× arrivals, and fleet arena ≤3× initial. Record 50–400 engagements, eight
  systems, busiest-system ≤40%, and 10% minority-faction attack share as
  diagnostics until original telemetry or calibrated playtests justify hard
  lower bounds.

Parameter autoresearch remains paused until this milestone closes; otherwise it
would tune around known simulation feedback defects.

### M2: One Authoritative Game Engine (3 weeks)

- [ ] Route the app and playtest through one simulation tick API and event sink.
- [ ] Move the panel-action handling out of `rebellion-app/src/main.rs` into
  modules native tests reach; every mutation run leaves survivors there.
- [ ] Make automatic, tactical, and ground combat share resumable core state and calculations. Unplayed battles run the tactical engine headless; the Planetary Assault (`assault.rs`) resolves in one pass with nothing to resume.
- [ ] Construct the victory modal and unify win/loss transitions.
- [ ] Remove `AdvanceTicks` or make it execute the real simulation.
- [ ] Prove identical fingerprints for the same seed and command stream across app/playtest, native/WASM, and auto/tactical paths.

### M3: Exceptional Browser Runtime (3 weeks)

- [x] F-014A deterministic `runtime.orpk` startup in four requests for both factions: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-runtime-pack.md)).
- [ ] Complete `UIP-T01` through `UIP-T05`: replace the synthetic strategy,
  object-window, report, and tactical surfaces with the original bitmap-driven
  compositions for both factions. No invented visible control may satisfy a
  parity cell.
- [ ] Bring the ground assault to parity (`UIP-F-010`, `TAC-08`,
  `RE-GND-01`).
  - The original report-only flow is ported (manual pp. 119-121, figs. 2.58,
    3.66 and 3.67; `ghidra/notes/ground-assault.md`): the fleet's Planetary
    Assault order (`assault::order_enabled`, greyed out against two
    planetary shields), immediate resolution (`assault::resolve`), and the
    Operation Report window (`rebellion-render/src/operation_report.rs`),
    filed to both sides. The invented ground-combat screen is gone.
  - Cover both factions, win and loss, casualties, occupation and the return
    to the map.
  - Close the eight `TAC-08` cells against A0 captures of the original.
    All eight references are captured (an A0-only family outside the
    106-cell tactical denominator; saves `empire-day53-yaga-orbit` and
    `empire-day128-assault-fails-selonia`).

Strategic interface checkpoints. Each is a bounded checkpoint; the cells it
leaves open belong to the family named.

- [x] P44 / `RE-ADV-01` advisor idle frames in native and WASM: pass; actions and voice under `RE-ADV-02` ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-10-authentic-droid-advisors.md)).
- [x] P46A / `RE-STR-01` strategic shell canvas: checkpoint; `UIP-T01` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-11-strategic-shell-canvas.md)).
- [x] P46B / `RE-STR-02` cockpit command bitmaps and input: checkpoint; A0 and `UIP-T01` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-11-strategic-command-controls.md)).
- [x] `UIP-B01` sector and detailed-system shells and the rail: checkpoint; `CMD-03` and `CMD-04` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-11-strategic-window-navigation.md)).
- [x] Strategic-window routing boundary and app coordinator: infrastructure; each window family still migrates onto it ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-12-strategic-window-routing-coordinator.md)).
- [x] P46D detailed-system tab items: checkpoint; nested compositions, drag, intelligence and A0 open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-11-detailed-system-tab-items.md)).
- [x] P47A / `UIP-B02` STRATEGY 902 galaxy backdrop (the Display Off resource): pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-11-authored-galaxy-backdrop.md)).
- [x] P47B default Popular Support GID view: checkpoint; `CMD-02` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-11-popular-support-gid.md)).
- [x] GID browser harness and 38-scenario catalog: in place, 152 probes pass; no original capture accepted, `CMD-02` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-gid-browser-harness.md)).
- [x] P49 Message Index rail resting artwork: pass; `CMD-08` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-message-index-rail.md)).
- [x] P60 Game Speed control, stop-day pause and accelerators (save v14): checkpoint; `CMD-05` and `CMD-08` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-24-game-speed-recovery.md)).
- [x] P63 Message Index shell: checkpoint; `CMD-08` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-28-message-index-shell.md)).
- [x] P50 GID root and submenu frame: checkpoint; `CMD-02` open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-gid-menu-frame.md)).
- [x] P51 GID row-hover wash removed: checkpoint; original hover art and stacking open ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-gid-hover-and-occlusion.md)).

Encyclopedia (`OBJ-01`): all thirteen cells pass against the original under
Wine, accepted by Tom on 2026-10-07; native-Windows parity is not claimed.

- [x] P62 EDATA artwork transport: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-28-encyclopedia-artwork-transport.md)).
- [x] P64 index shells: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-30-encyclopedia-index-shell.md)).
- [x] P65 index catalog, corrected by P68 to 346 visible topics: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-08-encyclopedia-visible-catalog-correction.md)).
- [x] P66A topic sources: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-01-encyclopedia-topic-source-bindings.md)).
- [x] W2 content session: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-content-session.md)).
- [x] W3 presenter: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-presenter.md)).
- [x] W4 topic surface: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-topic-surface.md)).
- [x] W5A canonical publication: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-canonical-publication.md)).
- [x] W6 opt-in native faithful-HD selection: pass; browser stays original-only ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-06-encyclopedia-hd-selection.md)).
- [x] W7 native mod overlays: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-07-encyclopedia-mod-overlays.md)).
- [x] E32 production activation: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-07-encyclopedia-production-activation.md)).
- [x] E34 assembled technical acceptance: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-07-encyclopedia-technical-acceptance.md)).
- [x] E35 original comparison: pass ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-10-07-encyclopedia-a0-compatibility.md)).

Tactical (`TAC-01` through `TAC-07`, 106 cells). Each checkpoint below passes
its muted browser gate; every strict cell waits on the A0 matrix after them.

- [x] P52 tactical shell (TACTICAL 1000) and bitmap controls ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-shell-controls.md)).
- [x] P53 tactical control hit masks and held states ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-control-hit-states.md)).
- [x] P54 tactical 3D resource staging ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-3d-staging.md)).
- [x] P55 tactical mesh, image and palette conversion ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-12-tactical-3d-runtime-pack.md)).
- [x] P56 single-resource browser rendering ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-3d-render-proof.md)).
- [x] P57A three-LOD selection ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-3d-lod-family.md)).
- [x] P57B1 live cached-slot switching ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-live-lod-journey.md)).
- [x] P57B2A source camera and D-pad ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-camera-contract.md)).
- [x] P57B2B1 selected-object target ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-target-control.md)).
- [x] P57B2B2 active-force layout ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-battle-layout.md)).
- [x] P57B2B3 participant placement ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-participant-placement.md)).
- [x] P57B2C1 authored transforms and system palettes ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-13-tactical-transform-palette.md)).
- [x] P57B2C2A light rig ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-light-rig.md)).
- [x] P57B2C2B retained-mode render state ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-render-state.md)).
- [x] P58A tactical resource identity ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-resource-join.md)).
- [x] P58B production capital-ship families ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-production-participants.md)).
- [x] P58C fighters and projected selection ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-fighters-selection.md)).
- [x] P58D fighter-detail journey ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-fighter-detail-journey.md)).
- [x] P58E task-force and fighter-group controls ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-group-presentation.md)).
- [x] P58F1 system-selected planets ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-planets.md)).
- [x] P58F2 impact effects ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-impact-effects.md)).
- [x] P58F3 projectiles and fields ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-14-tactical-projectile-fields.md)).
- [x] P58F4 selected-capital damage panel ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-selected-damage.md)).
- [x] P58F5 subsystem and field-source presentation ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-subsystem-field-commands.md)).
- [x] P58F6 live subsystem damage ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-live-subsystem-damage.md)).
- [x] P58F7 subsystem repair and mobility ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-15-tactical-subsystem-repair-mobility.md)).
- [x] P58F8 maneuver and movement ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-16-tactical-maneuver-movement.md)).
- [x] P58F9 command assignment panels ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-17-tactical-command-assignment.md)).
- [x] P58F10 order execution ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-17-tactical-order-execution.md)).
- [x] P58F11 command progression and fighter recovery ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-18-tactical-command-progression.md)).
- [x] P58F12 attack target acquisition ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-18-tactical-attack-targeting.md)).
- [x] P58F13 attack-target lifecycle ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-19-tactical-attack-target-lifecycle.md)).
- [x] `UIP-B06` practical space-battle bundle, P58-B06 through P58-B22: implemented, 144/144 final muted matrix ([evidence](../docs/qa/2026-09-10-interface-parity-audit/evidence/2026-09-28-tactical-source-completion.md); the B-series files sit beside it).
- [ ] Pass the strict 106-cell tactical matrix against A0 captures of the
  original. 113 of 114 references are captured in the capture VM, with
  TAC-03-C020 settled without a frame. The first strict C6 verdict was pass
  11, fail 66, not-run 28, settled 1, with the negative controls failing
  their rows (6 of 6). The remaining work is the task graph
  (`agent_docs/task-graph.md`, `or-ep-tac-x15`): one acceptance node per
  failing region, fed by Ghidra questions, diagnoses and the fixtures that
  let the not-run cells run. Exact beam dimensions and timing, native
  playback comparison and rare audio routes close with it.

Further M3 work:

- [ ] After original parity, use a separate Fable and `/ce:plan` design pass for
  an optional X-Wing Alliance-style skirmish setup menu. It may select ship
  distribution, planet, and space background, but it must reuse the production
  battle scene and remain outside the original 106-cell parity denominator.
- [ ] After original 640×480 parity, add an optional enhanced-widescreen layout.
  Preserve authentic bitmap panels and control sizes while extending only the
  galaxy, system, and tactical viewports into the additional horizontal space.
  Treat genuine 16:9 support as a separate modernization track outside the
  original 106-cell parity denominator:
  - anchor authentic cockpit panels to the screen edges;
  - extend backgrounds without stretching their bitmaps;
  - adjust tactical-camera projection and galaxy-map bounds;
  - translate pointer coordinates through the widescreen layout;
  - preserve original hit masks and control geometry;
  - pillarbox videos and other fixed 4:3 surfaces; and
  - maintain separate visual baselines for supported 16:9 resolutions.
- [ ] Execute `RE-ADV-02` for authored SPT/BIN/FDT action, cadence, preemption,
  and voice mappings; extend the decoder/pack to briefings and every other
  required resource kind; then consume the proven shell/control/window mappings.
  Done: triggered droid actions, voices, and the opening briefing tour with
  its skip, both factions, native and packed (`ghidra/notes/droid-advisor-triggers.md`).
  Open: the Advice topics the tour leaves, droids during the battle dialog.
- [ ] Add Brotli compression plus bounded raw-byte and decoded-texture caches to the verified runtime-pack foundation.
- [ ] Complete P45 faithful-HD modernization. P45A is complete: original parity
  is separate from opt-in HD, manifests are source-bound, and deterministic
  indexed scaling plus fail-closed fallback are verified. P45B-P45E own complete-family model evaluation, protected masks,
  optional browser packs, and cross-browser acceptance.
- [ ] Enable high DPI; use one egui pass; cache sector geometry.
- [ ] Move saves from synchronous base64 `localStorage` to compressed asynchronous IndexedDB.
- [ ] Add authored advisor voice and preemption after user gesture; advisor idle
  assets are now staged from a contributor-owned installation. Voices play
  natively and are packed for the browser; browser playback is unverified.
- [ ] Exercise real mouse/keyboard input and bitmap screenshots in Chrome, Firefox, and Safari.
- [ ] Meet budgets: ≤3 s cold start at 50 Mbps/30 ms, ≤4 pre-menu requests, ≤8 ms frame, ≤12 ms WASM tick at 1,000 fleets, ≤5 MB optimized WASM, and ≤256 MB combined memory after 10 minutes.

### M4: Authoritative Multiplayer (6–8 weeks after M1–M3)

- [ ] Add validated `{player, tick, sequence}` commands and route single-player through the same boundary.
- [ ] Add `rebellion-net` and `rebellion-server` responsibilities for authoritative simulation and fog-filtered faction deltas/snapshots.
- [ ] Start with in-process test transport and secure WebSockets; never expose the hidden full `GameWorld` to clients.
- [ ] Add prediction/reconciliation, reconnect snapshots plus command log, persistence, authentication, rate limits, TLS, and observability.
- [ ] Pass two clients for 5,000 ticks with hashes matching every 250 ticks; no 200 ms RTT input stalls; ≤1 reconcile per 100 commands; reconnect ≤60 s; reject 100% of illegal commands.
- [ ] Consider trusted lockstep/WebRTC only after deterministic replay is proven.

### M5: Continuous Release Proof (parallel from M0)

- [ ] Restore a reviewed CI provider; GitHub Actions workflow definitions are intentionally local/untracked as of 2026-09-08, so current gates are manual.
- [ ] Enforce format, strict clippy, native, packaged-WASM, and clean-package boot gates.
  `make all` defines tests, formatting, strict all-target Clippy and the native
  build, but runs Cargo in place, so it does not run on this Mac; formatting and
  strict Clippy fail on the audited baseline. Packaged browser and CI
  enforcement remain open.
- [ ] Add at least 20 app/browser integration cases and promote long campaigns when stable.
- [ ] Generate the complete bitmap resource ledger from the entity map; fail on unexplained misses. Fleet-list miniatures are verified in F-010B.
- [x] Interface baseline of 627 required cells across 43 families, 106 of them tactical (`TAC-01` through `TAC-07`): established ([interface audit](../docs/qa/2026-09-10-interface-parity-audit/README.md)).
- [ ] Close every required surface-state cell in the original-interface audit
  with authoritative captures, exact-resource provenance, screenshot and hotspot
  comparison, complete navigation edges, and zero unknown or invented pixels.
- [ ] Retain screenshot, console, network, performance, data-hash, and save-schema evidence. F-010B fleet and F-014A runtime-pack evidence are retained; the full ledger remains open.
- [x] P40 project documentation on GitHub Pages: pass ([evidence](../docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-github-pages.md)).
- [ ] Close every supported P00–P40 functionality pass and every required
  interface surface-state cell from release artifacts before claiming 100%
  functionality.

### v1.0: Password-Protected Cloudflare Release

- [ ] Deploy the self-contained browser artifact to Cloudflare Pages with Pages Functions middleware.
- [ ] Keep `SITE_PASSWORD` and `SESSION_SECRET` in encrypted Wrangler/Pages secrets; commit no credentials or reusable auth token.
- [ ] Sign an expiring `HttpOnly; Secure; SameSite=Lax` session cookie and provide logout; reject missing, expired, or tampered sessions.
- [ ] Gate HTML, WASM, DAT, bitmap/HD packs, save endpoints, WebSockets, and multiplayer APIs—not only the landing page.
- [ ] Add CSP and security headers plus immutable caching for hashed public assets; never publicly cache authenticated HTML or auth responses.
- [ ] Maintain preview and production environments, commit-linked deploys, smoke checks, retained artifacts, and documented rollback.
- [ ] Have `/codex-orchestrator` run an independent browser-use reviewer against the deployed URL for authentication, single-player, multiplayer, input, performance, and bitmap screenshot acceptance in Chrome, Firefox, and Safari.

v1.0 exits only when the protected Cloudflare deployment passes P39, the
GitHub Pages documentation passes P40, and every supported earlier pass. This follows the local Pages `functions/_middleware`
pattern while replacing hardcoded passwords/cookie values with encrypted
secrets and signed sessions.

---

## Roadmap: Addons (v0.15.0+)

### Addon 1: Create Your Own Character (Demiurgos)
*Plan at `docs/plans/2026-03-21-addon-create-your-own-character.md`*

**Phase 1: Homeworld System** (universal—enriches base game)
- [ ] Add `homeworld: Option<SystemKey>` to Character
- [ ] Create `data/homeworlds.json` with canon homeworld assignments (Luke→Tatooine, etc.)
- [ ] HomeworldModifier: +10% at homeworld, -5% when enemy controls, -15 loyalty on fall
- [ ] Wire into missions.rs probability + betrayal.rs loyalty
- [ ] Display in the character's Status window and Encyclopedia

**Phase 2: Character Creation Core**
- [ ] Add `CharacterOrigin`, `AcademyArchetype`, `ArchetypeTemplate` to rebellion-core
- [ ] 6 archetypes: Diplomat, Operative, FleetOfficer, GroundCommander, Engineer, Fringer
- [ ] 7-step creation UI as egui modal (faction → archetype → skills → homeworld → Force roll → name → review)
- [ ] Point-buy within archetype floors/ceilings (5-point increments)
- [ ] Max 3 custom characters per game

**Phase 3: Force Sensitivity**
- [ ] 8-16% base chance depending on species + archetype
- [ ] Success: jedi_probability 15-30 (Luke = 100—custom characters strictly inferior)
- [ ] Wire to existing JediSystem progression

**Phase 4: Portraits & Flavor**
- [ ] 128 portrait BMPs (8 per species × 2 factions × 8 species)
- [ ] Backstory templates: "Former Separatist Holdout", "Outer Rim Survivor", "Imperial Defector"
- [ ] Encyclopedia integration

**Phase 5: Mod Integration**
- [ ] Extend mod loader for `"action": "add"` entity creation
- [ ] DatId allocation from reserved custom range (0xFF000000+)
- [ ] Example mod: "Kira Noss" character pack

### Addon 2: Expanded Galaxy (Future)
- Additional star systems from EU (200 → 400)
- New sectors (Unknown Regions, Corporate Sector)
- Hyperspace lane network (optional overlay on Euclidean model)

### Mod surface (Ghidra groundwork)
The tactical engine resolves every space battle, played or not; its parity
is tracked under `or-ep-tac-x15`. Before addons open content to mods, the
`or-ep-mod-surface-vdch` questions (all answered) map what the original
fixes in code rather than data: Force users and training, the story encounters, mission
families, ship-type assets, the 200-system galaxy, the movies, the
per-side switches and the two-side limit, the object ids the executable
names directly, the Death Star, record limits, the character roster, ground
units, audio and text. Each answer names the constants, family checks and
hard-coded ids the port must turn into data for new Jedi, encounters,
missions, ships, systems, movies, sounds and translations, and for era
overhauls that rename the sides, replace the scenario or add a superweapon.
`agent_docs/mod-surface.md` collects the answers and the rules for exposing
a fixed rule as mod data.

- [ ] Port what the answers found the port lacks or invents: officer rank
  and command orders `0x260`-`0x263` (`or-port-officer-rank-x4kn`),
  majors-only recruiters and major-first mission leads
  (`or-port-recruit-major-lead-8grm`), the original music selector in place
  of `track_for_context` (`or-port-music-selector-v9nd`), the Force fields
  (`or-port-force-fields-2u0m`), and the post-battle pass for system
  squadrons (`or-port-system-squadron-pass-oa3p`).
- [ ] Decide whether mods may append records. The original loads any count
  but has no class for a new family byte and the port's loader skips
  patches with unknown ids (`ghidra/notes/record-registry.md`); allowing
  new records is an architecture change.

---

## Known Technical Debt

- dat-dumper in `tools/` is also a library dep of rebellion-data; it works but is unconventional
- Browser data now loads from deterministic `runtime.orpk`; compression, HD
  entries, and bounded caches remain open.
- No additive entity creation in mods; patches only modify existing entities (until Addon 1 Phase 5)
- 27 LOC dead code identified by simplicity reviewer (2026-03-21)
