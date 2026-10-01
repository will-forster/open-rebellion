---
title: "Original Encyclopedia Capture Evidence"
description: "Profile-bound Wine compatibility captures and explicit remaining A0 gaps for ENC-UI-01..21"
category: reference
created: 2026-09-29
updated: 2026-10-01
tags: [encyclopedia, ui, reverse-engineering, parity, provenance]
---

# Original Encyclopedia Capture Evidence

E51 exercised the inspected English executable whose SHA-256 is
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.
The read-only installation remained unchanged; all execution used the isolated
working copy described below. This is original-executable reference acquisition
under Wine, not proof that Wine pixels equal an original Windows installation,
not a comparison to the port, and not final encyclopedia acceptance.

## Environment and retained evidence

| Item | Value |
|---|---|
| Compatibility environment | Ubuntu 24.04 amd64, distro Wine 9.0, isolated win32 prefix |
| Display | authenticated local Xvfb `:91`, 1024 × 768 × 24; Fluxbox; no TCP listener, VNC, or public remote service |
| Game window | X11 title `Star Wars Rebellion`, observed `640 × 480+1+1` |
| Encyclopedia client | source-proven 470 × 330 child inside the unscaled 640 × 480 game client |
| Audio | environment-local null output |
| Capture pair | full 1024 × 768 desktop, then unscaled crop `(1,1,640,480)`; no resize or painted overlay |
| Evidence root | `/data/projects/open-rebellion/agent-work/original-game-capture/E51/` |
| Manifest | `manifest.json`, SHA-256 `8f9912f7e23d6d6209ce97e84ed103fee63fc451704235e818fad101add7a187` |
| Baseline inventory | 198 retained PNGs in `manifest.json`; 60 cited matrix states, each with full/client frames; eight additional inspected live-cache setup/context frames remain supporting-only |
| Reachable continuation | `reachable-captures-r7-summary.json`, SHA-256 `bb39f107c4b2cf44c04391559b4a9c893f3044739b3f3ffe4e8e5bd3f2b1afaa`; six immutable attempts/runs, four endpoint observations, four System selectors |
| Reviewed later additions | r41 retained 75 raw full-desktop PNGs plus 75 lossless same-acquisition client crops outside the baseline manifest; r42/r43/r45/r46 and r49–r52 added reviewed static evidence; r47 retained 70 raw full-desktop PNGs plus 70 lossless same-acquisition client crops; r53 retained 35 raw full-desktop PNGs plus 35 lossless same-acquisition client crops, all outside the baseline manifest |

The manifest records every PNG's byte length, dimensions, SHA-256,
disposition, action sequence, setup, and matrix association. Retained attempts
that were interrupted, mislabeled during acquisition, or did not change the
intended state remain hashed but are explicitly not cited. Proprietary pixels,
original prose, the working prefix, and any campaign state remain outside Git.

The final verification rehashed every PNG, checked all cited full/client
pairs, required separate acquisition and row-acceptance states for every
`ENC-UI-01..21` row, and rejects any row that hides unmet subcriteria behind a
generic captured label. Every cited client was inspected at original resolution;
full frames were checked for the unscaled client placement. The isolated Wine
game was stopped after capture while display `:91` remained available.

The 198-PNG manifest is the original bounded inventory and is not silently
rewritten when later runs add evidence. Coordinator-verified r41 evidence is
retained under `visible-r41-1/` and `visible-r41-2/`: 75 raw full frames plus
75 exact same-acquisition crops, with sorted PNG identity digest
`36f046e7c9d9672887df3027827811ed5605a0876365b34726d2f6539c55d28e`.
Its report SHA-256 is
`60fa5c5ae076c07360b7cc0a97185181fa34691357c8b1d23deda997ade9abf8`
and checker SHA-256 is
`0ff4fcb8e3346d4bea002a53caf7b741b9fdf8e518bd05355eae22fc92f362ba`.
The independently reviewed r42 source correction adds no pixels; its report,
source note, and checker SHA-256 values are respectively
`3c955d02f686b004ffa143ef21774f6033c7eaf69d1f35ab06da430d93ce455b`,
`9147007d8bff711d9d78b37589f92b11ab576ee53f7d91db7d7aa95ce68ea7c0`,
and `9b53a58dada0265dd5d2841964e2cd440336aa6ba0942cb1ecd792808b0ae68e`.
These additions preserve the baseline manifest and every rejected/interrupted
frame. They are Wine-visible observations or reviewed static inference as
identified below, never interchangeable proof.

The independently reviewed r43 Fleet ship-route trace is promoted only at its
static scope (review SHA-256
`e40d6772bb21103b85eff9bad9d92e40160c21bcffd7b963e007e60a93c9748a`).
It connects the r41 Corvette route through the typed Fleet owner, selected-row
wrapper collection, popup command `0x100`, `FUN_00486fb0`,
`FUN_0041d6b0`, and `FUN_00429f30`. On its create branch,
`FUN_00429f30` passes the contextual wrappers to `FUN_0045d400`; an existing
encyclopedia window is reused without reconstructing it. Deselect removes the
wrapper matching the same packed/base row identity, not a proven discriminator
match. This does not turn the pixels into a live numeric-command, resource,
canonical-DAT, notification, or focus observation.

The independently reviewed r45 mission-dialog trace proves a second
class-context caller at static scope. `FUN_0046c3c0` command `0x67` reads the
selected mission-definition row, passes a class wrapper and empty entity
companion through `FUN_0041d6b0`, and reaches `FUN_00429f30`. Only the
create branch consumes those wrappers in `FUN_0045d400`; an existing
encyclopedia child is reused without reconstructing its context. The r45
report/source/checker/review SHA-256 values are
`4621912fa106bfd8f5ba433c8a4fdb4ac7d33a1c1d08fd00b590b659a10c2dbe`,
`7fb3b40c4d0288646d7c6b8e23adf3a0d778923b979b185259a1ac08c4ede520`,
`207f78b59f72e9b48e1a4888611f34502e94d525da5f6de4de11c1a8c016f68c`,
and `a0d746f21ff00227d2a0eed8d77ea0943aea22f731f2acc5dbac0f529fdae3bb`.

