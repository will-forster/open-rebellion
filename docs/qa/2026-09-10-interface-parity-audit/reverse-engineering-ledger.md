---
title: "Interface Reverse-Engineering Ledger"
description: "Executable and original-resource evidence queue for closing the original interface parity audit"
category: qa
created: 2026-09-10
updated: 2026-09-14
tags: [qa, interface, ghidra, resources, bitmap, wasm]
---

# Interface Reverse-Engineering Ledger

This ledger connects the [surface ledger](surface-ledger.json) to the original
executable and game resources. It tracks questions that static analysis can
answer without duplicating the screenshot inventory. The machine-readable queue
is in [reverse-engineering-ledger.json](reverse-engineering-ledger.json).

## Current answer

Ghidra and the owned original files can recover substantially more of the
missing interface contract. They can establish exact rectangles, control and
resource IDs, button state selection, command routing, window constructors,
display predicates, animation action tables, embedded audio, tactical event
routing, multiplayer timers, and error paths.

They cannot by themselves prove final palette output, compositing, animation
cadence, live two-peer behavior, or browser equality. Those cells still require
lossless captures from the original executable followed by native and browser
comparison.

## Proven strategic foundation

| Evidence | Static result | Surfaces |
|---|---|---|
| `REBEXE.EXE` `FUN_00421c70` | Faction-specific galaxy apertures and all twelve window-reference rectangles | CMD-01, CMD-04 |
| `FUN_00427270` | Original control positions, sizes, command IDs, and paired bitmap resource IDs | CMD-10 |
| `FUN_006028c0`, `FUN_00602d30`, `FUN_006030c0`, `FUN_006030f0`, and `FUN_006035f0` | Normal and captured-press paint states, pointer capture, release cancellation, dispatch, and disabled flags | CMD-10 |
| `FUN_005fca00`, `FUN_005fd170`, and `FUN_005fc140` | Strict four-edge rejection, bottom-left palette-key hit mask, natural-size paint, and control-window clipping | CMD-10 |
| `FUN_00422ce0` and freshly recovered `FUN_00429020` | `WM_COMMAND`, double-click routing, exact rail hit testing, child-window focus, and rail removal | CMD-03, CMD-04 |
| `FUN_00427010`, `FUN_00425d00`, `FUN_00426d00`, `FUN_00426e70`, `FUN_00426ee0`, `FUN_00427270`, and `FUN_0042b330` | Active 903 versus Display Off 902, GID captions, exact faction control, compact and expanded legends, native marker families, Popular Support thresholds, and floating 180×240 display-window path | CMD-02 |
| `FUN_00427270` and `FUN_00422ce0` | Nine Message Index rail controls per faction, exact 27x22 rectangles, resting and illuminated resource pairs, command IDs `0x136..0x13e`, and dispatch to `FUN_0042a240` | CMD-08 |
| `FUN_0044f670`, `FUN_00442d70`, and `FUN_0044c410` | Code-built, modeless object windows using GOKRES rather than an invented sidebar | CMD-03, OBJ-02–OBJ-14 |
| `FUN_0042d650` and freshly recovered `FUN_0042adb0` | Faction advisor/briefing DLL selection and exact advisor apertures | PRE-05, CMD-07 |

P46A implements the centered 640x480 strategic canvas, the 640x481 source
crop, both faction galaxy apertures, and one shared map, hit-test, blockade,
and advisor transform. Native tests and packaged-browser viewport checks
corroborate this shell checkpoint. UIP-B01 now implements the faction-specific
twelve-slot rail geometry and first focus, close, minimize, restore, and
eviction lifecycle. Exact active and inactive rail thumbnails plus the full
multiwindow matrix remain open. See the
[P46A evidence](evidence/2026-09-11-strategic-shell-canvas.md).

P46B restores the six primary faction controls from the recovered constructor,
paint, hit-test, capture, and `WM_COMMAND` paths. It removes the replacement
text strip and renders the exact normal and captured-press bitmap pairs. PR #11
corrects `0x131` to Encyclopedia, `0x132` to GID, and the side-globe `0x133` to
Game Options for both factions. F1 and F7 match the native destinations. GID
opens and closes its original menu, while Game Options and Encyclopedia fail
closed until their original bitmap windows exist. Full A0 captures, the
disabled path, destination compositions, and speed controls remain open. See
the [P46B evidence](evidence/2026-09-11-strategic-command-controls.md) and
[routing correction](evidence/2026-09-14-cockpit-routing-correction.md).

The first UIP-B01 checkpoint also replaces the invented sidebar with recovered
235x360 sector and 226x304 detailed-system shells. Original planet pictures,
relationship title art, six tab resource families, single and double-click
routing, pointer occlusion, and rail transitions work in both faction shells.
P46D maps characters, fleet representatives, facilities, regiments, special
forces, mines, and refineries to their GOKRES miniatures. It also restores the
three-column item viewport, STRATEGY `10365` through `10369` scrollbar art,
selection, displayed labels, bounded input, and coarse current-intelligence
gating for opposing objects. Nested compositions, exact intelligence rules,
drag and command semantics, uncommon states, and the complete A0 matrix remain open. See the
[navigation evidence](evidence/2026-09-11-strategic-window-navigation.md) and
[tab-item evidence](evidence/2026-09-11-detailed-system-tab-items.md).

P49 restores the eighteen source-mapped Message Index rail resting BMPs. Both
faction rails match their original 8-bit resources pixel-for-pixel at 640x480.
The illuminated predicate and original Message Index destination remain open.
See the [rail evidence](evidence/2026-09-12-message-index-rail.md).

P60 recovers and implements the Game Speed control. The day readout opens a
five-item STRATEGY menu (`FUN_0042d190`), and the rates come from
`FUN_00487eb0`. Pause sets a stop day one day ahead (`FUN_0041d2f0`,
`FUN_0041e290`) and opens the REBDLOG alert (`FUN_00417020`), and
TEXTCOMM accelerator table 11 maps the speed keys. The same pass recovers the
Message Index window (`FUN_0042a240`, `FUN_00466350`), its category mapping,
the rail resting path, and the Advice Very Slow drop (`FUN_00487ff0`). The
rail illumination-on path is not yet recovered. See the
[game-speed recovery](evidence/2026-09-24-game-speed-recovery.md) and
[Message Index recovery](evidence/2026-09-24-message-index-recovery.md).

