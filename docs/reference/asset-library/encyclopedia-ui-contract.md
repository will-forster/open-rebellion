---
title: "Original Encyclopedia UI Contract"
description: "Source-backed construction, controls, callers, interaction, and remaining A0 capture matrix for the original encyclopedia"
category: reference
created: 2026-09-29
updated: 2026-10-01
tags: [encyclopedia, ui, reverse-engineering, parity, provenance]
---

# Original Encyclopedia UI Contract

This contract records the original encyclopedia surface independently of the
port's eventual catalog and renderer. It is bounded to the inspected English
`REBEXE.EXE` whose SHA-256 is
`b3fe3997cab9a6e96403d638875dcba25484e4d8601751afec748471ac0ed6ab`.
The semantic identity, availability, ordering, context, navigation, faction,
and system-selector rules remain defined by the
[source contract](encyclopedia-source-contract.md#semantic-research-checkpoint).
This document adds the source-proven window and input contract; it does not
change those rules or claim runtime visual acceptance.

## Evidence boundary

The evidence grades used here are deliberately separate:

- **Static source proof:** connected calls in the checked-in decompilation
  notes plus focused disassembly of missing bodies at `0x0041d6b0`,
  `0x00438800`, `0x004443a0`, `0x0045da70`, and `0x00467f10` from the
  identified executable. Revision 2 also checks the missing predecessor helper
  at `0x005f5c60` and background selector at `0x006075e0` directly.
- **Resource proof:** `strategy-dll` entries in
  `resource-inventory.json`, joined to the exact resource IDs passed by
  `FUN_0045ddc0`.
- **A3 visual leads:** Steam-guide captures 018, 019, and 027 and the French
  Abandonware capture. These corroborate composition only; they do not supply
  exact geometry, control state, English text, or strict-parity proof.
- **A0 proof:** E51 acquired a bounded compatibility reference under Wine for
  the identified executable. Fifteen rows have partial acquisition, six lack a
  direct acquisition, and all 21 row acceptances remain open. The exact
  environment, actions, file hashes, acquired evidence, and unmet subcriteria
  are recorded in the
  [original capture evidence](encyclopedia-original-captures.md). The partial
  set does not establish Windows-rendering parity or complete A0 acceptance.

All coordinates below are integer pixels relative to the encyclopedia client.
`FUN_00429f30` passes `470 × 330` (`0x1d6 × 0x14a`) to
`FUN_0045d400`/`FUN_00606380`. `FUN_0045ddc0` sizes and positions children in
that client coordinate space. The shell supplies placement through its
`+0xcc..+0xd8` rectangle to `FUN_00606980`; no fixed desktop origin is claimed.
The primary faction backgrounds are 470 × 331 resources, so their stored
bitmap height is not a second client-height claim.

## Construction and callers

### Context-free routes

`FUN_00422ce0` is the proven shell caller for both context-free entry paths:

| Trigger | Connected flow | Exact behavior |
|---|---|---|
| `WM_KEYDOWN`, virtual key `0x76` (F7) | `FUN_00422ce0` → `FUN_00429f30` | Builds empty class/entity wrappers and requests the encyclopedia directly. |
| `WM_COMMAND` `0x131` | `FUN_00422ce0` → `FUN_00429f30` | Builds the same empty wrappers, but does not open while `FUN_004fcee0() < 2`. |

`FUN_00429f30` first posts message `0x409` to the shell, then searches the
shell child collection at `+0x6c` for type/command `0x19`. It constructs only
when none exists, through `FUN_0045d400(..., 470, 330, shell, 0x19, contexts)`;
the new child is registered and shown through vtable slot `+0x2c` with argument
`5`. Repeating either route therefore does not construct a second encyclopedia
child in the same shell.

### Contextual routes

The exact trampoline at `0x0041d6b0` calls `FUN_00422ca0` to obtain the active
shell and, when non-null, forwards both typed context wrappers to
`FUN_00429f30` at call address `0x0041d6c5`. An exhaustive direct `call rel32`
cross-reference scan of the inspected executable proves five callers:

1. The Build Selection handler at `0x00438800`, command `0x67`, reads the
   selected row from the control at `this + 0x114`, takes its typed class
   identity at row `+0x54`, constructs an empty entity companion wrapper, and
   calls the trampoline at `0x004388ff`. The surface name and class/entity
   distinction are accepted only through the corrected r49–r52 chain below,
   not inferred from proximity or visible text.
2. The unrenamed handler at `0x004443a0`, command `0x66`, rejects an empty
   identity and an identity whose high byte is `0xf1`, closes its current
   surface, and calls the trampoline at `0x0044445d` with the retained context
   at `this + 0x128` and an empty companion wrapper. Command `0x65` only closes.
3. The unrenamed handler at `0x00467f10`, commands `0x67` and `0x97`, chooses a
   current child according to mode at `this + 0x118`; only a non-null child
   whose virtual `+0x10` result is `4` reaches `0x0046884e`. That branch copies
   the child context at `+0x60` and calls the trampoline at `0x0046886a` with
   the already-prepared companion wrapper.
4. `FUN_0046c3c0`, command `0x67`, reads the selected row from the mission
   dialog list at `this + 0x128`, copies its identity into a context wrapper,
   and calls `FUN_0041d6b0`.
5. `FUN_00486fb0`, event `0x100`, reads the first row from the caller-supplied
   list/collection, extracts its typed identity through `FUN_00403040`, and
   calls `FUN_0041d6b0`. Its sibling event `0x103` calls `FUN_0041d7f0`
   instead and is not an encyclopedia route.

The same scan finds only the trampoline plus the two shell sites at
`0x00423667` and `0x00423b5a` calling `FUN_00429f30` directly. This bounds the
direct-call graph; it does not claim that an unresolved indirect function
pointer has a friendly surface name. The visible owning surfaces for the two
still-unrenamed handlers and every possible dispatch into `FUN_00486fb0` are
not renamed from proximity alone. A0 capture must retain the originating
control, command/event, and typed identity for each route.

Accepted r45 evidence resolves the mission caller's typed contract without
changing that five-caller bound. `FUN_0046c3c0` command `0x67` copies the
selected mission-definition row into a class wrapper, pairs it with an empty
entity companion, and calls `FUN_0041d6b0`. `FUN_00429f30` passes the wrappers
to `FUN_0045d400` only when constructing the encyclopedia child; an existing
child is reused without reconstructing its context. The selected mission row
is therefore not the live Personnel entity used to assemble a mission team.

Accepted r46 evidence proves that distinct upstream setup at static scope.
`TEXTCOMM.DLL` accelerator table 11 maps Alt+M to command `0xbc0`; the root
`TranslateAcceleratorA` loop forwards it to the active child as `0x483`; a
valid selected Personnel entity may create order `0x240`; and only a nonempty
`FUN_004f5380` legal-mission result admits the mission dialog. This is a
conditional setup path, not a guarantee for every selected character/system.

Coordinator-accepted r47 Wine-visible evidence exercises the downstream
mission caller for two class rows. In an Alliance / Sumitra / Yavin run, the
real Recruitment and Espionage mission rows each opened the matching article
through the visible mission-dialog Encyclopedia control; ordinary close
retained the mission and Personnel windows. The isolated Alt+M attempt did not
visibly change the Personnel surface, and a later distinct double-click
preceded dialog appearance, so r47 does not causally prove the hotkey path.
Neither the visible control nor pixels prove command/notification/resource,
canonical current-key, DatId, or target-thread-focus identity.

Accepted r49–r52 evidence identifies the first contextual caller as the
ordinary Build Selection dialog and retains the complete command-`0x67`
dispatch. The selected row supplies a class wrapper and the companion is an
empty entity wrapper; `FUN_00429f30` consumes them only on its create branch.
The r50/r51 reviews reject type-only instance inference. The repaired proof is
limited to the ordinary fresh-created collection and the key-check/equal edge:
`0049e130–0049e196` (SHA-256
`2e279e7bf11780e1c28ecfd66dbd02329794b35bfb813969456ae43ab88dcc70`)
returns exact nodes from the same collection and `00568ee0–00568ee5`
(SHA-256
`ad754bd90725bde9c206591c56972f8678048a6af3b94433e5587eb7acd98c1e`)
reaches the exact-node leftmost fallback. The bypass at `0x00439ae4`,
duplicate-key or malformed serialized state, and an already-open encyclopedia
are not covered by that identity conclusion.

Coordinator-accepted r53 Wine-visible evidence exercises the resulting class
surface in one Alliance / Chandrila run. Under `visible-r53-1/`, Alliance Escort
Carrier is selected in Build Selection, its visible Encyclopedia control opens
the matching article, and ordinary close retains Build Selection and Chandrila
Shipyards. The first post-shortcut frame showed no candidate dialog; a later
visible yard click opened Build Selection, so the evidence does not attribute
dialog creation to Alt+B. These pixels do not observe the internal command,
notification, canonical current key/DatId, selected title/body/art resources,
or target-thread focus.

`FUN_0045d400` resolves the supplied class/entity context to the canonical
cached topic described by the source contract. A resolved current row selects
topic mode; empty or unresolved context selects index mode. No raw identity is
fabricated into a topic.

## Client composition

`FUN_0045ddc0` composes the client from `STRATEGY.DLL` resources. The two
faction paths use the same content rectangles but different shell and rail
chrome where shown below.

| Element | Rectangle `(x,y,w,h)` | Command / role | Source/resource evidence |
|---|---:|---|---|
| Client shell | `(0,0,470,330)` | Faction chrome | Alliance `0x285f`; Empire `0x2860`; both 470 × 331. |
| Topic inner overlay | `(12,14,400,306)` | Background selector `1`; same mode resource for both factions | `0x2861`, 400 × 306. |
| Index inner overlay | `(12,13,400,306)` | Background selector `2`; same mode resource for both factions | `0x2862`, 400 × 306. |
| Index baked static text | Origin `(36,48)`; extent measured from a zero-size rectangle | Drawn into background selector `2`, not a child control | `FUN_00601700` constructs text resource `0x1843`; `FUN_00601b80` measures it through `DrawTextA` flag `0x400`; `FUN_00601ce0` renders it into the index composite before registration. Its semantic label is not inferred. |
| Right rail | `(412,0,58,330)` | Faction rail chrome | Alliance `0x2959`; Empire `0x295d`. |
| Return/close | `(423,25,32,31)` Alliance; `(426,21,44,41)` Empire | `0xfb` | Alliance normal/pressed `0x2882/0x2883`; Empire `0x2888/0x2889`; layout identity `0x1954`. |
| Header/title static text | Positioned `(36,14)` | static ID overwritten to `0x15e` | Constructed from `0x1842`; reset to that resource in index mode and populated from current row `+0x14` in topic mode. |
| Selected-category static text | Positioned `(40,119)` | static ID overwritten to `0x11b` | Constructed from `0x1850`; `FUN_0045f100` replaces its text from the selected category control. |
| Mode tab container | `(423,93,32,116)` Alliance; `(426,89,44,136)` Empire | `0x66` | Holds topic/index buttons. |
| Category tab container | `(36,78,361,41)` | `0x6e` | Holds the seven category buttons. |
| Index selection field | `(143,45,245,18)` | field command set to `0x64` | `CoolStringField`, white text; selected list row `+0x14`; hidden in topic mode. |
| Index list | `(36,137,350,160)` | `0x65` | `CoolDragList`; row-height argument `20`, font/resource `0x299d`. |
| Topic body | `(17,231,395,80)` | read-only text field | `TextScrollField`; font/resource `0x299d`. |
| Forward | `(380,14,21,17)` | `0x84` | normal/pressed/disabled `0x288e/0x288f/0x2890`; layout `0x1857`. |
| Backward | `(28,14,21,17)` | `0x83` | normal/pressed/disabled `0x2891/0x2892/0x2893`; layout `0x1858`. |
| Topic art | blit origin `(12,31)` | current image | `FUN_0045f090` draws the source image without a resize request; owned first-profile images are 400 × 200, but that measurement is not a universal UI assumption. |

`FUN_0045ddc0` first chooses one faction shell and rail, then makes two
composites from that same chosen pair. It blits `0x2861` at `(12,14)` into the
first composite and registers it as background `1`. It blits `0x2862` at
`(12,13)` into the second composite, creates the `0x1843` static text object at
`(36,48)` with zero initial extent, measures it through `FUN_00601b80`, renders
that text through `DrawTextA` into the bitmap, and registers the finished
composite as background `2`. `FUN_006075e0` resolves a registered
background selector and makes it current. `FUN_0045f480(1)` selects background
`2` for index mode; `FUN_0045f480(2)` selects background `1` for topic mode.
Index background selector `2` and topic background selector `1` apply to both
factions.

The overlays are therefore mode-specific, while the underlying shell and rail
remain faction-specific. Resource `0x1843` is proven to be baked static text;
this contract does not guess its semantic meaning from its position or pixels.

The header object and navigation controls remain constructed in both modes;
the index selection field is hidden in topic mode. `FUN_0045f480` switches
child visibility and focus rather than constructing a different window.

### Mode buttons

| Mode | Command | Relative rectangle | Alliance normal/active | Empire normal/active | Behavior |
|---|---:|---:|---|---|---|
| Topic | `0x67` | `(0,0,32,31)` / `(0,0,44,41)` | `0x2886/0x2887` | `0x288c/0x288d` | Selects topic mode only when a current row exists; otherwise the handler reselects index. |
| Index | `0x68` | `(0,54,32,31)` / `(0,54,44,41)` | `0x2884/0x2885` | `0x288a/0x288b` | Selects index mode and focuses the list. |

The selected-state bitmap is copied from each button's pressed resource by
`FUN_0060d700`. `FUN_0045ddc0` also assigns that same active resource as the
mode button's disabled-state image.

### Category controls

The category labels remain unjoined to source strings. Stable UI identities are
the command, rectangle, source-identity range, and resources; category names
must not be guessed from icon appearance.

| Slot | Command | Rectangle | Source range / role | Alliance normal/active | Empire normal/active |
|---:|---:|---:|---|---|---|
| 0 | `0x6f` | `(36,78,49,41)` | full master index | `0x2864/0x2863` | same |
| 1 | `0x70` | `(88,78,49,41)` | `[0x90,0x98)` | `0x286e/0x286d` | same |
| 2 | `0x71` | `(140,78,49,41)` | `[0x14,0x20)` | `0x286c/0x286b` | `0x2878/0x2877` |
| 3 | `0x72` | `(192,78,49,41)` | `[0x20,0x30)` over admitted master rows | `0x2868/0x2867` | `0x2874/0x2873` |
| 4 | `0x73` | `(244,78,49,41)` | `[0x40,0x80)` | `0x2d60/0x2d5f` | `0x2d62/0x2d61` |
| 5 | `0x74` | `(296,78,49,41)` | `[0x10,0x14)` | `0x2870/0x286f` | `0x287a/0x2879` |
| 6 | `0x75` | `(348,78,49,41)` | `[0x30,0x40)` | `0x286a/0x2869` | `0x2876/0x2875` |

`FUN_0045f100` supplies the exact filtering and early-return semantics. In
topic mode a changed, non-forced category request is ignored. Selecting a
category in index mode binds or rebuilds the appropriate sorted collection,
updates the index label from the selected category control, and leaves focus in
the list.

## Button state contract

The shared `CoolStrobeButton` path is connected, not inferred from the bitmap
pairs:

| State | Source behavior | Bitmap slot |
|---|---|---|
| Normal | No pressed, disabled, or tab-selected flag. | constructor first resource at `+0x94`. |
| Hover | `FUN_006028c0` has no separate mouse-move/hover resource transition. | normal bitmap remains selected; an A0 capture must still check cursor/strobe timing outside this bitmap selector. |
| Pressed | Button hit-test succeeds, mouse down sets flag `0x1`, captures the pointer, and release inside posts `WM_COMMAND`; leaving the hit area cancels visual activation. | constructor second resource at `+0x98`. |
| Disabled | vtable disable path at `0x006028a0` sets flag `0x2`; `FUN_00602d30` selects state `2`. | `+0x9c`, falling back to normal if absent. Forward/back have dedicated disabled images. |
| Selected tab | `FUN_0060d7e0` clears flag `0x4` on the old tab and sets it on the new tab. | `+0xa0`, initialized by `FUN_0060d700` from the pressed resource. |

Mouse press capture and cancellation are implemented at `0x006028c0`; state
flags are set/cleared by `0x006030c0`/`0x006030f0`, and painting is selected at
`0x00602d30`. This proves the bitmap-state mapping. It does not prove a separate
animated hover effect, and none is admitted by this contract.

## Mode, focus, keyboard, and scrolling

### Index mode

- The list, category tabs, and index-side static field are shown; the body and
  directional buttons are hidden. `FUN_0045f480(1)` selects the index mode tab.
- Category selection and switching to index mode both focus the list window.
- A new single-click selection sends notification `0x29b`; a repeat click may
  send `0x29d`. Neither opens the topic. A double-click sends `0x309`. These
  are independently reviewed static emitter paths, not claims about which
  notification a retained Wine frame delivered.
- With list focus, Return (`VK_RETURN`, `0x0d`) marks the selected row and sends
  the same `0x309` notification. When both the selected list item and current
  row resolve non-null, `FUN_0045da70` maps `0x309` to mode command `0x67`.
  The static receiver chain is conditional and is not a retained runtime
  notification trace.
- Left (`0x25`) and Right (`0x27`) are category navigation in index mode, not
  topic navigation. `FUN_0045fe60` reads the selected category child from the
  category container at `this + 0x124`, recovers its ordered-tree node, then
  uses `FUN_005f5c60` for the predecessor on Left or the node's threaded
  successor at `+0x10` on Right. It skips non-null candidates whose child
  windows are not visible and selects the first visible candidate through
  `FUN_0060d7e0(..., command, 1)`, which updates the selected state and sends
  `WM_COMMAND` to the encyclopedia handler.
- The index edge behavior is asymmetric and must not be replaced with the
  topic endpoint rule. If the *first* predecessor/successor lookup is null,
  both keys fall back through `FUN_005f5060` to the tree's leftmost child:
  Left on the first child reselects that first child, while Right on the last
  child wraps to the first. That immediate fallback does not run the visibility
  test. If a non-null candidate is hidden, traversal continues in the requested
  direction; reaching null only after skipping hidden candidates retains the
  current category and does not invoke the leftmost fallback.
- `FUN_0045fe60` and `FUN_0060d7e0` do not transfer focus themselves. Their
  synchronous `WM_COMMAND` reaches the category branch at `0x0045dad0`, which
  rebuilds/binds the list and explicitly calls `SetFocus` on the list window at
  `this + 0x12c`. Category keyboard selection therefore finishes with list
  focus.
- Up/Down move by one linked row; Page Up/Page Down move by the visible-row
  count derived from list height / row height; Home/End select first/last.
  Reaching either end retains the endpoint. The list does not wrap.

### Topic mode

- The list, category tabs, and index-side field are hidden. The header, body,
  image region, Backward, and Forward are shown, and `FUN_0045f480(2)` focuses
  the body window.
- Left/Right in `FUN_0045fe60` invoke commands `0x83`/`0x84`. The handler at
  `0x0045da70` follows the skip-disabled row links, retains the current row on a
  null endpoint, refreshes body/title/art for a non-null neighbor, and returns
  focus to the body.
- `FUN_0045fd20` enables each direction only when its corresponding neighbor is
  non-null. The endpoint is therefore both non-wrapping and visibly disabled.
- The body is read-only. Up/Down and Page Up/Page Down scroll the body;
  Left/Right escape the body to the encyclopedia handler and become topic
  navigation.
- Return in the read-only body is forwarded to the encyclopedia handler, whose
  default path forwards it to the shell parent. No encyclopedia-local Return
  action is defined in topic mode. Tab (`0x09`) is consumed by
  `FUN_0045fe60`; Escape (`0x1b`) invokes the encyclopedia close method.
- Command `0xfb` also invokes the close method (vtable `+0x30`).

`FUN_0041fd00` calculates body extent with `DrawTextA` flags `0x2410`, including
word breaking and calculated height. When text exceeds the 395 × 80 viewport it
enables the attached vertical scrollbar and recalculates with the reduced text
width. `FUN_0041fe70` respects line feed and tab boundaries, wraps at spaces,
and splits an overlong run to the available width. Exact glyph breaks depend on
the original font resource `0x299d` and remain an A0 pixel-comparison gate.

## Semantic rules as UI actions

The UI consumes the source contract without adding a second interpretation:

| Semantic result | UI state/action |
|---|---|
| Context-free open | Index mode, full-cache selector `0x6f`, list focus, no fabricated current topic. |
| Resolved class/entity context | Topic mode on the canonical cached row; typed source identity is routing input, not the topic key. |
| Unresolved/stale context | Index mode. No raw resource or static asset creates an otherwise unavailable row. |
| Category selection | Commands `0x6f..0x75` bind/filter the retained master cache in the proven fixed order. |
| Viewer-faction selector | Applicable topic identity remains fixed while the proven `+0x1000` or `+0x2000` image key is selected. |
| System picture selector | The proven 1–26 selector chooses key `0x2b5c..0x2b75`; it is not faction arithmetic. |
| System ancestry exclusion | An excluded type-`0xf2` ancestry view never enters the list and cannot be opened contextually. No friendlier gameplay label is inferred. |
| First/middle/last topic | Backward/Forward state follows the skip-disabled links; endpoints disable and never wrap. |
| Missing art object | `FUN_0045f090` omits the EData blit when current art at `+0x14c` is null; header/body mode remains separately constructed. The generic branch remains untested. It is corpus-not-applicable to the checksum-pinned supported profile because all 347 bound rows have effective art; no fictional original topic or E26 mod-null fixture is required. |

### No live-stat binding recovered

The original constructor creates only the header/index fields, list, text body,
image region, tabs, and navigation/close controls above. `FUN_0045fa60` reads
row `+0x0c` for the qualified body/art key, loads the type-10
`ENCYTEXT.DLL` body resource, copies the decoded narrow string into body object
`+0xa0`, and lays it out. Only afterward does it read row `+0x14` for the
display title. The title does not select the body. Its only connected
world-derived display lookup is the
system picture selector used for art. There is no constructed stat-row control
and no connected lookup of current maintenance, capacity, shield, weapon,
character, or facility values. `FUN_0045fa60` writes row `+0x14` into the
header object at `this + 0x120`; the separate `CoolStringField` is the
index-selection field and is hidden in topic mode.

Accordingly, values that look like ship statistics in A3 guide capture 027 are
treated as authored body text for this contract, not live fields. If future
source tracing or an A0 controlled-state capture shows a dynamic substitution,
the source/schema review must reopen before a consumer adds a live binding.

## Finite A0 capture matrix

The matrix is finite but intentionally does not claim that source proof is a
visual pass. Each capture must be lossless at native 640 × 480, retain faction,
entry route, typed identity/current key, selected command, and original
resource identities, and compare exact client bounds and state pixels.

| ID | Required state(s) | Static expectation | E51 acquisition / acceptance / remaining proof |
|---|---|---|---|
| `ENC-UI-01` | Alliance context-free index | `0x6f` full cache; Alliance shell/rail; shared index overlay `0x2862`, baked text resource `0x1843`, background selector `2`; list focus. | **Partial acquisition / Open:** F7, post-gate `0x131`, duplicate-F7 visible-child evidence, and a live ordinary command-`0x19` mode-1/current-null child observation exist. R56 adds two equal bounded target-thread samples focused on the validated initial-index child. Pre-gate command, runtime resources, and instrumented duplicate-child count remain absent. |
| `ENC-UI-02` | Empire context-free index | Empire shell/rail with the same index overlay `0x2862`, baked text resource `0x1843`, and background selector `2`. | **Partial acquisition / Open:** F7 and pre/post behavioral `0x131` evidence exist; no direct `FUN_004fcee0` value, runtime resources, focus, or instrumented child identity/count. |
| `ENC-UI-03` | Alliance commands `0x6f..0x75` | Seven fixed x slots; proven full/range collections and stable ordering. | **Partial acquisition / Open:** visible frames cover all seven selected categories. Reviewed static evidence fixes command order, profile candidate counts 347/200/38/14/15/10/69, and source-order preservation for comparator ties. Complete runtime-admitted membership/order, empty/gap outcomes, selected resources, and focus remain absent. |
| `ENC-UI-04` | Empire commands `0x6f..0x75` | Same command/range contract with faction-specific icon resources where mapped. | **Partial acquisition / Open:** visible frames cover all seven selected categories. Reviewed static evidence supplies the same fixed order, candidate counts, and tie rule. Complete runtime-admitted membership/order, empty/gap outcomes, selected resources, and focus remain absent. |
| `ENC-UI-05` | Normal, pointer-over, captured press, canceled press, selected, disabled | Source bitmap slots and capture behavior above. | **Partial acquisition / Open:** selected Category/Close sequences exist; exhaustive mode/navigation/disabled states and timing remain absent. |
| `ENC-UI-06` | Index selection by click, double-click, and Return | Static emitter paths produce `0x29b`, repeat-click `0x29d`, and double-click/Return `0x309`; the receiver conditionally maps `0x309` to `0x67` only with non-null selected/current rows. | **Partial acquisition / Open:** click/double-click/Return pixels exist; list PageDown/Return produced live canonical key `5963` / packed identity `0x14000002`. Delivered notification identity, internal focus, and pointer-click/double-click selected-resource joins remain absent. |
| `ENC-UI-07` | Class-context topic across all five direct contextual callers | Canonical definition-derived row opens; body focus. | **Missing acquisition / Open:** bounded visible class results exist for Fleet/Corellian Corvette, mission Recruitment/Espionage, Build Selection/Alliance Escort Carrier, and r58 Character Status/Leia. Reviewed static evidence names all five owners: Fleet `FUN_00486fb0`, mission `FUN_0046c3c0`, Build Selection `0x00438800`, r55 Character Status `0x004443a0`, and r57 Message Index Research Report `0x00467f10`. R59's complete visible Manufacturing list was empty, so Research Report remains fixture-blocked rather than successfully captured. Create/reuse and fresh-created/key-check limits remain as documented. The Personnel entity, mission-definition class, and resolved Character Status class are distinct; Alt+M, Alt+B, and F6 causality remain unisolated. Live command/notification, selected resources, canonical current keys, action-specific focus, and an eligible Research Report row/article/close sequence remain absent. |
| `ENC-UI-08` | Direct entity-context topic across all five direct contextual callers | Same canonical-row topic flow. | **Missing acquisition / Open:** all five caller/control, entity/definition/current-key, and focus subcases remain absent. |
| `ENC-UI-09` | Special-range entity fallback and non-special miss | Proven fallback may resolve; non-special retry cannot invent another key. | **Missing acquisition / Open:** both fallback branches and a non-special miss lack a source-backed setup. |
| `ENC-UI-10` | Unavailable/stale context | Index mode, no fabricated current topic. | **Missing acquisition / Open:** unavailable/stale title/list/mode/focus fixture absent. |
| `ENC-UI-11` | Same applicable topic as Alliance and Empire | Text identity fixed; both use topic background selector `1` and shared overlay `0x2861`, while faction shell/rail and the proven faction art key differ. | **Partial acquisition / Open:** paired visible topic exists; runtime image/text identities and composition trace remain absent. |
| `ENC-UI-12` | Representative system selectors 1, 24, 25, 26 | Keys `0x2b5c`, `0x2b75`, `0x2b73`, `0x2b74`; text identity fixed. | **Partial acquisition / Open:** Alliance selectors 1/24/25/26 are source-joined to live current identities/body IDs with nonnull art and inspected images. Empire counterparts and a direct original-loader selected-filename/composition trace remain absent. |
| `ENC-UI-13` | System side-view admitted/excluded | Exact type-`0xf2` ancestry predicate controls list presence. | **Missing acquisition / Open:** admitted/excluded pair, view identity, and ancestry chain absent. |
| `ENC-UI-14` | First enabled topic | Backward disabled; a null `0x83` neighbor branches before the `this+0x148` write and retains current; disablement is separate. | **Partial acquisition / Open:** explicit Left in full and filtered collections retained current canonical key `5696` / packed identity `0x1c000002` in equal snapshots. Internal Win32 focus and complete after-action desktop pairs remain absent; native frames are intact but immediate desktop frames caught a Wine/Xvfb repaint gap. |
| `ENC-UI-15` | Middle topic and disabled intermediate row | Both directions enabled; navigation skips disabled row. | **Partial acquisition / Open:** middle topic exists; disabled-row setup and skip identity trace absent. |
| `ENC-UI-16` | Last enabled topic | Forward disabled; a null `0x84` neighbor retains `this+0x148`; disablement is separate. | **Partial acquisition / Open:** explicit Right retained the full last key `6802` / identity `0x38000002` and filtered last key `5699` / identity `0x1c000002` in equal snapshots. Internal Win32 focus and complete after-action desktop pairs remain absent; native frames are intact but immediate desktop frames caught a Wine/Xvfb repaint gap. |
| `ENC-UI-17` | General current-topic null-art branch; no supported-profile fixture | No EData blit when art is null; header/body remain independently populated. Reviewed corpus accounting proves 347/347 supported-profile rows have effective art (331 direct/system plus 16 complete viewer-faction pairs) across 186 bound files. | **Missing historical acquisition / Open; supported profile corpus-not-applicable:** no original-profile no-art topic is demanded. `EDATA.192` is unbound/deferred, not a fixture. The general branch remains untested and E26 mod-null is separate. |
| `ENC-UI-18` | Short, wrapped, explicit-newline, long-token, and scrolling body | Read-only `0x2410` layout, scrollbar only when needed; Up/Down/Page keys scroll. | **Partial acquisition / Open:** r41 Chewbacca showed first movement on discrete Down 4 and reversal on Up 4; Up 5-8 were interrupted by Message Index and rejected. A separate down-arrow click moved immediately; an attempted pointer drag showed no motion and is not drag proof. Static metadata joins Chewbacca to `0x38000343`, title `10819`, lookup `6723`, `EDATA.081`. Newline/long-token/body Page/font/extent/internal-focus and successful drag cases remain absent. |
| `ENC-UI-19` | Topic Return, Tab, Escape, and `0xfb` | Return has no local topic action; Tab consumed; static command `0xfb` dispatches to the same vtable `+0x30` close method as Escape. | **Partial acquisition / Open:** visible Return/Tab/Escape/Close results exist; r7 Escape has a two-snapshot command-`0x19` child-absent observation; r41 Close restored Fleet 1; r47 Close restored the Recruitment and Espionage mission dialogs with the Personnel window retained; r53 Close restored Build Selection on Alliance Escort Carrier over Chandrila Shipyards; and r58 Close returned to Personnel with Leia selected while Character Status was no longer visible. R58 does not prove hidden destruction order. Runtime `0xfb` command/notification and internal focus restoration remain absent. |
| `ENC-UI-20` | Index Left from a middle category, the first category, and across hidden predecessor candidates | Select first visible predecessor; immediate null reselects the tree's first child; running off the start only after hidden candidates retains current; list focus remains. | **Partial acquisition / Open:** visible middle/first cases exist; hidden predecessor and complete command/list/focus trace remain absent. |
| `ENC-UI-21` | Index Right from a middle category, the last category, and across hidden successor candidates | Select first visible successor; immediate null at the last child wraps to the tree's first child; running off the end only after hidden candidates retains current; list focus remains. | **Partial acquisition / Open:** visible middle/last-wrap cases exist; hidden successor/leftmost and complete command/list/focus trace remain absent. |

The acquisition rollup remains 15 partial / 6 missing / 0 accepted. That is a
historical acquisition and row-acceptance count, not an applicability count.
`ENC-UI-17` remains in the historical missing bucket while its original-topic
capture is explicitly corpus-not-applicable for the supported profile; the
general null-art branch stays untested without blocking on a fictional topic.

Coordinator-verified r41 evidence is additive to, not part of, the original
198-PNG manifest: 75 raw full-desktop frames plus 75 exact same-acquisition
client crops have sorted identity digest
`36f046e7c9d9672887df3027827811ed5605a0876365b34726d2f6539c55d28e`.
The r41 report/checker SHA-256 values are
`60fa5c5ae076c07360b7cc0a97185181fa34691357c8b1d23deda997ade9abf8`
and `0ff4fcb8e3346d4bea002a53caf7b741b9fdf8e518bd05355eae22fc92f362ba`.
The independently reviewed r42 report/source-note/checker hashes are
`3c955d02f686b004ffa143ef21774f6033c7eaf69d1f35ab06da430d93ce455b`,
`9147007d8bff711d9d78b37589f92b11ab576ee53f7d91db7d7aa95ce68ea7c0`,
and `9b53a58dada0265dd5d2841964e2cd440336aa6ba0942cb1ecd792808b0ae68e`.
The independently reviewed r43 Fleet route report/source/checker/review hashes
are `fd32ddc0e98a8b451d42a3c1544ef41adece418f3ec3f4a294244a0b7fb24cc9`,
`85a7a02df550b084aaebef67565a9bb3cdee348c42e2b4886c1c1db1095b51e9`,
`8ed791fdb84bb85ccf371555bce99d0e35e675ce5fc054ae22e87d0a2a75fd95`,
and `e40d6772bb21103b85eff9bad9d92e40160c21bcffd7b963e007e60a93c9748a`.
Its static route does not establish a live command occurrence, selected
resources, canonical DAT identity, notification delivery, or Win32 focus.

The independently reviewed r45 mission route report/source/checker/review
hashes are
`4621912fa106bfd8f5ba433c8a4fdb4ac7d33a1c1d08fd00b590b659a10c2dbe`,
`7fb3b40c4d0288646d7c6b8e23adf3a0d778923b979b185259a1ac08c4ede520`,
`207f78b59f72e9b48e1a4888611f34502e94d525da5f6de4de11c1a8c016f68c`,
and `a0d746f21ff00227d2a0eed8d77ea0943aea22f731f2acc5dbac0f529fdae3bb`.
The independently reviewed r46 upstream setup report/source/checker/review
hashes are
`166d66f446039b8ddf96bb5cd50827f28e77ba3a127a72bf67355fe712ddbd96`,
`1dd786bf12e9f02378e26b5a14329a09266e2ec5cc10a7538a58971027169837`,
`8d7468b75db9b43c5071398a04bcfca392229cef0953955105f0ba86afc4fecb`,
and `7f114fcb62e16b39a6ded1146a047c8085076c6d625eb73d453f80b8d77d4e81`.
The accelerator table/root-loop/active-child forwarding range hashes are
`3decf87850057749ba7041949f7402420039e24fad3869fe20f0a0e9ec9f7d65`,
`319355da35d2c57d4fcba65072c0033b7a6c85c9ed3d403826385345df6e7da8`,
and `df0179ee6aadc6155ca32f41c0e4edba24e93f05bc22b1d8be173ffd5c55f747`.

Coordinator-accepted r47 evidence is append-only to every earlier inventory:
70 raw full frames and 70 exact same-acquisition crops have sorted identity
digests `8fd989a44bd4fa5f8fd281da7437ef267a687b1c315080170801554bd128e631`
and `9666c639bf6528e99c6014bc19077c72c4bf3834614e56b2bc7badf9697444e1`.
Its report/checker/action-journal hashes are
`538a0d3f5b6e71b361e0f9b37031fb8255bc1b3634077dd4255e6358a532acdf`,
`1c75f64daf9296546fe826b670af0ffedc711e68663402b352d5726356bd9d89`,
and `de324268a33f48bf6c4901f68c3755fe0ba9d233587c8beeed61651f2a91fb7c`.
The two Alt+M bracket frames `055`/`056` are byte-identical at
`8ff85f6dcfffed2c73b8bedc121140981dddf5e942f24f81faa08c24f861f97b`;
the later post-double-click dialog frame `057` is
`802b2e74d4963881a3af120ea84c14d732753a680aa16809c9c03bf7a31024c5`.
The source-only hotkey chain and accepted downstream visible mission-context
result are therefore recorded separately.

The accepted r49 report/source/checker/review hashes are
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

Coordinator-accepted r53 adds 35 raw full frames and 35 exact
same-acquisition crops under `visible-r53-1/`. Its report/checker/manifest/
action-journal hashes are
`596b732d5925632a6a855ce35252592dfdb609706ecd902a977b867004b101d4`,
`d05db1e6d73babf700c5e6276049e84f399e6ac42f7e5a36d499c9016dd29cd8`,
`6b0fe05f26222a4f88a69637900680403ffc0cb634cb1896d4f07647dde73dbe`,
and `baefeb1e52ca31d4ca67e1de792cdbea8b3e9e522144db476b4c2769ca82274c`.
Key full-frame hashes for selected Alliance Escort Carrier, matching article,
and surviving Build Selection are respectively
`3abec7b2221f0819160e46e21bf9e3451dd762a31e90b584b064a22143a751ea`,
`3899decfb20b1b1e937e2d432c539444afbb439feb63a33876356f59b00a5b00`,
and `4c49f5fe41a59cbfc117f038c87e42f4cda245ed7cd5f9be926f99d6f606d505`.
Launcher and exact-prefix stop both returned zero; the launcher was reaped and
the guard was cancelled with its PID absent. Guard wait/reap exit remains
unknown; lifecycle-shell exit `143` is not a guard-exit substitute.

Accepted r55 and r57 close the two source-owner naming gaps without closing a
runtime row. The Character Status source note SHA-256 is
`ff7a0c7b58d0e1d4d29ebbf9fe022281b7766a2e24e32e2b447e47c0fe6b2b97`;
the Message Index Research Report source-note SHA-256 is
`5703716043c4db73e3d4c1d59fa84954b079f532da2ca7e0f057efc725b3170b`.
R56 separately records one bounded Wine initial-index focus observation at
SHA-256
`5de023bb410b25abf34d81bbb08e922a32e83cae19b099955fd71af8d38ea4bf`:
two equal target-thread samples focused the validated index child, and helper
and game cleanup completed within bounds. It supplies no loader-resource,
notification, selected-key, contextual-action, or Windows proof. The older r35
quarantine remains immutable at
`ab5d3c77ea52326b7152ee80b3120887146dd07e2549deb2687dc4ad129f24a7`.

R58's accepted visible manifest SHA-256 is
`1198943b9f390124cc247a13f02740d710917ecf9f57dbc0d9e96acf114154d7`:
Character Status / Leia opened a matching article and encyclopedia-only close
returned to Personnel with Leia selected. Hidden Status destruction/order is
not observed. R59's verified-inconclusive manifest SHA-256 is
`d1af9b876dc93f05b5c6e1ed67a8c7690915930c8dd35fe68d18435a235c5403`:
Message Index appeared later and Manufacturing's complete visible list was
empty. It proves no F6 causality or Research Report article/close behavior.
Its lifecycle records retain supervisor `143`; launcher and guard exit/reap
remain unknown, and incomplete launcher output is not reinterpreted.

E51 r7 adds a bounded read-only observer and immutable interaction runs without
changing row acceptance. The metadata summary is
`/data/projects/open-rebellion/agent-work/original-game-capture/E51/reachable-captures-r7-summary.json`
(SHA-256
`bb39f107c4b2cf44c04391559b4a9c893f3044739b3f3ffe4e8e5bd3f2b1afaa`).
It retains exact process identity, two equal snapshots per checkpoint, current
keys/packed identities, endpoint links, selector joins, locale, actions, and
full/native frame hashes. It also explicitly records that source-expected
focus is not a live Win32 `GetFocus` observation and that the original art
loader's chosen filename was not read.

The immutable r7 snapshot field `current_topic.dat_id` is a superseded label
for `(row + 0x68) & 0x00ffffff`, not a general original DAT identity. Future
observer output calls it `packed_handle_low24`. Original DatId attribution
requires a unique canonical-key/profile join; the System DatIds in the r7
summary remain source-proven only through the accepted E39 join. Qualification
artifact `ui-state-packed-handle-field-provenance-r8.json` has SHA-256
`ab6b32ab15d771bb403f4ef179b27faabf89ca4c6a5b4524f94d06776d180b5c`;
it preserves every raw snapshot/run and records no recapture.

The source-guided exceptional investigation remains open for exact named
prerequisites: runtime command/resource/current-key/action-focus proof for the
visibly exercised `0x00438800`, `0x004443a0`, `FUN_0046c3c0`, and
`FUN_00486fb0` class routes; a legitimate eligible Research Report for the
source-proven `0x00467f10` route; both
`FUN_0045fd90` association branches and a non-special miss; a legitimate
stale/unavailable context; a System view with type-`0xf2` ancestry; and a
connected writer/configuration hiding a category.
Retained fresh-viewer cache runs had zero System exclusions, all four proven
selector topics had art, no suitable owned save was present, and all seven
categories were visible. Those bounded observations are not universal absence
proof and do not authorize fabricated fixtures.

Guide 018 leads the index composition, guide 019 leads a character topic, and
guide 027 leads a ship topic with stat-like authored text. Their SHA-256 values
are, respectively, `0769ec7407f75133d1f85460d995ab976f0039540aec85b676381f131a0f3be8`,
`d74940b352a5e7102e0549e727c24daf70bad3593df72c85c847cbe4bce8c6c7`,
and `1c7c7264f3b57a2d4875671c0fedf7bbde21e2ed88692f7f4cd83c1796a86e78`.
They are A3 and cannot close any matrix row.

## Explicit gaps and change control

- E51 supplies the bounded partial A0 set linked above. It accepts no complete
  row: 15 are partial acquisitions and six are missing, so
  `runtime_capture_required` remains true.
- E51 visually joins the inspected English labels to commands `0x6f..0x75`
  through its recorded click sequence. Their backing localized string-resource
  identities remain unjoined; other profiles must not inherit these labels
  without evidence.
- Five direct callers of `FUN_0041d6b0` are source-proven. The Fleet/generic
  `FUN_00486fb0`, mission-dialog `FUN_0046c3c0`, Build Selection `0x00438800`,
  Character Status `0x004443a0`, and Message Index Research Report
  `0x00467f10` class-context routes now have reviewed static chains. The first
  four have bounded visible results; r59 found no eligible Research Report.
  All still lack live command/notification, resource/current-key, and
  action-specific target-thread-focus proof. R56 focus applies only to the
  initial index child.
- The r46 Alt+M chain is a conditional mission-setup path, distinct from the
  downstream mission-definition encyclopedia control. An uncontaminated
  selected-character / Alt+M / bounded-after capture would close that causal
  observation, but it is not a universal encyclopedia acceptance gate and
  must not invalidate the already-observed mission articles/close behavior.
- The button path proves no separate hover bitmap. E51 captures selected
  Category and Close pointer/press/cancel states, but exhaustive mode and
  navigation control coverage in both factions remains open.
- The supported profile needs no null-art fixture: all 347 bound rows have an
  effective selection. Preserve the generic source paint branch as untested,
  do not create a fake original topic, and keep E26 mod-null separate.
- Unused lookup IDs `7188` and `11284` are not unused files:
  `EDATA.142`/`EDATA.143` are bound through lookup IDs `7200`/`11296`.
- E51 captures a real long-body scrollbar change with current identity
  retained. Two source-bounded PageDown attempts changed the selected list row,
  so exact body Page-key, explicit-newline, long-token, extent, font-resource,
  and internal-focus cases remain open.
- The category keyboard path supports skipping hidden child windows, but no
  original encyclopedia state that hides a category has yet been connected.
  E51 should exercise hidden-child branches only after recovering such a
  configuration; it must not manufacture hidden controls as original proof.
- Unproven alternate artwork, including `EDATA.192`, remains inventoried and
  unused under deferred task `orlocal-2kq`. This contract defines no alternate
  predicate, binding, switch, or acceptance row.

If an A0 presentation contradicts the frozen identity, availability, context,
navigation, faction, or system-selector semantics, reopen the source/schema
review before changing a renderer or catalog consumer. A screenshot alone must
not replace the connected source rule.