The independently reviewed r46 trace proves the conditional upstream player
route at static scope: `TEXTCOMM.DLL` accelerator table 11 maps Alt+M to
command `0xbc0`; the root loop forwards it to the active child as message
`0x483`; a valid selected live Personnel entity may construct order `0x240`;
and only a nonempty `FUN_004f5380` legal-mission result admits the mission
dialog. That live Personnel entity is the mission-team input, not the later
mission-definition class row used by r45. The r46 report/source/checker/review
hashes are
`166d66f446039b8ddf96bb5cd50827f28e77ba3a127a72bf67355fe712ddbd96`,
`1dd786bf12e9f02378e26b5a14329a09266e2ec5cc10a7538a58971027169837`,
`8d7468b75db9b43c5071398a04bcfca392229cef0953955105f0ba86afc4fecb`,
and `7f114fcb62e16b39a6ded1146a047c8085076c6d625eb73d453f80b8d77d4e81`.

Coordinator-accepted r47 visible evidence is retained under
`visible-r47-1/`: 70 raw 1024×768 frames plus 70 exact same-acquisition
640×480 crops. Its full-frame and crop sorted identity digests are
`8fd989a44bd4fa5f8fd281da7437ef267a687b1c315080170801554bd128e631`
and `9666c639bf6528e99c6014bc19077c72c4bf3834614e56b2bc7badf9697444e1`.
The report/checker/action-journal hashes are
`538a0d3f5b6e71b361e0f9b37031fb8255bc1b3634077dd4255e6358a532acdf`,
`1c75f64daf9296546fe826b670af0ffedc711e68663402b352d5726356bd9d89`,
and `de324268a33f48bf6c4901f68c3755fe0ba9d233587c8beeed61651f2a91fb7c`.
This was one ordinary Alliance / Sumitra / Yavin run against the identified
English profile and executable. Frames `058`–`063` visibly join selected
Recruitment to its matching article and close back to the surviving mission /
Personnel surfaces; `066`–`070` do the same for Espionage. These are visible
mission-definition class-context observations, not runtime command, resource,
notification, DatId, or target-thread-focus traces.

| r47 full-frame artifact | SHA-256 | Reviewed visible role |
|---|---|---|
| `058-mission-choices-full.png` | `dcc614223b56071d0f9032d75518f42506816fe3210caa73c7a80c772da739fe` | Real Recruitment selector row visible |
| `059-recruitment-selected-full.png` | `563369cd429763d988b95f3c7cc6fd0cbe20d18ef8a664446acc4f672ed4fd8c` | Recruitment selected |
| `060-mission-encyclopedia-before-full.png` | `e5ef44d41362a071a7cbccf8fccd609893a8c8fc9941f769a4e87bd033cd75a1` | Visible mission-dialog Encyclopedia control/tooltip |
| `061-recruitment-article-full.png` | `e15de48fbadf2f32b667702587a36bcdfe18c4cf079547c12363cc2c2b5e42d6` | Matching Recruitment article visible |
| `063-article-close-after-full.png` | `13dc00123d88408afb507c1159f81ca82e33ce74d016b88cd993f43b9bec84e0` | Recruitment mission and Personnel windows retained after close |
| `066-espionage-row-full.png` | `f62449aa0a6ef32d2383b8fc9fd5e618b1a8523707079362d33ba61f35404864` | Real Espionage selector row visible |
| `067-espionage-selected-full.png` | `13f3378540f70efc25f7d8241284f1a933d2b47b6885c9033796ccd87a1c5be6` | Espionage selected |
| `068-espionage-ency-before-full.png` | `bf1b93ccf5f9191c571cc5ce48c523bd2688c356a65458d42983ebc010700dfc` | Visible mission-dialog Encyclopedia control/tooltip |
| `069-espionage-article-full.png` | `bbe29e3c7e181fb7ef3c1ca525a156bf4864f94f2586627f4cfae7cad5a4a28f` | Matching Espionage article visible |
| `070-espionage-close-after-full.png` | `3d39de178befc518bb8db4b4ebb8eafa1bdd6d73c5a6f2b6fb925562e810554b` | Espionage mission and Personnel windows retained after close |

The upstream r47 hotkey result stays explicitly negative/inconclusive:
`055-leia-selected-before-altm-full.png` and
`056-after-altm-full.png` are byte-identical, both SHA-256
`8ff85f6dcfffed2c73b8bedc121140981dddf5e942f24f81faa08c24f861f97b`.
The distinct Personnel-tab double-click is the next game input, and only then
does `057-personnel-tab-double-full.png` (SHA-256
`802b2e74d4963881a3af120ea84c14d732753a680aa16809c9c03bf7a31024c5`)
show Create Mission. The visible downstream mission route is retained without
attributing dialog creation to Alt+M. Cleanup is also qualified: the exact
prefix stop returned zero and no owned process/window remained, but the
supervisor exited 143 and the launcher exit status is unknown.

The accepted r49–r52 static packet identifies handler `0x00438800` as the
ordinary Build Selection dialog's class-context route, subject to the r50/r51
review corrections and r52 retained-edge limit. Command `0x67` table index `3`
reaches `0x004388d6`; the selected row supplies a class wrapper at `+0x54`, a
separately empty entity companion is constructed, and the pair reaches
`FUN_0041d6b0` / `FUN_00429f30`. The submission bridge reaches the retained
build controller. Same-instance identity is proven only for the ordinary
fresh-created collection on the key-check/equal edge: helper range
`0049e130–0049e196` (SHA-256
`2e279e7bf11780e1c28ecfd66dbd02329794b35bfb813969456ae43ab88dcc70`)
returns exact nodes from the same collection and trampoline
`00568ee0–00568ee5` (SHA-256
`ad754bd90725bde9c206591c56972f8678048a6af3b94433e5587eb7acd98c1e`)
reaches the exact-node leftmost fallback. The `0x00439ae4` bypass, malformed
or duplicate-key serialized state, live command/resource/current-key/focus,
and an already-open encyclopedia remain outside that proof. The r49 report /
source / checker / review hashes are
`a3b3033417ff76eb187259aa3a18109a814dcf7ae75da5dc1f01d823162b5d46`,
`095f8d20b6da3727294348abb93bf7869ff664f29b94206cee70b51fc85156ef`,
`ea1d2f5d679f31a5f7950ad098c1a718117c8b5ae321715ce622ba0a0737a3d2`,
and `b4f93ebe5e02e4c05ba246a4f2217bfd2a1e4ecdb1b34ac4584e541cf84a893a`.
The corrected r50 review, r51 review, and r52 report/source/checker hashes are
`69e98d13601cd4f977c6d106580c65cae89c8d4626e310a03032cbfd33e61fa8`,
`786facd40953ad8d8d7c48276ac4894d1283b44319f1224d8e6ab06820373610`,
`bbfb800a56a9b088c4deb33cef5e6b7b338b6fd7c849dc48b854e68ca4853e62`,
`9c9728cc9b5de81348b4973dedaaccd67d5e3efefdccec7238825aed47832c36`,
and `3050238e9e25c9a18a29717f092bb7932eb2a6ae3bd9f7ee623ad16d64af8552`.