P47A identifies STRATEGY 900 and 901 as the faction shells, with 902 and 903 as
the bright and dim galaxy resources. P47B proves 902 belongs to Display Off and
903 to every active GID mode, then restores the default Popular Support caption,
exact faction GID control, compact 10168 legend, native 10146 through 10158
marker families, and support-size thresholds. It also establishes that the
nine tall side controls are Message Index categories, not GID filters. P48
maps the code-built GID command tree. P50 restores the STRATEGY 10100 through
10107 menu-frame tiles with instant display and source-pixel browser checks.
P51 withholds an unsupported hover wash and verifies frame pixels still visible
beneath the detailed system window.
The menu interior, typography, exact geometry, expanded 180x240 legend,
remaining modes and overlays, and exact map input remain open. See the
[galaxy-backdrop evidence](evidence/2026-09-11-authored-galaxy-backdrop.md) and
[Popular Support evidence](evidence/2026-09-11-popular-support-gid.md), plus the
[P50 frame record](evidence/2026-09-12-gid-menu-frame.md) and
[P51 overlap record](evidence/2026-09-12-gid-hover-and-occlusion.md).

The read-only Ghidra pass also recovered six formerly empty high-priority UI
exports and the `CoolStrobeButton` paint/input path. The text export is not a
complete interface corpus: of 4,934 canonical `FUN_????????.c` files, 2,790 are
zero-byte placeholders. New interface work must query the saved Ghidra project
when a required export is empty.

## Original resource truth

Runtime pack v2 contains 52 game-data entries, 2,303 standard BMPs, all 3,988
ALSPRITE and EMSPRITE type-302 frames, and 118 audio files: two music cues,
four menu effects, 22 tactical weapon effects, and 90 tactical command voices.
The owned
installation contains important additional families that are not yet staged or
packed:

| Original module | Additional authentic content currently omitted |
|---|---|
| ALSPRITE | 752 BIN controls and 213 WAVs; its 38 BMPs and 1,640 type-302 frames are staged |
| EMSPRITE | 753 BIN controls and 216 WAVs; its 34 BMPs and 2,348 type-302 frames are staged |
| ALBRIEF | 366 BIN controls, 2,684 type-302 frames, 17 WAVs |
| EMBRIEF | 471 BIN controls, 2,738 type-302 frames, 22 WAVs |
| TACTICAL | 87 type-301 DirectX meshes, 397 type-303 textures, and 44 WAVs beyond the 22 staged weapon variants |
| VOICEFXA and VOICEFXE | 195 WAVs beyond the 90 staged battle-ready and group-command recordings |
| STRATEGY | 96 RCDATA resources and 66 WAVs beyond its staged BMPs |
| REBDLOG and TEXTCOMM | Original dialog chrome, text, templates, and accelerators |
| EData and ENCYTEXT | 187 400×200 entity images and 348 descriptions |
| MDATA | 15 Smacker films and the remaining original music cues |

The droid path is now concrete. `C3POACT.SPT` and `IMP22ACT.SPT` map logical
actions to BIN resource IDs, which in turn reference standard bitmap anchors and
type-302 frame runs. The current sorted-filename thirds and modulo frame mapping
in `advisor.rs` are provisional and cannot pass parity.

## Work queue

The queue contains 27 bounded packages. Every one of the 43 required surface
families links to at least one package with named sources, a retrieval method,
and a next proof in the [machine-readable ledger](reverse-engineering-ledger.json).

| Package | Scope | Status | Next proof |
|---|---|---|---|
| RE-FE-01 | Boot, introduction, and credits routing | static-partial | Map media selection and every completion, failure, skip, and return callback |
| RE-MENU-01 | Shuttle controls and destinations | static-proven | Capture remaining original interaction and edge-probe states |
| RE-OPT-01 | Unified options, save, load, and delete | static-partial | Constructor, controls and confirmation resources mapped in [Game Options checkpoint](evidence/2026-09-24-game-options.md); exact slider/context/persistence and A0 evidence remain pending |
| RE-STR-01 | Shell, apertures, and reference rail | static-proven; shell and first rail lifecycle runtime-corroborated | Replace provisional rail thumbnails, exercise the full multiwindow matrix, then compare against A0 captures |
| RE-STR-02 | Cockpit controls, states, input, and command routing | static-partial; primary controls implemented | Capture the six primary controls in A0, implement original destinations, then recover speed and facility-indicator predicates |
| RE-GID-01 | Filters, legends, marker rules, pan, zoom, and selection | static-partial; default GID and source frame runtime-corroborated | Recover menu interior and geometry, expanded legend, remaining filter predicates and overlays, and exact map input |
| RE-OVR-01 | Galaxy Overview | untriaged | Recover geometry, category formulas, resources, and destinations |
| RE-MSG-01 | Messages, Agent menus, alerts, and reports | static-partial; rail and empty index shell runtime-corroborated | Continue from the corrected [P63 Message Index mapping](../../../ghidra/notes/message-index-window.md): populate rows, selection, navigation, clear/delete, Advice slowdown, chat, production routing, and A0 comparison |
| RE-ENC-01 | Encyclopedia index and topics | semantic and UI/control static recovery complete; exact empty index shells, ten control pairs, and source-derived English catalog runtime-corroborated; original Wine compatibility captures partial/blocked with bounded endpoint/current-row, Fleet-context, mission-context, and Build Selection class-context evidence; unproven alternate art deferred to `orlocal-2kq` | Continue from the [P65 catalog checkpoint](evidence/2026-09-30-encyclopedia-index-catalog.md) and preserve the [P63 mapping](../../../ghidra/notes/encyclopedia-window.md): bind 356 catalog objects to ENCYTEXT and EDATA; complete E51's named partial/missing `ENC-UI-01..21` subcases with selected resources, internal focus, notifications, the two unrenamed caller routes, entity/faction/context fixtures and complete desktop pairs; complete topic composition and integrate production routing |
| RE-OBJ-01 | System, sector, and object-window constructors | static-partial; sector and system shells runtime-corroborated | Complete system item compositions and commands, then map the remaining object families |
| RE-ADV-01 | Type-302 advisor and briefing frame decoding | runtime-corroborated | Extend the verified advisor decoder/transport to briefings and compare with A0 captures |
| RE-ADV-02 | SPT/BIN/FDT action semantics, cadence, and sound | static-partial | Replace inferred priority thirds with authored action mappings |
| RE-PACK-01 | Complete native/WASM resource transport | implementation-needed | Version the pack for arbitrary resources, films, and EData |
| RE-MSN-01 | Create Mission and Mission Status composites | untriaged | Trace constructors, legal-target predicates, and outcome routing |
| RE-EVT-01 | Strategic events and authored reports | static-partial | Resolve every common, faction, and rare event variant |
| RE-BAT-01 | Battle Alert, strategic reports, and results | static-partial | Trace state setters and callers into constructors, choices, force tabs, results, media callbacks, and return routing |
| RE-GND-01 | Original ground-assault presentation | static-partial | Prove report-only flow and remove the invented live-combat route |
| RE-TAC-01 | Tactical loader, control tree, and event-handler registry | static-proven | Join subordinate control vtables and event slots to exact rectangles, predicates, handlers, and observable transitions |
| RE-TAC-02 | Tactical control geometry and resource-state selection | static-partial; bounded practical launcher complete at P58-B22 | Follow the [ranked recovery map](../../reference/space-battle-launcher/reverse-engineering-map.md) through whole-process RNG continuity, strategic commander binding, native beam/playback behavior, rare audio callers, remaining controls, and A0 comparison |
| RE-TAC-03 | Tactical battle-results composition | static-partial | Connect state setters, result construction, canonical application, reports, media, and strategic return |
| RE-DS-01 | Strategic Destroy System and sabotage paths | static-partial | Resolve confirmation, report, and family `0x34` predicates |
| RE-DS-02 | Tactical Death Star and trench-run routing | static-partial; exact producer, timer, chatter, casualties, and result-to-film dispatch implemented | Bind the strategic commander slot, compare native beam/playback/audio timing and return behavior, and close A0 |
| RE-END-01 | Campaign endings, skip, return, restart, and failure | static-partial | Recover the complete terminal media matrix |
| RE-NET-01 | Original multiplayer screens and controls | static-partial | Finish template 10100–10103 geometry and provider/host/join routing |
| RE-NET-02 | Two-peer sync, chat, pause, saves, departure, and errors | runtime-needed | Run an original two-peer fixture and compare protocol traces |
| RE-EXT-01 | Remove visible replacement dashboards | implementation-needed | Preserve every action inside original paths with zero replacement pixels |
| RE-A0-01 | Lossless original-executable baselines | runtime-needed | Capture every required surface-state cell at native 640×480 |