Coordinator-accepted r53 Wine-visible evidence is retained under
`visible-r53-1/`: 35 raw 1024×768 frames plus 35 exact same-acquisition
640×480 crops. Its report/checker/artifact-manifest/action-journal SHA-256
values are
`596b732d5925632a6a855ce35252592dfdb609706ecd902a977b867004b101d4`,
`d05db1e6d73babf700c5e6276049e84f399e6ac42f7e5a36d499c9016dd29cd8`,
`6b0fe05f26222a4f88a69637900680403ffc0cb634cb1896d4f07647dde73dbe`,
and `baefeb1e52ca31d4ca67e1de792cdbea8b3e9e522144db476b4c2769ca82274c`.
In one ordinary Alliance / Chandrila run, the first post-shortcut frame showed
no candidate dialog; a later visible Shipyards yard click opened Build
Selection. The selected Alliance Escort Carrier row, matching article, and
close back to the surviving Build Selection and Chandrila Shipyards surfaces
are visible class-context evidence. They do not prove that Alt+B caused the
dialog, nor a live command/notification, canonical key/DatId, loader-selected
resource, or target-thread focus.

| r53 full-frame artifact | SHA-256 | Reviewed visible role |
|---|---|---|
| `030-selected-escort-carrier-full.png` | `3abec7b2221f0819160e46e21bf9e3451dd762a31e90b584b064a22143a751ea` | Real Alliance Escort Carrier row selected in Build Selection |
| `032-contextual-article-full.png` | `3899decfb20b1b1e937e2d432c539444afbb439feb63a33876356f59b00a5b00` | Matching Alliance Escort Carrier article visible |
| `036-surviving-build-state-full.png` | `4c49f5fe41a59cbfc117f038c87e42f4cda245ed7cd5f9be926f99d6f606d505` | Encyclopedia closed; Build Selection and Chandrila Shipyards survive |

The r53 launcher exited `0` and was reaped; exact-prefix stop returned `0`;
the guard wrote its cancellation record and its PID is absent. The guard
wait/reap exit status remains unknown. Lifecycle-shell exit `143` is not
substituted as the guard exit. The r35 helper quarantine remains unchanged.

The accepted r44 four-file metadata snapshot remains a historical validation at
commit `1c3140059d8bd69d94715e47c907667c0fe29801`; its source hashes are not
rewritten to describe this later evidence. The separate r48 metadata checker
(`check-r48-metadata.py`, SHA-256
`091e4130d991dd2798305a27e77d20634724b6834aac29500d93743ccc02bd4d`)
recomputes those historical Git-object hashes, the current four-file identities,
the 21 row dispositions, and the accepted r45/r46/r47 artifact links.

## Setup and routes

Both factions began from a fresh original main-menu Standard Game selection.
The faction intro was skipped with Escape. F7 was exercised directly. Shell
command `0x131` was exercised from the original encyclopedia hotspot: the
Alliance route opened after the shell was ready; on the fresh Empire campaign
the pre-gate click retained the strategy shell, then the same route opened
after the campaign reached the source-proven admission condition. Pressing F7
again while the child was open left one visible encyclopedia child on both
sides. This visual result corroborates the single-child route but does not
instrument the internal collection count.

The seven command buttons were selected in source order on both sides. The
inspected English environment displayed, in order, `All Databases`, `System
Database`, `Ship Database`, `Facilities Database`, `Missions Database`,
`Troop Database`, and `Personnel Database`. This is an A0 observation tied to
the action sequence; it does not independently identify the backing localized
string resources or prove other profiles use the same labels.

Index single-click, double-click, Return, Home/End selection, category
Left/Right, topic navigation buttons, body scrollbar Down, Tab, Return,
Escape, and the close control were exercised. The same first Ship topic was
captured under both faction shells. The r7 continuation additionally retained
source-connected current-row metadata for full and filtered endpoints, an
ordinary list PageDown/Return selection, pointer body scrolling, Escape close,
and Alliance System selectors 1, 24, 25, and 26. It did not read localized
strings or the filename chosen inside the original art loader.

The later r47 Alliance route used ordinary Yavin system Personnel navigation.
Leia Organa was visibly attached to Yavin and awaiting orders, then selected in
the portrait grid. Recruitment and Espionage were selected as distinct real
mission-definition rows; each mission dialog's Encyclopedia control visibly
opened the matching article, and ordinary close retained the mission and
Personnel windows. No mission was confirmed. This visible sequence does not
identify the internal selected resource or focus, and its separate Alt+M
attempt did not isolate the dialog-opening cause.

The later r53 Alliance route used ordinary Chandrila manufacturing navigation.
One Alt+B action was followed by a no-dialog frame; clicking the visible
Shipyards yard later opened Build Selection. Selecting Alliance Escort Carrier
and activating the visible Encyclopedia control opened its matching article;
ordinary close retained both Build Selection and Chandrila Shipyards. No build
order or save action was issued. This completes that visible class-context
subcase only, not its internal runtime identities.

The r55–r59 additions separate three evidence types. Accepted r55 static
evidence names `0x004443a0` as the guarded Character Status class-context
owner; its source note SHA-256 is
`ff7a0c7b58d0e1d4d29ebbf9fe022281b7766a2e24e32e2b447e47c0fe6b2b97`.
Accepted r57 static evidence names `0x00467f10` as the Message Index
Research Report class-context owner; its source-note SHA-256 is
`5703716043c4db73e3d4c1d59fa84954b079f532da2ca7e0f057efc725b3170b`.
These are source traces, not live command or resource observations.

R56 is a separate bounded Wine runtime observation. Two equal
`GetGUIThreadInfo` samples identified the validated initial-index child as the
target thread's focused HWND, with bounded helper and game cleanup. The
observation SHA-256 is
`5de023bb410b25abf34d81bbb08e922a32e83cae19b099955fd71af8d38ea4bf`.
It clears only the initial-index focus subcriterion; it does not observe
notifications, loader-selected resources, a selected canonical current key,
other actions, contextual routes, or Windows parity. The older r35 quarantine
remains immutable at
`ab5d3c77ea52326b7152ee80b3120887146dd07e2549deb2687dc4ad129f24a7`.

R58 visibly joined Alliance Yavin Personnel / Leia Character Status to a
matching Leia article and showed Personnel with Leia still selected after the
encyclopedia alone closed. Its manifest SHA-256 is
`1198943b9f390124cc247a13f02740d710917ecf9f57dbc0d9e96acf114154d7`.
The article covered Status, so this does not prove the hidden Status object's
destruction or close ordering. R59 is verified inconclusive evidence, not a
Research Report success: Message Index became visible later, Manufacturing's
tooltip was inspected, and its complete visible list was empty. Its manifest
SHA-256 is
`d1af9b876dc93f05b5c6e1ed67a8c7690915930c8dd35fe68d18435a235c5403`.
It proves neither F6 causality nor report/article/close behavior. Lifecycle
records retain supervisor `143`; launcher and guard exit/reap remain unknown,
and incomplete launcher output is not reinterpreted.

## Matrix result

`Partial acquisition` means retained evidence exists but at least one original
E27 subcriterion is unmet. `Missing acquisition` names an unavailable
source-backed setup. `Open` is the independent row-acceptance state; no row is
accepted merely because screenshots exist.

| ID | Acquisition / acceptance | Acquired evidence and exact unmet subcriteria |
|---|---|---|
| `ENC-UI-01` | Partial / Open | Alliance F7, post-gate `0x131`, duplicate-F7 visible-child evidence, and an ordinary live child command `0x19`/mode-1/current-null observation exist. R56 adds two equal bounded target-thread samples focused on the validated initial-index child. Missing: Alliance pre-gate command observation, runtime-selected resources, and instrumented duplicate-F7 child count. |
| `ENC-UI-02` | Partial / Open | Empire F7, pre/post behavioral `0x131` sequence, and duplicate-F7 visible-child observation exist. Missing: the direct `FUN_004fcee0` gate value, runtime-selected resources, focus trace, and instrumented child identity/count; hotspot plus elapsed game time does not itself prove the gate value. |
| `ENC-UI-03` | Partial / Open | Visible: all seven Alliance selected-category frames, labels, and visible list portions. Reviewed static inference: commands are ordered `0x6f..0x75`; the accepted profile candidate counts are 347/200/38/14/15/10/69 and comparator-equal ties preserve source insertion order. Missing runtime proof: complete admitted membership identities/order, empty/gap outcomes, selected resources, and focus. |
| `ENC-UI-04` | Partial / Open | Visible: all seven Empire selected-category frames, labels, and visible list portions. Reviewed static inference: the same fixed command order, candidate counts, and tie rule. Missing runtime proof: complete admitted membership identities/order, empty/gap outcomes, selected resources, and focus. |
| `ENC-UI-05` | Partial / Open | Category and Close pointer/press/cancel/selected sequences on both sides plus selected Alliance endpoint controls exist. Missing: exhaustive mode/navigation/disabled states and cursor/strobe timing. |
| `ENC-UI-06` | Partial / Open | Visible: single click, clean double-click, Return, selected-row pixels, and a same-run Ship PageDown/Return ending at canonical key `5963`, packed identity `0x14000002`, mode 2/category `0x71`. Reviewed static inference: `FUN_006083c0` emits `0x29b`, repeat-click `0x29d`, and double-click/Return `0x309`; `FUN_0045da70` conditionally maps `0x309` to command `0x67` only when selected/current rows are non-null. Missing runtime proof: delivered notification identity, internal Win32 focus, and pointer-click/double-click selected-resource joins. |
| `ENC-UI-07` | Missing / Open | Visible subcases: r41 Fleet/Corellian Corvette, r47 Recruitment and Espionage mission rows, r53 Build Selection/Alliance Escort Carrier, and r58 Character Status/Leia each opened a matching article within their bounded visible evidence; their ordinary close outcomes are retained. Reviewed static evidence now names all five direct class-context owners: Fleet `FUN_00486fb0`, mission `FUN_0046c3c0`, Build Selection `0x00438800`, Character Status `0x004443a0`, and Message Index Research Report `0x00467f10`. R59 found no eligible Research Report in the complete visible Manufacturing list, so that fifth visible route remains blocked by a legitimate fixture. The r46 Personnel setup, r47 mission-definition row, and r58 resolved Character Status class are distinct identities; Alt+M, Alt+B, and F6 causality remain unisolated. Missing: live command/notification and selected-resource proof, canonical current keys, action-specific target-thread focus, and an eligible Research Report row/article/close sequence. |
| `ENC-UI-08` | Missing / Open | No controlled fixture drives all five entity-context callers with caller/control, entity/definition/current-key provenance and focus. |
| `ENC-UI-09` | Missing / Open | No source-backed save/setup exposes both special association fallback branches and a non-special miss. |
| `ENC-UI-10` | Missing / Open | No controlled stale/unavailable contextual-open fixture retains title/list/mode/focus state. |
| `ENC-UI-11` | Partial / Open | Same first Ship topic exists under both faction shells. Missing: runtime-selected image/text identities and composition trace. |
| `ENC-UI-12` | Partial / Open | Alliance selector rows 1, 24, 25, and 26 were source-joined to live current identities/body IDs with nonnull art objects and inspected distinct images. Missing: matching Empire-side observations and direct observation of the original loader's selected filename/composition; the DAT/lookup join is not itself a loader-memory read. |
| `ENC-UI-13` | Missing / Open | No controlled admitted/excluded System pair retains the selected view plus complete type-`0xf2` ancestry chain. |
| `ENC-UI-14` | Partial / Open | Visible/runtime metadata: explicit Left at full and filtered first rows retained canonical key `5696` and packed identity `0x1c000002` in two equal snapshots; native frames were inspected. Reviewed static inference: a null `0x83` neighbor branches before the `this+0x148` write, retaining the current row, while control disablement is handled separately. Missing: internal Win32 focus and complete after-action desktop pairs because immediate full frames caught a Wine/Xvfb repaint gap. |
| `ENC-UI-15` | Partial / Open | A middle topic with both directions enabled exists. Missing: connected disabled-row setup and skip-disabled identity trace. |
| `ENC-UI-16` | Partial / Open | Visible/runtime metadata: explicit Right retained full last canonical key `6802` / packed identity `0x38000002` and filtered last key `5699` / packed identity `0x1c000002` in two equal snapshots; native frames were inspected. Reviewed static inference: a null `0x84` neighbor retains `this+0x148`, with disablement separate. Missing: internal Win32 focus and complete after-action desktop pairs because immediate full frames caught a Wine/Xvfb repaint gap. |
| `ENC-UI-17` | Missing historical acquisition / Open; supported profile corpus-not-applicable | Reviewed corpus evidence: all 347 bound rows have effective art (331 direct/system selections plus 16 viewer-faction rows with two complete variants), covering 186 bound files; no legitimate no-art topic exists in this checksum-pinned profile. `EDATA.192` is unbound and publication-deferred, not a fixture. The general null-art paint branch remains untested, but no fictional original topic or E26 mod-null case is required for this profile. |
| `ENC-UI-18` | Partial / Open | Visible: r41 Chewbacca produced eight discrete Down and eight discrete Up actions; first visible motion and reversal occurred on the fourth respective press. Up 5-8 were interrupted by Message Index and are rejected as article-scroll evidence. A separate scrollbar down-arrow click moved immediately; the attempted pointer drag showed no motion and is retained as a no-motion observation, not drag proof. Reviewed static join: Chewbacca is `0x38000343`, title `10819`, body/art lookup `6723`, `EDATA.081`. Missing: explicit-newline, long-token, body Page-key, font/extent parity, internal focus, and a successful pointer-drag result. |
| `ENC-UI-19` | Partial / Open | Visible: topic Tab, Return, Escape, Close; r41 context-opened article Close restored Fleet 1; r47 Close restored the Recruitment and Espionage mission dialogs with the Personnel window still present; r53 Close restored Build Selection on Alliance Escort Carrier over Chandrila Shipyards; r58 Close returned to Personnel with Leia selected and Character Status no longer visible, without proving hidden destruction order. Runtime metadata: r7 Escape observed command-`0x19` child absence in two equal snapshots. Reviewed static inference: command `0xfb` dispatches to the same vtable `+0x30` close method as Escape. Missing: runtime observation of the `0xfb` command/notification and internal focus restoration. |
| `ENC-UI-20` | Partial / Open | Left from middle and first categories exists. Missing: connected hidden-predecessor configuration and complete selected-command/list/focus trace. |
| `ENC-UI-21` | Partial / Open | Right from middle and last-to-first wrap exists. Missing: connected hidden-successor/hidden-leftmost configurations and complete selected-command/list/focus trace. |