### RE-ENC-01 semantic checkpoint

The [source contract](../../reference/asset-library/encyclopedia-source-contract.md#semantic-research-checkpoint)
records the profile-bounded decision table and the machine-readable ledger holds
the same testable scenario IDs. Static recovery now proves:

- selector order `0x6f`, `0x70`, `0x71`, `0x72`, `0x73`, `0x74`, `0x75`
  from `FUN_0045ddc0` and the exact `FUN_0045f100` early-return/forced
  transitions; `0x6f` binds shell `+0x474`, it does not clear a list;
- shell `+0x474` is an if-null, once-per-shell master cache built by
  `FUN_00422620` from registry definitions and a viewer-side system iterator;
  category changes filter that retained cache rather than reading a fresh
  campaign snapshot;
- both master and filtered collections use `FUN_0060a790(..., 2)` and
  `FUN_005f59f0`, so `FUN_0060a890`/`FUN_00626ad0` impose case-insensitive
  narrow-byte order with stable source-order ties;
- class and entity contexts route through definition `+0x30` low12 + `0x1000`,
  including the connected `FUN_0045fd90` entity fallback, rather than treating
  raw entity identity as topic identity;
- the viewer-side system iterator selects a type-`0x90` view and calls its
  vtable `+0x10` predicate, `FUN_004f6330`; that function walks the view's
  `+0x1c` container ancestry and causes `FUN_0053f090` to exclude the view
  exactly when an ancestor's virtual type is `0xf2`;
- row vtable functions `FUN_004ad730`/`FUN_004ad750` skip disabled neighbors,
  preserve non-wrapping endpoints, and operate over the sorted bound list; and
- the bounded viewer-side and system-picture EData key selectors in
  `FUN_0045fa60`.

The package remains `static-partial` and `runtime_capture_required`. The system
predicate is structurally recovered, but source type `0xf2` remains deliberately
unnamed: `FUN_005696b0` installs vtable `0x006639b8`, whose `+4` method
`FUN_00569880` returns `0xf2`; that proves the exact ancestry test, not a
friendlier knowledge, destruction, or visibility label. For the inspected
source profile, Luke key `0x1842` maps to `EDATA.074`, `0x2842` is empty, and
ENCYBMAP has no `EDATA.192` mapping; recovered REBEXE static evidence includes
the `EDATA\` directory-literal reference inside `FUN_0045f7b0` and the identified
table-selector loader callers, but does not establish a connected selector or
predicate. `EDATA.192` therefore remains inventory-only under the current
publication policy and no alternate-Luke predicate, runtime binding, or UI
switch is published. The user-approved scope defers that research to
`orlocal-2kq`; the asset remains inventoried but unused and does not block E08,
the first-profile schema, or publication. This is not proof that original
behavior is impossible or that the alternate was implemented. Static asset
presence does not imply gameplay visibility, and no expression or mod-supplied
predicate may substitute for future proof.

The repository ledger validator continues to validate the shared ledger shape,
surface coverage, and required package fields; it does not inspect the nested
`semantic_research` contract. E08's retained ignored semantic checker separately
validates all 14 rule records, all 25 named decision scenarios, their references
and required fields, the recovered selector transitions, cache/comparator
contract, canonical context key, exact type-`0xf2` system predicate, and bounded
alternate evidence state: inventoried, unused, deferred to `orlocal-2kq`, with
no predicate or runtime binding.

The [original UI contract](../../reference/asset-library/encyclopedia-ui-contract.md)
adds the E27 source checkpoint without changing those 14 semantic rules or 25
scenarios. `FUN_00429f30` constructs at most one shell child `0x19` through
`FUN_0045d400` with a 470 × 330 client. `FUN_0045ddc0` supplies the exact
faction chrome, category and mode controls, header/index/list/body rectangles,
navigation controls, and `STRATEGY.DLL` resource-state IDs. It selects the
faction shell/rail first, then creates both mode composites from that same
pair: topic background `1` uses shared overlay `0x2861` at `(12,14)`, while
index background `2` uses shared overlay `0x2862` at `(12,13)` and has static
text resource `0x1843` created at `(36,48)` with zero initial extent, measured
by `FUN_00601b80` through `DrawTextA` flag `0x400`, and rendered into it before
registration.

`FUN_0045f480` selects `2` for index and `1` for topic; `0x1843` is proven as
baked static text but is not assigned a guessed semantic label. The connected
`CoolStrobeButton` path proves normal, captured-press, disabled, and selected
bitmap slots; it exposes no separate hover-resource transition.

The missing checked-in function bodies were bounded directly against the same
identified `REBEXE.EXE`: `0x0041d6b0` obtains the active shell and forwards
typed context to `FUN_00429f30`, while `0x0045da70` dispatches close `0xfb`,
mode container `0x66`, category container `0x6e`, list `0x65`, and navigation
`0x83`/`0x84`. Context-free shell paths are F7 (`0x76`) and command `0x131`.
An exhaustive direct-call scan proves five contextual callers into
`FUN_0041d6b0`: unrenamed handlers `0x00438800` command `0x67`, `0x004443a0`
command `0x66`, and `0x00467f10` commands `0x67`/`0x97` with selected-child
type `4`; `FUN_0046c3c0` command `0x67`; and `FUN_00486fb0` event `0x100`.
Their selected-row, guarded retained-context, selected-child, mission-row, and
first-row flows are recorded by address. No visible surface name is inferred
from proximity.

Accepted r45 evidence proves the mission caller's exact class-context route:
`FUN_0046c3c0` command `0x67` copies the selected mission-definition row into
a class wrapper, pairs it with an empty entity companion, and calls
`FUN_0041d6b0`. `FUN_00429f30` passes those wrappers into `FUN_0045d400` only
on child construction; an existing encyclopedia child is reused without
reconstructing context. Accepted r46 evidence proves the distinct conditional
setup route: TEXTCOMM accelerator table 11 maps Alt+M to `0xbc0`; the root
loop forwards it to the active child as `0x483`; a valid selected live
Personnel entity may build order `0x240`; and only a nonempty
`FUN_004f5380` legal-mission result admits the mission dialog. The live entity
and later mission-definition class row are not interchangeable identities.

Accepted r49–r52 evidence identifies `0x00438800` as the ordinary Build
Selection dialog's class-context handler. Its complete command-`0x67` table
branch reaches the selected row `+0x54`, supplies a class wrapper plus empty
entity companion, and reaches `FUN_0041d6b0` / `FUN_00429f30`; only the create
branch supplies those wrappers to `FUN_0045d400`. The r50/r51 independent
reviews rejected type-only instance identity. The corrected conclusion is
conditional: for the ordinary fresh-created collection on the key-check/equal
edge, range `0049e130–0049e196` (SHA-256
`2e279e7bf11780e1c28ecfd66dbd02329794b35bfb813969456ae43ab88dcc70`)
returns an exact same-collection node and trampoline `00568ee0–00568ee5`
(SHA-256
`ad754bd90725bde9c206591c56972f8678048a6af3b94433e5587eb7acd98c1e`)
reaches the exact-node leftmost fallback. The `0x00439ae4` bypass,
duplicate-key or malformed serialized state, and already-open-window retarget
are outside that result.

Index focus belongs to the `CoolDragList`; Return emits the same `0x309`
activation as a double-click and enters topic mode. In index mode Left uses
`FUN_005f5c60` predecessor traversal and Right uses the threaded successor;
both skip hidden non-null candidates. An immediately null candidate falls back
to the tree's leftmost child through `FUN_005f5060`, so Left at the first child
reselects it and Right at the last child wraps to it. Running off an edge only
after hidden candidates retains the current category. The immediate fallback
does not visibility-test the first child. `FUN_0060d7e0` sends the category
command, whose branch at `0x0045dad0` explicitly focuses the list after
rebuilding/binding it. Topic focus belongs to the read-only `TextScrollField`;
Left/Right navigate topics without wrapping, Up/Down/Page keys scroll, and
Return is forwarded with no encyclopedia-local topic action. Body measurement
uses `DrawTextA` flags `0x2410`, source word/newline/tab handling, and a
conditional scrollbar. The apparent ship statistics in A3 guide capture 027
travel through the ordinary `ENCYTEXT.DLL` body-string path: no live-stat
control or world-field binding is present in the recovered constructor,
mode/render, and text-population paths.

The package deliberately remains `static-partial` and
`runtime_capture_required`. E51 supplies the bounded
[original capture set](../../reference/asset-library/encyclopedia-original-captures.md)
for the identified executable under Ubuntu Wine 9 on authenticated Xvfb
`:91`. Its original manifest retains 198 hashed PNGs and cites 60 paired
full-desktop/native-client states. Acquisition and acceptance are separate: 15 rows have partial
acquisition, six lack a direct acquisition, and all 21 acceptances remain
open. Both faction index shells, all seven visible command states, F7 and
shell-command behavior, duplicate-F7 visible-child behavior,
click/double-click/Return pixels, selected control states, filtered endpoints,
a real body-scroll change, visible close results, and visible category edge
behavior are represented. E51 r7 separately adds bounded current-row evidence
for full and filtered Left/Right endpoints, an ordinary list PageDown/Return,
pointer body scrolling, Escape child absence, and Alliance System selectors
1/24/25/26. These observations still do not establish complete membership or
runtime-admitted comparator order, empty/gap outcomes, delivered notification
identities, direct loader filename selection, internal Win32 focus, the direct
`FUN_004fcee0` value, or duplicate-F7 internal child count.

Coordinator-verified r41 additions are append-only and are not folded into the
198-PNG baseline: 75 raw full frames plus 75 exact same-acquisition crops have
sorted identity digest
`36f046e7c9d9672887df3027827811ed5605a0876365b34726d2f6539c55d28e`.
They visibly establish Chewbacca scroll onset on discrete Down 4 and reversal
on Up 4, with Up 5-8 rejected after a Message Index interruption; a separate
down-arrow click moved immediately while an attempted pointer drag did not.
They also show `Corellian Corvette 1` inside expanded Fleet 1 opening the
`Corellian Corvette` article and Close preserving Fleet 1. Those pixels do not
prove internal focus, notifications, selected resource identities, or the
exact owning typed wrapper. The r41 report/checker hashes are
`60fa5c5ae076c07360b7cc0a97185181fa34691357c8b1d23deda997ade9abf8`
and `0ff4fcb8e3346d4bea002a53caf7b741b9fdf8e518bd05355eae22fc92f362ba`.

The independently reviewed r42 static correction (report/source/checker hashes
`3c955d02f686b004ffa143ef21774f6033c7eaf69d1f35ab06da430d93ce455b`,
`9147007d8bff711d9d78b37589f92b11ab576ee53f7d91db7d7aa95ce68ea7c0`,
and `9b53a58dada0265dd5d2841964e2cd440336aa6ba0942cb1ecd792808b0ae68e`)
proves the `0x29b`/`0x29d`/`0x309` emitter paths, conditional non-null
`0x309 -> 0x67` receiver, command `0xfb`'s same-vtable-`+0x30` close route as
Escape, and null-neighbor retention for `0x83`/`0x84`. It also corrects the
master cache lifecycle (`FUN_00421c70` initializes shell `+0x474` null;
`FUN_00422620` allocates/populates when null) and body/title ordering
(`FUN_0045fa60` uses row `+0x0c` for body/art and reads row `+0x14` only after
body layout). These are static facts, not retained runtime event/resource
observations.

The independently reviewed r43 trace connects the r41 Fleet ship route through
the typed Fleet owner, selected-row wrapper collection, popup command `0x100`,
`FUN_00486fb0`, `FUN_0041d6b0`, and `FUN_00429f30` (report/source/checker/review
hashes `fd32ddc0...24cc9`, `85a7a02d...b51e9`,
`8ed791fd...fd95`, and `e40d6772...748a`). On the create branch,
`FUN_00429f30` passes the contextual wrappers to `FUN_0045d400`; an existing
window is reused. Deselect matches the wrapper's same packed/base identity,
not a proven discriminator match. The visible Corvette result plus static
route does not prove a live command occurrence, selected resources, canonical
DAT identity, notification delivery, or target-thread focus.

The r45 report/source/checker/review hashes are
`4621912fa106bfd8f5ba433c8a4fdb4ac7d33a1c1d08fd00b590b659a10c2dbe`,
`7fb3b40c4d0288646d7c6b8e23adf3a0d778923b979b185259a1ac08c4ede520`,
`207f78b59f72e9b48e1a4888611f34502e94d525da5f6de4de11c1a8c016f68c`,
and `a0d746f21ff00227d2a0eed8d77ea0943aea22f731f2acc5dbac0f529fdae3bb`.
The r46 report/source/checker/review hashes are
`166d66f446039b8ddf96bb5cd50827f28e77ba3a127a72bf67355fe712ddbd96`,
`1dd786bf12e9f02378e26b5a14329a09266e2ec5cc10a7538a58971027169837`,
`8d7468b75db9b43c5071398a04bcfca392229cef0953955105f0ba86afc4fecb`,
and `7f114fcb62e16b39a6ded1146a047c8085076c6d625eb73d453f80b8d77d4e81`.
Their accepted scope is static source evidence, not a runtime
command/resource/notification/focus observation.

Coordinator-accepted r47 adds 70 raw full frames and 70 exact
same-acquisition crops outside every earlier inventory (sorted digests
`8fd989a44bd4fa5f8fd281da7437ef267a687b1c315080170801554bd128e631`
and `9666c639bf6528e99c6014bc19077c72c4bf3834614e56b2bc7badf9697444e1`).
In one Alliance / Sumitra / Yavin
run, real Recruitment and Espionage mission-definition rows each opened the
matching article through the visible mission-dialog Encyclopedia control, and
ordinary close retained the mission and Personnel windows. Report/checker/
action-journal hashes are
`538a0d3f5b6e71b361e0f9b37031fb8255bc1b3634077dd4255e6358a532acdf`,
`1c75f64daf9296546fe826b670af0ffedc711e68663402b352d5726356bd9d89`,
and `de324268a33f48bf6c4901f68c3755fe0ba9d233587c8beeed61651f2a91fb7c`.
The Alt+M bracket frames `055`/`056` are byte-identical
(`8ff85f6dcfffed2c73b8bedc121140981dddf5e942f24f81faa08c24f861f97b`);
a separate later double-click precedes dialog frame `057`
(`802b2e74d4963881a3af120ea84c14d732753a680aa16809c9c03bf7a31024c5`).
Thus the downstream class-context result is observed,
while hotkey causality remains unisolated. Cleanup removed the owned
game/window after exact prefix stop, but the supervisor exit 143 and unknown
launcher exit status remain recorded rather than normalized.

Coordinator-accepted r53 adds 35 raw full frames and 35 exact
same-acquisition crops under `visible-r53-1/`. Its report/checker/manifest/
action-journal hashes are
`596b732d5925632a6a855ce35252592dfdb609706ecd902a977b867004b101d4`,
`d05db1e6d73babf700c5e6276049e84f399e6ac42f7e5a36d499c9016dd29cd8`,
`6b0fe05f26222a4f88a69637900680403ffc0cb634cb1896d4f07647dde73dbe`,
and `baefeb1e52ca31d4ca67e1de792cdbea8b3e9e522144db476b4c2769ca82274c`.
In one Alliance / Chandrila run, the first post-shortcut frame showed no
candidate dialog; a later visible yard click opened Build Selection. Selected
Alliance Escort Carrier opened the matching article and ordinary encyclopedia
close retained Build Selection and Chandrila Shipyards. This is visible
class-context evidence only, not live Alt+B causality, command/notification,
canonical key/DatId, selected resource, or target-thread-focus proof. Launcher
and exact-prefix stop returned zero; the launcher was reaped; the guard was
cancelled and its PID is absent. The guard wait/reap exit status remains
unknown, and lifecycle-shell exit `143` is not substituted for it. The r35
quarantine remains authoritative.

The prior r44 four-file metadata state remains a historical checkpoint at
`1c3140059d8bd69d94715e47c907667c0fe29801`; its checker and hashes are not
relabeled as current. The additive r48 checker (`check-r48-metadata.py`,
SHA-256
`091e4130d991dd2798305a27e77d20634724b6834aac29500d93743ccc02bd4d`)
validates the old Git objects and the current four-file/evidence/row closure
separately.

The remaining gates are named rather than inferred: runtime command/resource/
current-key/focus proof for the observed Fleet, mission, and build class
routes; the two unrenamed contextual owners; all controlled entity-context routes;
fallback and unavailable contexts; the admitted /
excluded type-`0xf2` ancestry pair, Empire-side System-selector counterparts,
exhaustive control states, full keyboard/body-layout cases, and connected
hidden-category configurations. `ENC-UI-17` is corpus-not-applicable for this
supported profile: all 347 bound rows have effective art (331 direct/system
plus 16 complete viewer-faction pairs) across 186 bound files. The generic
null-art branch remains untested; no fictional original topic or E26 mod-null
fixture is required. Unused lookup IDs `7188`/`11284` must not be confused
with files `EDATA.142`/`EDATA.143`, which are bound through lookup IDs
`7200`/`11296`. The unchanged 15 partial / 6 missing / 0 accepted rollup is a
historical acquisition/acceptance count, not an applicability count. The same-topic faction pair is
visual evidence only; it did not instrument selected image identities. The
compatibility capture does not establish original-Windows rendering parity,
port parity, or final acceptance. Guide captures 018, 019, and 027 remain A3
leads only. Any A0 contradiction must reopen source/schema review before
consumers change.

The smallest source-only next investigation is one finite owner trace for
`0x004443a0` or `0x00467f10`; it needs no new runtime access. Correlating any
of the three visible class routes with live command, notification, resource,
current-key, or target-thread focus needs a separately reviewed non-invasive
observer, and r35 quarantine is a genuine gate rather than reusable authority.
Exceptional fallback/unavailable/hidden cases need a legitimate owned
save/scenario or connected source fixture. Original-Windows parity needs an
authenticated original-Windows capture environment; Wine evidence cannot
supply it.

A separately labeled metadata-only replay resolved the two comparator globals
without attaching them retroactively to the screenshots. For exact process PID
`3202977` (start `2026-09-29T12:37:49.560Z`), the identified executable mapped
at its preferred base `0x00400000`; RVAs `0x002be840` and `0x002be850`
therefore resolved to process addresses `0x006be840` and `0x006be850`.
Read-only, exact four-byte `/proc/<pid>/mem` reads observed LC_CTYPE LCID
`0x0409` (1033) and code page 1252 at two recorded post-input timestamps in the
same process. The first followed timed startup and intro-skip input; the second
followed a timed Alliance-intent click, Escape, and F7. Those actions and the
observer's raw stage labels do not independently verify main-menu state,
campaign entry, encyclopedia opening, or cache construction. The retained raw
metadata artifact remains unchanged with SHA-256
`711a5025826e40357ab069e0ea0978b2674d129f5f3d4503f5bb437cb86702c9`.
The superseding provenance qualification has SHA-256
`6151877e8c8b3e8a63e8b0f8e8048aea8b802d2c28a277c57a6888a902203004`.
This proves those two process values at those timestamps, not successful
`setlocale` timing, cache-time provenance, the full one-byte fold map, the live
System iterator, admitted-cache identities, or order; the 200 SYSTEMSD profile
rows remain static candidates rather than assumed same-viewer membership. A
connected source-derived cache root/pointer/identity recipe was requested in
Agent Mail message 21456; no traversal was attempted without it.

Coordinator release 21465 subsequently approved corrected recipe SHA-256
`2347e6adb3176877a5f93d7d9a4034ed93cb736d2ec6aba8d55be3a945c01448`.
The separate direct-parent run `E51-E09-live-cache-1` produced two fully equal
source-connected metadata snapshots for exact PID 3262757 and the identified
executable at runtime base `0x00400000`. Each snapshot independently retained
process identity and before/after sentinels. The canonical snapshot SHA-256 is
`1c94f1a66ae8b590c768800f5c49b2c2e6f5ab645c00781380a354297b2ab345`.

For raw viewer selector 1, the validated mode-2 master cache contains 247 rows,
including 100 system rows: 30 family `0x90` and 70 family `0x92`. The source
iterator independently returned the same 100 identities and multiplicities
with zero observed ancestry exclusions or canonical-key collisions. Its order
is not the title-sorted cache order: 98 of 100 positions differ. The cache
system sequence SHA-256 is
`f586d142009c70b12c8a66b9368d2987cf31fadcaab54f065f58faa1d569051a`;
the iterator sequence SHA-256 is
`67250e7ba7bdfcd50b86575029aa7e5349b70a903d656b27664cae5b785392dd`.
The same snapshots observed LCID `0x0409` and code page 1252. The reader used
10,084 fixed reads / 42,336 bytes and read no localized title/body data.

The metadata artifact SHA-256 is
`aeb4c1140f747571d63d18bb4e3c68fd37b14f3527d3023888e45bb7d76cafd3`;
the supporting context artifact SHA-256 is
`2e1dfe9aaad117104358d2a05f2b9143adb9d6e7660a03c56a1c4d92953dea6d`.
The inspected frame shows the encyclopedia open, but it is supporting context,
not cache proof. This single-viewer result does not establish registry order,
the other viewer, the full fold map, or static-corpus admission; E51's broader
UI matrix remains incomplete.

A second fresh direct-parent run, `E51-E09-live-cache-2-empire`, observed the
other viewer without reusing or overwriting run 1. Inspected checkpoint pairs
show `Standard Game` with the left Imperial control, the Empire strategy shell,
and the Empire encyclopedia. No galaxy/scenario size was visibly labeled, so
none is inferred from the cache size. The source-connected reader separately
reported raw viewer selector 2 for PID 3269886 (start
`2026-09-29T13:42:44.670Z`) with two complete equal snapshots, canonical SHA-256
`b250b7f486906f577a7c4f97121f405af1a632a91c0909f44fc7294166928cfc`.
It observed the same-snapshot LCID `0x0409`, code page 1252, mode 2, 247 cache
rows, 100 System rows, 100 iterator returns, zero exclusions, family counts 30
`0x90` / 70 `0x92`, and 98 cache-versus-iterator position differences. The
reader used 10,068 fixed reads / 42,272 bytes with no read larger than six
bytes, localized text read, write, injection, or dump.

The fresh run-2 full-cache, System-cache and iterator sequences equal run 1
exactly (sequence SHA-256 values `ecc114bc...e7c1a`,
`f586d142...9051a`, and `67250e7b...392dd`). This is a bounded two-run
observation, not a universal viewer-independent admission rule, registry-order
claim, same-generated-campaign claim, or proof that all 200 static SYSTEMSD
rows are admitted. Run-2 reader/context/summary hashes are respectively
`4c3db5cd...6fa1`, `07716545...03c6`, and `1c04afca...864b`. The game was
stopped after acquisition and display `:91` remained available. E51's broad
matrix remains incomplete.

Coordinator review message 21469 supersedes one field label in both immutable
raw reader files: iterator key `picture_selector` contains definition `+0x30`'s
topic/text selector used for the low-12-plus-`0x1000` canonical key. It is not
the separate System image selector 1, 24, 25 or 26. Qualification artifact
`live-cache-selector-field-provenance-r6.json` has SHA-256
`841e800d8d8112bd1c4ca5da6b97b6614d34855c13046d66180f7fae7d71d7b7`;
future output uses `topic_selector`. This changes no retained values,
identities, hashes, snapshot conclusions or matrix acceptance.

E51 r7 uses a separate bounded `O_RDONLY` observer for ordinary reachable UI
state. It validates the exact process/executable and source-connected child
vtable, reads only mode/category/control pointers/current-row metadata/art
presence/viewer/locale, and requires two complete equal snapshots. Four
synthetic tests cover valid index/topic states plus tree-cycle, changed-state,
wrong-vtable, wrong-mode/category and malformed-row rejection. Observer and
test SHA-256 values are respectively `568ac568...8fe31` and
`a33abd30...bfd9`.

Run 3 (PID 3352465) retained three same-run endpoint pairs before a later
PageDown step was rejected. Run 5 (PID 3359853) retained ordinary index,
pointer body-scroll, list PageDown/Return, filtered Right endpoint and Escape
close. Their `run.json` SHA-256 values are `5d0f5276...29ae` and
`e886522e...02a8b`. Left at the first full and filtered rows retained key 5696
/ identity `0x1c000002`; Right at the last full row retained key 6802 /
identity `0x38000002`; Right at the last filtered row retained key 5699 /
identity `0x1c000002`. The repeated packed identity across different filtered
canonical keys is not treated as a unique topic ID. Pointer scrolling visibly
moved the body while key 5696 remained current; two PageDown attempts instead
changed the selected list topic and are rejected as body-focus proof. Escape
left no command-`0x19` child in either equal snapshot.

Run 6 (PID 3399522, `run.json` SHA-256 `e12c0477...93dc`) source-joined four
Alliance current rows to accepted E39 metadata: selector 1 identity
`0x9200007a` / body 7714 / EDATA.166; 24 identity `0x9000010f` / body 8049 /
EDATA.191; 25 identity `0x90000117` / body 8057 / EDATA.189; and 26 identity
`0x9200011a` / body 8066 / EDATA.190. All were mode 2/category `0x70`, had
nonnull art objects, equal snapshots, and inspected distinct images. This is a
live-current-row plus accepted-source join, not a read of the filename chosen
inside the original loader and not Empire-side selector proof.

All six r7 attempts/runs remain immutable. Runs 1, 2, and 4 preserve rejected
setup, recipe-digest, and Page-key attempts; no check was weakened. The summary
artifact SHA-256 is `bb39f107...1afaa`, and its checker SHA-256 is
`752da75f...a595`. The checker now recomputes every cited canonical snapshot
hash rather than merely comparing the two stored hash strings. Immediate
after-action full-desktop frames for several
endpoints caught a Wine/Xvfb repaint gap; paired native 640 × 480 frames and
equal metadata are retained, but those transient desktops are supporting-only.
Consequently all 21 row acceptances remain open.

Revision 8 supersedes one raw snapshot field label without rewriting any r7
artifact. `snapshots[].current_topic.dat_id` contains
`(row + 0x68) & 0x00ffffff`: the low 24 bits of the runtime packed handle, now
named `packed_handle_low24` in future observer output. It is not a general
original DAT identity. Canonical keys 5696 and 5699 share packed handle
`0x1c000002` and low-24 value 2. Original DatId attribution therefore requires
a unique canonical-key/profile join. The System DatIds 122/271/279/282 remain
valid through their explicit accepted E39 joins, not the raw field. The
qualification artifact SHA-256 is `ab6b32ab...80b5c`; it records unchanged r7
summary/run hashes and no recapture. Future observer/test hashes are
`086c04b6...10c4d` and `33cc0a9f...5cafa`.

The exceptional rows are now source-guided investigated gaps rather than
unattempted assertions. The five contextual caller paths need legitimate
owning-surface routes and typed wrapper/current-key/internal-focus capture;
`FUN_0045fd90` needs owned setups for both `0xa0..0xaf` association branches
and a non-special miss; unavailable mode needs a real stale context; the two
fresh viewer runs' 100/100 System admission with zero exclusions does not
replace a type-`0xf2` ancestry fixture; reviewed profile accounting makes the
original no-art capture corpus-not-applicable for the supported profile while
leaving the generic null-art branch untested; and all seven visible
categories do not replace a connected category-visibility writer. No matching
owned save/scenario was present. `EDATA.192` remains deferred, not a fixture,
and E26 mod-null remains a separate synthetic runtime case.

Space battle is an explicit full mode, not a single panel. `TAC-01` through
`TAC-07` currently define 106 baseline cells covering battle entry, both
faction HUDs, capital ships, fighters, assignment, selection, targeting,
damage, camera and navigation, maneuvers, tactics, missions, recovery,
withdrawal, simulation and observation, Death Star controls, trench-run
routing, results, and strategic return. `EVT-02` owns Battle Alert entry;
`TAC-08` remains the separate strategic ground-assault report flow.

P54 now supplies the raw-resource boundary for `RE-TAC-02`: 87 type-301
binary X meshes and 397 type-303 texture or palette resources reproduce from
the owned `TACTICAL.DLL` with exact identities and hashes. The
[P54 evidence](evidence/2026-09-12-tactical-3d-staging.md) records the inventory
and one embedded mesh-to-texture edge. P55 decodes all of them into a
deterministic runtime store and verifies the mesh corpus against Assimp 6.0.5;
see its [evidence](evidence/2026-09-12-tactical-3d-runtime-pack.md). P56
packages and visibly renders exact pair `2560/1033` plus
`SDESTI52.BMP/1033`. P57A adds resources `2561/1033` and `2562/1033`, the
second named texture, and the original high- and reduced-detail LOD predicate
traced through `FUN_005d26c0`, `FUN_005d3770`, and `FUN_005d3650`; see its
[evidence](evidence/2026-09-13-tactical-3d-lod-family.md). P57B1 adds
`FUN_005c1160`, whose slot 2, 1, 0, restore sequence corroborates cached object
switching, and proves the live sequence with one browser family load; see its
[evidence](evidence/2026-09-13-tactical-live-lod-journey.md). P57B2A traces
`FUN_005c1d30`, `FUN_005d9490`, `FUN_005d9620`,
`FUN_005d9640`, and the command switch at `0x005d97c0` to recover exact faction
pose, field zoom, clip, orbit, pitch, and target-command state. Its
[evidence](evidence/2026-09-13-tactical-camera-contract.md) proves the bounded
camera fixture and four bitmap D-pad directions. P57B2B1 recovers switch case
9 through `FUN_00595be0` and `FUN_005c1080`, activates resources 1058/1059,
and proves its selected-object record and fallback centering; see its
[evidence](evidence/2026-09-13-tactical-target-control.md). P57B2B2 recovers
`FUN_005ab650`'s active-object count, docked-fighter exclusion, battle extent,
and four tactical lanes; see its
[evidence](evidence/2026-09-13-tactical-battle-layout.md). P57B2B3 traces
`FUN_005a9030` and the placement callsites, binds stable DAT and
unfiltered fleet-roster identity to production participants, reproduces the
source X-slot sequence and four faction lanes, and sends the selected source
world point to the camera. Its
[evidence](evidence/2026-09-13-tactical-participant-placement.md) closes that
bounded recovery package. P57B2C1 proves that the original path preserves
authored mesh coordinates without center/scale normalization, packages every
system palette, and selects `5530 + SYSTEMSD.picture_id`; see its
[evidence](evidence/2026-09-13-tactical-transform-palette.md). The P57B2C2A
light checkpoint recovers `FUN_005d4d10`'s directional RGB `0.8`
frame at `(5,5,-1)`, origin target with Z constraint, and ambient RGB `0.5`;
see its [evidence](evidence/2026-09-14-tactical-light-rig.md). P57B2C2B then
recovers `FUN_005c1c10` and `FUN_005d6e10` device and material state; see its
[evidence](evidence/2026-09-14-tactical-render-state.md). The
[P58A evidence](evidence/2026-09-14-tactical-resource-join.md) then joins the
source-named tactical registry to every ship and fighter DAT identity, removes
approximate sprite arithmetic, and browser-proves complete 87-mesh and
397-texture transport. [P58B evidence](evidence/2026-09-14-tactical-production-participants.md)
proves live production capital-family drawing at recovered source positions.
[P58C evidence](evidence/2026-09-14-tactical-fighters-selection.md) proves the
type-303 fighter resource triplets, exact detail thresholds, production fighter
pixels, and projection-aligned capital interactions. The
[P58D evidence](evidence/2026-09-14-tactical-fighter-detail-journey.md) proves
the nine independent indicator/far/close transitions through the original zoom
controls without family reload. The
[P58E evidence](evidence/2026-09-14-tactical-group-presentation.md) restores all
eight task-force and four RGBY controls, source-shaped input, Ctrl assignment,
keyboard routes, and selected fighter portraits. Automatic production grouping
and original visual acceptance remain open. P58F1 through P58F3 restore the
system-selected planet, target effects, projectiles, and tractor/gravity fields.
The [P58F4 evidence](evidence/2026-09-15-tactical-selected-damage.md) then binds
source tactical ordinals to capital portraits `2001` through `2029`, composites
their lime mats over panel `1302`, and renders live faction-correct shield and
hull meters. [P58F5 evidence](evidence/2026-09-15-tactical-subsystem-field-commands.md)
restores all five subsystem-condition bitmap families through exact source
quantization and placement, plus exact field identities and capacities.
[P58F6 evidence](evidence/2026-09-15-tactical-live-subsystem-damage.md)
connects those conditions to the live damage path. The
[P58F7 evidence](evidence/2026-09-15-tactical-subsystem-repair-mobility.md)
restores the repair cadence and selection plus the engine-condition and active
tractor-drag mobility calculation.
[P58F8 evidence](evidence/2026-09-16-tactical-maneuver-movement.md)
restores the maneuver-state bonus, effective-power velocity, signed faction
movement, and millisecond position integration.
[P58F9 evidence](evidence/2026-09-17-tactical-command-assignment.md) restores
both original command panels, exact shared order and tactic codes,
source-shaped controls, disabled states, and commit/cancel delivery.
[P58F10 evidence](evidence/2026-09-17-tactical-order-execution.md) restores the
four maneuver waypoint constructors, Hold stop behavior, and the first Recover
carrier state. [P58F11 evidence](evidence/2026-09-18-tactical-command-progression.md)
restores source-rate signed turning, deterministic waypoint completion, and the
Docking and Recovered fighter states while preserving strategic squadron
counts. [P58F12 evidence](evidence/2026-09-18-tactical-attack-targeting.md)
restores typed Attack Fighters and Attack Capital Ships acquisition for both
capital and fighter owners. [P58F13 evidence](evidence/2026-09-19-tactical-attack-target-lifecycle.md)
restores live invalidation, stable same-class replacement, and exhausted-list
clearing. The [P58-B06 checkpoint](evidence/2026-09-22-tactical-completion-bundle.md)
adds capital and fighter combat, collision, automatic groups, retained
formations, the separate Death Star, original result/options panels, and both
trench-run routes. [P58-B13 evidence](evidence/2026-09-23-tactical-withdraw-confirmation.md)
restores the executable-derived withdrawal confirmation and exact panel,
text, control, cancel, and confirm contract. The
[P58-B14 evidence](evidence/2026-09-23-tactical-detail-escort.md) adds complete
destroyed presentation, compact GOKRES assignments in panel 1302, and direct
right-click Escort from `FUN_005ca6d0` with order code 1, retained target,
marker, follow, opportunity fire, and target cleanup. P58-B21 shares the
post-battle campaign route. P58-B22 adds one recovered tactical RNG stream,
persisted power allocation, completion/destruction callbacks, and the timed
trench producer. Whole-process RNG continuity, strategic commander binding,
native beam/playback comparison, rare audio callers, and original visual
acceptance remain open.

[P58-B15 evidence](evidence/2026-09-24-tactical-battle-alert-audio.md) traces
`FUN_0044f860` into the faction Battle Alert resources and recovers MDATA 307
plus event `0x14` to TACTICAL WAVE 13054 through `FUN_005bae60`,
`FUN_005ba980`, `FUN_005bad50`, and `FUN_005ba520`. The deterministic A1
crosswalk now maps all 106 cells; A0 coverage and strict acceptance remain
0/106.

[P58-B16 evidence](evidence/2026-09-24-tactical-weapon-audio.md) corrects the
provisional event label: `FUN_005a7500` registers event `0x14` as
`SHIP_TAKE_TORPEDO_HIT`, not ship destruction. Together with the audio-manager
tables and `FUN_005b3f10`, it maps events `0x0d–0x14` to all 22 WAVE
`13033–13054` variants and dispatches them from production capital and fighter
combat. Exact shared-RNG sequencing and remaining non-weapon/voice events stay
open.

[P58-B17 evidence](evidence/2026-09-24-tactical-command-voice.md) follows the
faction event bases in `FUN_005bae60` and the production command callers to 90
exact VOICEFXA/VOICEFXE recordings. Battle ready and task-force/RGBY maneuver,
attack, formation, and mission acknowledgements now dispatch through native
and browser audio backends. The focused muted browser gate loads and routes
all 22 weapon and 90 command-voice resources for both factions and viewports.
Result, withdrawal, and Death Star voice families, mixing, interruption,
audible native comparison, and strict A0 acceptance remain open.

[P58-B18 evidence](evidence/2026-09-25-tactical-complete-voice-bank.md) extends
that source table through event `0x13c` and proves complete transport of 285
recordings: VOICEFXA `14001–14122`, VOICEFXE `15001–15132`, and VOICEFXA
`15133–15163`. Production now dispatches selected withdrawal, battle-outcome,
Death Star, and RGBY trench-run transitions. The focused muted browser gate
loads and routes all 307 tactical weapon and voice resources for both factions
and viewports. Remaining completion, destruction, recovery, warning, and
ordered trench chatter callers, exact shared-RNG sequencing, mixing,
interruption, audible native comparison, and strict A0 acceptance remain open.

[P58-B19 evidence](evidence/2026-09-25-tactical-mixed-task-force-target.md)
follows `FUN_005a24d0`'s task-force ordinal check before hostile focus-target
assignment. Mixed capital selections now preserve manual, active, and Escort
targets and queue Alliance event `0x84` / WAVE `14101` or Imperial event
`0x102` / WAVE `15105`. Four muted faction/viewport cases pass; audible native
comparison and strict A0 acceptance remain open.

[P58-B22 evidence](evidence/2026-09-28-tactical-source-completion.md) follows
`FUN_0061a310`, `FUN_005a8a70`, `FUN_00501510`, `FUN_005015a0`,
`FUN_005a5cf0`, `FUN_005cfec0`, `FUN_005d04e0`, and `FUN_005d03f0`. The
bounded practical launcher now uses one ordered tactical RNG stream, persisted
shield/weapon allocation, exact completion/destruction callbacks, and the
timed trench-run producer. Strict A0 acceptance remains 0/106.

## Immediate implementation order

1. Trace SPT to BIN to frame and WAV selection for both factions on top of the
   verified type-302 advisor idle-frame decoder and transport.
2. Extend the decoder and transport to ALBRIEF and EMBRIEF.
3. Extend staging and the runtime pack with EData, dialogs,
   voices, and media without weakening deterministic manifests.
4. Complete the first recovered sector, system, and reference-rail checkpoint
   with exact item compositions, commands, thumbnails, and uncommon states,
   then continue through the remaining managed original object windows.
5. Continue the full GID mapping without replacement art: recover the code-built
   menu, expanded legend, remaining filter predicates and overlays, and exact
   map input.
6. Preserve the P58-B22 tactical implementation, bind the strategic commander
   slot, verify whole-process RNG continuity and remaining rare audio/native
   Death Star behavior, then acquire A0 evidence for the complete mapped
   matrix using the [ranked Windows/Ghidra recovery map](../../reference/space-battle-launcher/reverse-engineering-map.md).
7. Use original-runtime capture only for the remaining dynamic proof boundary.

No static discovery marks a surface complete. It closes only the corresponding
evidence fields; the original, native, and browser acceptance cells remain in
the main audit.