Fifteen rows retain their historical partial-acquisition classification, six
retain their historical missing-acquisition classification, and all 21
acceptance rows remain open. The unchanged 15/6/0 rollup is an acquisition /
acceptance count, not an applicability count: `ENC-UI-17` is now explicitly
corpus-not-applicable for the supported profile, so no original no-art fixture
is demanded even though its historical capture bucket remains `missing` and
the general branch remains untested. In explicit checker terms this remains
**15 partial / 6 missing / 0 accepted / 21 open**. E51 therefore remains blocked as a complete
A0 package. The partial set is still an
authoritative reference for its named actions and states; it must not be used
to infer the missing contextual, unavailable, hidden, or terminal
system-selector-side cases.

The visible Fleet, mission, build-selection, and Character Status class
subcases are complete only at their stated pixel/action scope. All five direct
class-context source owners are now named, but Message Index Research Report
still lacks an eligible ordinary row. Every controlled entity-context route,
internal command/notification/resource/current-key evidence, action-specific
focus beyond r56's initial index open, exceptional fallback/unavailable/hidden
fixtures, and original-Windows comparison remain open. The r35 quarantine
supplies no reusable authority. Original-Windows parity requires an
authenticated original-Windows capture environment, which this Wine setup
does not provide.

## Current blocker ledger

| Gate | Exact missing criterion | Evidence held | Smallest bounded next work |
|---|---|---|---|
| Runtime | Loader-selected title/body/art and delivered notifications for visible opens | Static selectors plus matching Wine-visible topics | One reviewed bounded observation on an already-visible route; do not infer from pixels |
| Runtime | Action-specific target-thread focus | R56 initial-index focus only | Observe the exact action child for one contextual open/close under a separately reviewed lane |
| Fixture | Direct entity context | Five class-context source owners; no controlled entity provenance | A source-backed owned entity fixture retaining entity, definition, current key, and focus |
| Fixture | Message Index Research Report | R57 static route; r59 verified empty Manufacturing list | A legitimate accepted-profile state with an eligible Research Report; no blind recapture or fabricated save |
| Source | Fallback, unavailable/stale, and hidden-category branches | Static branches and bounded absence searches | A connected writer or legitimate owned save/scenario for each named branch |
| Fixture | Type-`0xf2` exclusion and Empire selector counterparts | Two 100-row runs with zero exclusions; Alliance selector joins | A legitimate excluded-ancestry pair plus bounded Empire selector observations |
| Runtime | Remaining control/body edge cases | Endpoint, pointer, key, and scroll subsets | Finite source-backed fixtures for disabled rows, Page keys, newline/long-token/extent, and successful drag |
| Windows | Original-Windows parity | Wine 9 compatibility evidence only | Authenticated original-Windows capture environment |

## Reachable interaction continuation (r7)

The continuation used a new bounded state observer after four synthetic tests
failed before the module existed and then passed. The observer opens the exact
direct-child PID's memory `O_RDONLY`, validates the executable mapping and
encyclopedia vtable, and reads only the shell child link, mode, category,
control handles, current-row links/key/identity/enabled value, art-object
presence, viewer selector, and locale globals. Every checkpoint requires two
equal complete snapshots. It does not read localized text/body bytes,
notification IDs, art-loader filenames, arbitrary memory, or process focus;
the recorded `source_expected_focus` is the recovered target, not `GetFocus`.

| Run | Retained result |
|---|---|
| `E51-reachable-r7-run-1-alliance` | Rejected before state observation when a transient Wine-desktop title/geometry was not a valid game client. |
| `E51-reachable-r7-run-2-alliance` | Rejected fail-closed because the sibling full-cache recipe's current digest differed from approved digest `2347e6ad...01448`; no guard was weakened. |
| `E51-reachable-r7-run-3-alliance` | PID `3352465`, start `2026-09-29T15:16:38.970Z`; full/filtered first and full last endpoint evidence retained. A later PageDown step was rejected because it selected a middle topic. `run.json` SHA-256 `5d0f527646ecdad32d07d9bb169d189547dac64d366d38dc75a334925af529ae`. |
| `E51-reachable-r7-run-4-alliance` | Rejected body-PageDown attempt: clicking the visible body then PageDown still selected a middle topic, so visible targeting is not claimed as internal focus. |
| `E51-reachable-r7-run-5-alliance` | PID `3359853`, start `2026-09-29T15:24:11.660Z`; ordinary index, pointer scroll, list PageDown/Return, filtered Right endpoint, and Escape close retained. `run.json` SHA-256 `e886522e7440a8f3df132b2fb2451031729c5a90eef1d36d7f8bf88d08302a8b`. |
| `E51-reachable-r7-run-6-alliance-system-selectors` | PID `3399522`, start `2026-09-29T15:32:12.460Z`; four Alliance System-selector joins retained. `run.json` SHA-256 `e12c0477755a62d16b5a732b7a0853dc28a1bb8c5192aa32bd28a541d70b93dc`. |

The endpoint observations are exact same-run before/after pairs:

| Collection / action | Retained current row |
|---|---|
| Full `0x6f`, Left at first | canonical/text `5696`, packed identity `0x1c000002`, previous null; unchanged after Left |
| Filtered Ship `0x71`, Left at first | canonical/text `5696`, packed identity `0x1c000002`, previous null; unchanged after Left |
| Full `0x6f`, Right at last | canonical/text `6802`, packed identity `0x38000002`, next null; unchanged after Right |
| Filtered Ship `0x71`, Right at last | canonical/text `5699`, packed identity `0x1c000002`, next null; unchanged after Right |

The identical packed identity at the first and last filtered Ship rows does not
make that packed value a unique topic ID; the canonical resources differ. All
native 640 × 480 endpoint frames were inspected. Several immediate
after-action 1024 × 768 frames caught a compatibility-environment repaint gap;
they are retained as supporting-only rather than described as complete desktop
state. The paired native frames and two equal metadata snapshots remain the
endpoint evidence.

Alliance System topics were selected from the observed filtered cache position
and then joined to accepted E39 `SYSTEMSD`/lookup metadata:

| Selector | Live current identity | Body | Accepted lookup / art |
|---:|---|---:|---|
| 1 | `0x9200007a` | `7714` | `11100` / `EDATA.166` |
| 24 | `0x9000010f` | `8049` | `11125` / `EDATA.191` |
| 25 | `0x90000117` | `8057` | `11123` / `EDATA.189` |
| 26 | `0x9200011a` | `8066` | `11124` / `EDATA.190` |

Every live row was mode 2/category `0x70`, had a nonnull art object, and had
two equal metadata snapshots; every native/full frame was inspected. This
proves the live current-row/source join for this Alliance run. It does not read
the filename inside the original loader, prove Empire selection, or turn a DAT
join into direct loader composition evidence.

The retained metadata-only summary is
`reachable-captures-r7-summary.json`, SHA-256
`bb39f107c4b2cf44c04391559b4a9c893f3044739b3f3ffe4e8e5bd3f2b1afaa`.
Its checker rehashes all six immutable `run.json` files, requires two equal
bounded snapshots, recomputes every cited canonical snapshot hash, validates
endpoint identities, selector joins, safety flags, and 640 × 480 / 1024 × 768
PNG dimensions.

### Runtime packed-handle field qualification

The immutable r7 snapshots name `(row + 0x68) & 0x00ffffff` as `dat_id`.
That label is superseded, not the raw files: the field is the low 24 bits of a
runtime packed handle and is now called `packed_handle_low24` by the future
observer. It is not a general original DAT identity. Canonical keys `5696` and
`5699` even share packed handle `0x1c000002` and raw low-24 value `2`, so the
handle fragment cannot uniquely identify either endpoint resource.

An original source DatId requires a unique canonical-key/profile join to
accepted source metadata. Accordingly, the run-6 System DatIds `122`, `271`,
`279`, and `282` remain valid only because each selector row was explicitly
joined to accepted E39 `SYSTEMSD` metadata; they do not derive their
provenance from the raw snapshot field, even where the numbers coincide.

The superseding qualification is
`ui-state-packed-handle-field-provenance-r8.json`, SHA-256
`ab6b32ab15d771bb403f4ef179b27faabf89ca4c6a5b4524f94d06776d180b5c`.
It rehashes the unchanged r7 summary and all six run manifests and records no
recapture. Future observer/test SHA-256 values are respectively
`086c04b6f00d98f88da03011494dfe766ea158f78adbcc2e9580cbca95710c4d`
and
`33cc0a9f627c72ad8210bce40f2f8fcbb225ddefdc335928a9aaec27c005cafa`;
future output contains `packed_handle_low24` and no `dat_id` field.

## Exceptional-state investigation

These are investigated open rows, not manufactured captures or universal
absence claims:

| Matrix gap | Source path and candidate trigger | Attempt and exact missing prerequisite |
|---|---|---|
| `ENC-UI-07/08` | The five `FUN_0041d6b0` callers at `0x00438800`, `0x004443a0`, `0x00467f10`, `FUN_0046c3c0`, and `FUN_00486fb0`; a real selected class/entity from each owner | All five class owners are source-named: Build Selection, Character Status, Message Index Research Report, mission dialog, and Fleet. Four have bounded visible class results. R59 found no eligible Research Report in Manufacturing, so that visible subcase remains fixture-blocked. Runtime command/notification/resource/current-key/action-focus identities and all controlled entity-context subcases remain open. |
| `ENC-UI-09` | `FUN_0045d400 -> FUN_0045fd90`; a real family `0xa0..0xaf` entity through each `FUN_0040d760` / `FUN_004025b0` association branch, plus a non-special miss | Source branches were traced and retained owned save/setup roots searched. No source-backed selected entity/save for either branch or miss was found. |
| `ENC-UI-10` | `FUN_0045d400/FUN_0045fd90`; real stale/unavailable context whose direct and fallback keys fail | No legitimate stale/unavailable contextual save or scenario was present in the retained prefix/capture workspace. |
| `ENC-UI-13` | `FUN_00422620 -> FUN_004f6330/FUN_0053f090`; selected System view with type-`0xf2` ancestry paired with an admitted view | Both fresh-viewer bounded runs returned 100/100 iterator candidates with zero exclusions. A campaign/save that actually produces an excluded candidate remains required; zero in two runs is not absence proof. |
| `ENC-UI-17` | `FUN_0045fa60 -> FUN_0045f970`, with `FUN_0045f090` omitting the blit for null art | **Supported-profile corpus-not-applicable.** Reviewed profile accounting proves 347/347 bound rows have an effective art selection (331 direct/system plus 16 complete viewer-faction pairs) across 186 bound files. `EDATA.192` is unbound/publication-deferred. The generic null-art branch stays untested, but no original-profile capture fixture is required and E26 mod-null is separate. |
| Hidden categories | `FUN_0045ddc0` creates commands `0x6f..0x75`; `FUN_0045fe60` skips controls for which `IsWindowVisible` is false | All seven were visible in both retained faction setups. No connected original writer/configuration that hides one has been recovered. |

These items are now investigated rather than merely unattempted. All except
the supported-profile `ENC-UI-17` applicability disposition retain missing
fixtures; the latter preserves a general untested branch without demanding a
fictional original topic. A blocked
exceptional row does not invalidate the reachable endpoint, scroll, close, and
selector evidence above.

The smallest remaining probes are finite and distinct: retain delivered
command/notification, canonical current key, selected resource, and
target-thread focus for one already-visible Fleet, mission, Build Selection,
or Character Status class-context open; obtain a legitimate eligible Message
Index Research Report state and capture its downstream visible route; drive
one controlled direct entity-context open; capture each `FUN_0045fd90`
association branch plus a non-special miss; retain one legitimate unavailable
context; obtain one admitted/excluded type-`0xf2` System pair; and recover a
real hidden-category writer/configuration before exercising category skipping.
The separate r46 Alt+M setup question needs an uncontaminated before/hotkey/
after observation, but it is not a universal encyclopedia acceptance gate and
does not erase the already-observed mission-dialog encyclopedia route.

Resource-accounting terminology is exact: unused lookup IDs `7188` and
`11284` have zero bindings, but the files `EDATA.142` and `EDATA.143` are
bound through lookup IDs `7200` and `11296`. They are not unused files and do
not supply a no-art case.

## E09 comparator metadata replay

This observation is a distinct replay, not metadata retroactively attached to
the screenshots above. Replay 1 (PID `3201446`) verified the main-image map but
a sibling reader received `EACCES` from `/proc/3201446/mem` under Yama
`ptrace_scope=1`; it was stopped without reading either DWORD. Replay 2 used a
read-only controller as the new game's direct parent:

| Item | Observed value |
|---|---|
| Run / process | `E51-E09-replay-2`; PID `3202977`; `/proc` start `2026-09-29T12:37:49.560Z` |
| Executable | same identified SHA-256; PE32 preferred base `0x00400000`, `SizeOfImage=0x002c3000` |
| Runtime image | offset-zero `REBEXE.EXE` map at `0x00400000`; target data map `0x006a6000-0x006c0000` readable |
| Address translation | `DAT_006be840` → RVA `0x002be840` → runtime `0x006be840`; `DAT_006be850` → RVA `0x002be850` → runtime `0x006be850` |
| Recorded post-input checkpoint 1, `2026-09-29T12:38:22.660Z` | After timed startup and intro-skip input, LC_CTYPE LCID raw `09040000` = `0x0409` / 1033; code page raw `e4040000` = `0x04e4` / 1252. No contemporaneous state observation verified the raw artifact's main-menu label. |
| Recorded post-input checkpoint 2, `2026-09-29T12:38:41.858Z` | After timed Alliance-intent click, Escape, and F7 input, unchanged LCID `0x0409` / 1033 and code page 1252. No contemporaneous state observation verified campaign entry, encyclopedia opening, or cache construction. |
| Retained metadata | `process-globals-replay-2.json`, SHA-256 `711a5025826e40357ab069e0ea0978b2674d129f5f3d4503f5bb437cb86702c9` |
| Superseding provenance qualification | `process-globals-replay-2-provenance-r3.json`, SHA-256 `6151877e8c8b3e8a63e8b0f8e8048aea8b802d2c28a277c57a6888a902203004` |

Each stage opened `/proc/<exact-pid>/mem` read-only and performed two exact
four-byte `pread` calls. No process memory was written, no executable was
patched, and no broad dump was created. These process-resolved values are
stronger than registry, `GetACP`, LANGID, or resource-codepage inference, but
the raw artifact's stage/setup strings are historical operator labels for the
intended timed inputs, not instrumented UI or cache state. The qualification
artifact preserves that history and supersedes only its interpretation. The
reads do not prove successful `setlocale` timing, encyclopedia-cache
construction at the second timestamp, the full one-byte fold map, live System
iterator identities, admitted-cache membership, or order. The accepted
profile's 200 SYSTEMSD rows remain static candidates, not a claim that all were
admitted in this run. Agent Mail message `21456` requested the connected
source-derived root/pointer/identity recipe from SilverFalcon; none was
available for this bounded correction, so no cache traversal or new replay was
attempted.

## Source-connected live cache and iterator observation

Coordinator release `accepted-agent-mail-21465` approved corrected recipe
SHA-256
`2347e6adb3176877a5f93d7d9a4034ed93cb736d2ec6aba8d55be3a945c01448`.
Run `E51-E09-live-cache-1` then launched a new direct child and used only the
reviewed bounded reader. This run is separate from replay 2 and does not
overwrite or reinterpret its artifacts.

| Item | Observed value |
|---|---|
| Process | PID `3262757`; `/proc` start `2026-09-29T13:27:48.070Z`; exact executable SHA-256 `b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab` |
| Image | preferred/runtime base `0x00400000`; `SizeOfImage=0x002c3000` |
| Stability | two complete canonical metadata snapshots, identical SHA-256 `1c94f1a66ae8b590c768800f5c49b2c2e6f5ab645c00781380a354297b2ab345`; process identity and before/after sentinels equal |
| Shell/cache | raw viewer selector `1`; source-connected nonnull shell/cache with reviewed vtables; mode `2`; `247` validated title-sorted cache rows |
| Locale | same-snapshot LC_CTYPE LCID `0x0409` / 1033 and code page 1252 |
| Live systems | `100` system cache rows and `100` iterator-returned rows; family `0x90`: `30`, family `0x92`: `70`; no iterator exclusion, multiplicity difference, or canonical-key collision in this snapshot |
| Order boundary | iterator order and title-sorted cache order are not equal: `98` of `100` positions differ; neither is called registry order |
| Read budget | `10,084` fixed reads / `42,336` bytes; at most six bytes per read, below the shared `750,000` call / 16 MiB caps |
| Reader artifact | `live-cache-run-1-reader.json`, SHA-256 `aeb4c1140f747571d63d18bb4e3c68fd37b14f3527d3023888e45bb7d76cafd3` |
| Context artifact | `live-cache-run-1-context.json`, SHA-256 `2e1dfe9aaad117104358d2a05f2b9143adb9d6e7660a03c56a1c4d92953dea6d` |
| Metadata-only summary | `live-cache-run-1-summary.json`, SHA-256 `863f75bd2bb5fc2795570bd0f1cac38e6634b5fa527f141804f7bd8f2b754630` |

The actual full-cache identity/key sequence, live iterator identity/key
sequence, candidate dispositions, ancestry types and family counts are retained
as integer metadata in the reader artifact. Their canonical sequence hashes
are:

- full cache:
  `ecc114bc73c0cfdf496d1bd0eb6b03a89d4950284d9b73e06f4b60d13f1e7c1a`;
- cache system subset:
  `f586d142009c70b12c8a66b9368d2987cf31fadcaab54f065f58faa1d569051a`;
- iterator-returned systems:
  `67250e7ba7bdfcd50b86575029aa7e5349b70a903d656b27664cae5b785392dd`.

The supporting unscaled client frame visibly shows the encyclopedia index,
while the full-desktop frame records the compatibility environment. Their
SHA-256 values are respectively
`001294068c39923c19e51059a2391135dc75432bac72119187ea9774d425febe`
and
`176f437f2750edc34dd971194b586ea3a969893c6339254b543a3926934a3773`.
Those pixels are supporting state context only. Cache existence, membership
and order come from the connected roots, vtables, list invariants and two equal
snapshots—not the screenshot or timed input.

No title/body/string bytes, process-memory writes, injection or broad dump were
used. This first raw-viewer observation did not by itself establish the other
viewer, registry order, a full byte-fold map, tie behavior beyond the captured
list, or the remaining UI matrix. The 200 static SYSTEMSD rows remain source
inventory; this run proves only its own 100 live system-cache/iterator
identities.

### Other-viewer bounded observation

Run `E51-E09-live-cache-2-empire` was a fresh process and fresh Standard Game;
there is no same-campaign claim relative to run 1. Before the
reader was admitted, three full/client pairs were captured and individually
inspected: the menu visibly showed `Standard Game` and the left green Imperial
selection control, the next checkpoint visibly showed the Empire strategy
shell, and the final checkpoint visibly showed the Empire encyclopedia open.
No galaxy or scenario size was visibly labeled in the observed setup frame, so
none is inferred from the 100 observed systems. These pixels establish only
the human setup/action context. The raw viewer selector and cache contents are
separate process facts from the source-connected reader.

| Item | Observed value |
|---|---|
| Process | PID `3269886`; `/proc` start `2026-09-29T13:42:44.670Z`; exact executable SHA-256 `b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab` |
| Setup context | fresh `Standard Game`; visible left Imperial control selected; Empire strategy shell and encyclopedia inspected; galaxy/scenario size not visibly labeled |
| Stability | two complete canonical metadata snapshots, identical SHA-256 `b250b7f486906f577a7c4f97121f405af1a632a91c0909f44fc7294166928cfc`; process identity and before/after sentinels equal |
| Shell/cache | raw viewer selector `2`; source-connected nonnull shell/cache with reviewed vtables; mode `2`; `247` validated title-sorted cache rows |
| Locale | same-snapshot LC_CTYPE LCID `0x0409` / 1033 and code page 1252 |
| Live systems | `100` system cache rows and `100` iterator-returned rows; family `0x90`: `30`, family `0x92`: `70`; no iterator exclusion, multiplicity difference, or canonical-key collision in this snapshot |
| Order boundary | iterator order and title-sorted cache order are not equal: `98` of `100` positions differ; neither is called registry order |
| Read budget | `10,068` fixed reads / `42,272` bytes; at most six bytes per read, below the shared `750,000` call / 16 MiB caps |
| Reader artifact | `live-cache-run-2-empire-reader.json`, SHA-256 `4c3db5cdf85840ccbf85df1fe7ad537f631e838bf660daeead61dc416ce76fa1` |
| Context artifact | `live-cache-run-2-empire-context.json`, SHA-256 `0771654552d158a8ce444d81befdb94efdd56d0259a9e0cad2fdb6d40c2c03c6` |
| Metadata-only summary | `live-cache-run-2-empire-summary.json`, SHA-256 `1c04afca3b7f2c9e93a2dbf53fa36e604277ad9574362e68a1900ecbf666864b` |

The full-cache, cache-System and iterator sequences in this fresh raw-selector
2 run have the same canonical hashes as run 1:
`ecc114bc73c0cfdf496d1bd0eb6b03a89d4950284d9b73e06f4b60d13f1e7c1a`,
`f586d142009c70b12c8a66b9368d2987cf31fadcaab54f065f58faa1d569051a`
and
`67250e7ba7bdfcd50b86575029aa7e5349b70a903d656b27664cae5b785392dd`,
respectively. Their exact identity/key sequences and multisets are equal across
these two observations. That equality is deliberately bounded to these two
fresh runs: it does not establish a universal viewer-independent admission
rule, a shared generated campaign, registry order, or admission for all 200
static candidates.

The supporting run-2 frame hashes are `114d055c...656e8` and
`b2858ffd...3851` for the menu, `9e6985e0...0ae2` and
`a5b19fee...8f0` for the strategy shell, and `c8bd4741...115f` and
`ba16ce53...3b0e` for the encyclopedia (full desktop then native client in
each pair). Full hashes live in the retained context and summary. The game
process was stopped after the observation; authenticated display `:91` remains
available. The same no-write, no-injection, no-localized-text and bounded-read
rules used for run 1 remained enforced.

Coordinator review message `21469` corrected one metadata label without
changing either immutable raw reader artifact. Their iterator objects use the
historical key `picture_selector`, but the value is definition `+0x30`'s
topic/text selector used to derive `canonical_key` as low 12 bits plus
`0x1000`. It is not the separate System image selector 1, 24, 25 or 26. The
superseding qualification is
`live-cache-selector-field-provenance-r6.json`, SHA-256
`841e800d8d8112bd1c4ca5da6b97b6614d34855c13046d66180f7fae7d71d7b7`.
Future reader output and tests call the field `topic_selector`; neither raw
value, identity sequence, canonical key nor matrix status changed.

## Handoff and change control

- Preserve the manifest and isolated working prefix for coordinator review.
- A future capture must add evidence rather than overwrite prior attempts, and
  must identify its executable hash, compatibility/native environment, exact
  setup, action, state identity, and screenshot hashes.
- `EDATA.192` and all unproven alternate-art behavior remain outside this
  matrix under deferred task `orlocal-2kq`.
- These observations do not prove the original comparator for unseen rows,
  Windows-rendering parity, browser/native port parity, or final UI acceptance.
- A contradiction with the source contract reopens source/schema review; a
  screenshot must not silently replace a connected identity or predicate.
