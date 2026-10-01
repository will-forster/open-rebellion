# Modding Open Rebellion

Open Rebellion loads its simulation data from the original game's `.DAT`
files, then lets you patch the resulting in-memory world with JSON overlay
files — no Rust required. This document covers how to write, install, and
test a mod. For the runtime implementation details, see
`agent_docs/modding.md` and `agent_docs/mod-runtime.md`.

## 1. Generate your reference data first

Before you can write a patch you need two things: the numeric `dat_id` of
the entity you want to change, and the exact field names/current values on
it. Both come from dumping the original `.DAT` files to JSON.

**Via Docker** (see the main `README.md` Docker section for the base setup):

```bash
# In .env:
PREPARE_MODDING=1

docker compose up
```

This runs an extra step in the `builder` service that writes:

- `data/base/json/<TABLE>.json` — one file per original `.DAT` table
  (e.g. `CAPSHPSD.json` for capital ships), with every field the game reads.
- `data/base/json/textstra.json` — `{ "<string_id>": "Display Name" }`,
  looked up via each entity's `text_stra_dll_id` field.

`PREPARE_MODDING` defaults to `0` (skipped) since most people don't need it;
turn it on when you're about to write a mod. It's idempotent to rerun.

**Natively**, the equivalent is:

```bash
cargo build -p dat-dumper --release
./target/release/dat-dumper --gdata data/base --output data/base/json
./target/release/dat-dumper --gdata data/base --extract-strings --output data/base/json
```

Both `data/base/json/` and the game data itself are gitignored — this is
local reference material, not something to commit.

### Stage and inspect canonical encyclopedia identities

The asset staging tool can build a readable, strictly validated encyclopedia
catalog from a supported owned installation. Use placeholders for local roots;
do not copy an installation path into a mod or a committed document:

```bash
OWNED_INSTALL="/path/to/owned-install"
CATALOG_ROOT="$PWD/data/base/encyclopedia"

go run ./tools/stage-ui-assets --encyclopedia-only \
  --source "$OWNED_INSTALL" \
  --edata "$OWNED_INSTALL/EData" \
  --encyclopedia-output "$CATALOG_ROOT"

go run ./tools/stage-ui-assets --encyclopedia-only --verify \
  --encyclopedia-output "$CATALOG_ROOT"

# List canonical topic IDs without printing original prose.
jq -r '.topics | keys[]' "$CATALOG_ROOT/catalog.json"

# Show family-qualified bindings for one selected topic.
TOPIC_ID="original:5696"
jq --arg id "$TOPIC_ID" \
  '.bindings[] | select(.topic_id == $id)' \
  "$CATALOG_ROOT/catalog.json"
```

Inspect `data/base/encyclopedia/catalog.json` for canonical topic IDs,
family-qualified `{family, dat_id, variant}` bindings, potential index/category
membership, localized text, and base image IDs. These preserve original DAT
and DLL/resource identities; runtime handles are not replacement IDs. View
arrays are source membership plus tie order, not frozen displayed-title order.

`manifest.json` is immutable extraction provenance and hashes only the runtime
catalog plus referenced base art. `source-report.json`, `raw/`, and
unreferenced `assets/` files are local research evidence rather than mod inputs.
Do not edit or distribute generated base prose/art. Author content belongs in a
mod's separate `encyclopedia.json` and confined `encyclopedia/assets/` paths
and must never replace the generated base directory.

The identified profile accounts for all 348 decoded ENCYTEXT records: 347 are
bound topics and resource `7176` is source-proven unused. There are no v1
runtime aliases or unresolved text records. `EDATA.192` is a separate,
inventory-only deferred artwork record; file presence does not create a topic,
selector, or supported alternate-art rule.

### Finding an entity's `dat_id`

Open the relevant JSON file and find your entity by name, cross-referencing
each record's `text_stra_dll_id` against `textstra.json`. The `id` field in
that record **is** the exact number to use as `"id"` in your overlay patch —
it's written into `DatId` unmodified at load time (`dat_id: DatId::new(dat.id)`
in `crates/rebellion-data/src/lib.rs`), with no re-encoding. These are small,
plain, per-table sequential numbers, not a bit-packed scheme — don't guess one.

Example, from a real `CAPSHPSD.json` entry:

```json
{ "id": 133, "text_stra_dll_id": 10117, "hull": 2750, "shield_strength": 300, ... }
```

`textstra.json["10117"]` resolves to `"Imperial Star Destroyer"`, confirming
which record this is.

## 2. Which `.DAT` table feeds which overlay filename

Your overlay file's name must match a `GameWorld` field name **exactly** —
there is no aliasing or fuzzy matching. Get it wrong and the mod loader
silently skips the file with an `unknown arena` warning.

| Overlay filename | Source `.DAT` table (for `dat_id` + field lookup) | Notes |
|---|---|---|
| `capital_ship_classes.json` | `CAPSHPSD.DAT` | Star Destroyers, cruisers, etc. |
| `fighter_classes.json` | `FIGHTSD.DAT` | X-wings, TIEs, etc. |
| `characters.json` | `MJCHARSD.DAT` (named) + `MNCHARSD.DAT` (generic) | Both feed the same `characters` arena |
| `troop_classes.json` | `TROOPSD.DAT` | Ground unit stats |
| `defense_facility_classes.json` | `DEFFACSD.DAT` | Planetary shields/defenses |
| `gnprtb.json` | `GNPRTB.DAT` | Global balance constants; overlay `id` selects the matching `parameter_id` entry |
| `sdprtb.json` | `SDPRTB.DAT` | Per-side startup parameters; overlay `id` selects the matching `parameter_id` entry |

The full, authoritative field list for `GameWorld` (and thus every other
possible overlay target) is `crates/rebellion-core/src/world/mod.rs` —
search for `pub struct GameWorld`.

## 3. Directory layout

```
mods/
└── my-mod/
    ├── mod.toml                  # manifest (required)
    └── capital_ship_classes.json # one file per entity category you patch
```

### `mod.toml`

```toml
name = "my-mod"
version = "1.2.0"
author = "you"
description = "What this mod does."

[dependencies]
"some-other-mod" = ">=1.0.0"
```

| Field | Required | Notes |
|---|---|---|
| `name` | Yes | Must be unique. Kebab-case is the authoring convention, but the current resolver preserves the exact UTF-8 bytes and does not enforce that spelling. |
| `version` | Yes | Semver, e.g. `"1.2.0"` |
| `author` | No | Display only |
| `description` | No | Display only |
| `dependencies` | No | mod name → semver requirement; validated and topologically sorted (Kahn's algorithm) at load time |

## 4. Writing the overlay JSON

Each file is an array of patch objects, merged via **RFC 7396 JSON Merge
Patch**:

```json
[
  { "id": 133, "hull": 3000, "shield_strength": 2000 }
]
```

Rules:
- `"id"` is required and must match the target entity's `dat_id`, or the
  parameter entry's `parameter_id` for `gnprtb.json` and `sdprtb.json`.
- A field you include **overwrites** that field.
- A field set to `null` **deletes** it.
- Any field you omit is **left untouched**.

See the worked example at `mods/examples/star-destroyer-rebalance/` — it
raises the Imperial Star Destroyer's hull to 3000 and shields to 2000.

### Encyclopedia text and artwork

Encyclopedia changes use a separate root file named `encyclopedia.json`.
Selectors are canonical string topic IDs from the staged catalog, not numeric
world entity IDs. Only localized `title`, `body`, and `image` fields are
author-editable; categories, bindings, provenance, canonical image IDs, source
hashes, and base descriptors are protected.

For example, this changes one English title/body and replaces its art with
author-owned PNG bytes:

```text
mods/my-encyclopedia-mod/
├── mod.toml
├── encyclopedia.json
└── encyclopedia/
    └── assets/
        └── interceptor.png
```

```json
[
  {
    "id": "original:5696",
    "localized": {
      "1033": {
        "title": "My Interceptor",
        "body": "Author-written replacement text.",
        "image": { "path": "encyclopedia/assets/interceptor.png" }
      }
    }
  }
]
```

The runtime reads and validates the exact confined BMP/PNG bytes, then computes
their digest, dimensions, format, and collision-free mod image identity. Do
not put a hash, dimensions, `image_id`, or base path into the patch. Paths are
relative to that mod's directory, use `/`, and must remain within
`encyclopedia/assets/`.

The generated identity is
`mod:v1:<lowercase-hex-of-exact-UTF8-mod-name>:<path>`. It is not an authoring
field. For example, `demo` becomes
`mod:v1:64656d6f:encyclopedia/assets/interceptor.png`, while `MyMod` becomes
`mod:v1:4d794d6f64:encyclopedia/assets/interceptor.png`. Names are not
case-folded or Unicode-normalized. There is no independent fixed mod-name
length limit; checked allocation and the single retained-byte budget decide
whether an encyclopedia candidate can construct its identity. Paths are a
separate, ASCII-only contract: 25–256 bytes, slash-separated, beginning with
`encyclopedia/assets/`, with no empty, `.`, `..`, absolute, drive, colon, or
backslash component, and ending in lowercase `.bmp` or `.png`.

Presence matters:

- Omitting a field inherits the preceding effective value.
- `"body": ""` deliberately publishes an empty body.
- `"image": null` deliberately removes art; it does not fall back to base or
  faithful-HD art.
- `"title": null`, `"body": null`, or deleting a whole required localized
  record is rejected when it would leave incomplete content.
- A viewer-faction image pair must supply both `alliance` and `empire` sides
  and is accepted only for a source-proven faction-capable topic. Do not infer
  that capability from a name or existing picture.

A complete pair has this shape; either side may be `null`, but neither key may
be omitted:

```json
[
  {
    "id": "original:7184",
    "localized": {
      "1033": {
        "image": {
          "alliance": {"path": "encyclopedia/assets/mission-alliance.png"},
          "empire": {"path": "encyclopedia/assets/mission-empire.png"}
        }
      }
    }
  }
]
```

Permission comes from the immutable validated base topic capability. A mod can
use a pair in a newly added language for one of those proven topics, and can
restore the pair after an earlier mod supplied static or null art. A static
topic can never gain pair capability from an earlier overlay. Any missing,
unsafe, corrupt, oversized, or mismatched side rejects that mod's entire batch.

Only `localized.<LANGID>.title`, `body`, and `image` are accepted. V1 has no
aliases, custom topics, categories, bindings, author-supplied IDs, hashes, or
provenance fields. See the normative
[`encyclopedia-overlay.schema.json`](docs/reference/asset-library/schemas/encyclopedia-overlay.schema.json)
and the contributor-authored
[`overlay fixtures`](tests/fixtures/encyclopedia/fixtures/overlays/).
Validate the checked-in synthetic schema and relationship corpus with:

```sh
npm --prefix tools/interface-parity ci
node tools/interface-parity/validate-encyclopedia-fixtures.mjs
```

That command validates the repository fixtures; it is not a substitute for
the native runtime validating a mod's exact retained bytes. `jq empty
mods/my-mod/encyclopedia.json` is useful as a syntax check, but it likewise
does not prove schema, image, provider, budget, or base-capability validity.

### Language, ordering, and limits

All overlays are applied before choosing a language. The runtime selects the
complete requested LANGID record, or the complete catalog default record; it
never combines a title from one language with a body or art from another.
Deleting the requested-language record restores whole-record fallback. If both
requested and default records are absent, the topic is disabled with a
diagnostic. Category labels fall back independently; when both labels are
missing the category is disabled. That last behavior is a deliberate robust
port divergence from the original's empty-label display. A present empty label
remains empty.

Catalog membership arrays provide source membership and stable tie order, not
display-title order. After overlays, language choice, and art resolution, rows
are stable-sorted by their effective displayed titles. Windows-1252-
representable titles sort first using strict encoding and ASCII-only `A`–`Z`
folding; high bytes are unchanged. Other titles use pinned Unicode 15.1
per-scalar lowercase UTF-8 without normalization and sort afterward. Equal
keys retain registry order. This is a deterministic compatibility policy, not
a claim that the original CRT implements Unicode sorting.

Safety limits are implementation budgets, not original-game limits:

- 16 MiB and depth 8 for one overlay, with at most 10,000 patches;
- 64 KiB UTF-8 per title and 1 MiB UTF-8 per body;
- 32 MiB and 16,000,000 pixels per image;
- 128 MiB for the effective logical image set; and
- one 512 MiB retained-byte cap for live base/mod buffers, one serialized
  candidate, in-flight reads, and retained identity/name/path bytes.

The 512 MiB cap is not split into per-mod quotas and is not a total process or
GPU-memory promise. Reload work is serialized; a rejected candidate releases
its reservations and leaves the previous valid publication live.

When two enabled mods edit the same field, dependency order applies first and
the later mod wins. Declare a dependency when that precedence is intentional;
unrelated ready mods use lexicographic exact-name order. A malformed edit is
reported with its mod/topic/path while the last accepted contribution remains
visible if that mod is still eligible. Fixing the file recovers automatically.
Disabling or removing a mod rebuilds from immutable base plus the remaining
enabled snapshots, so a disabled contribution cannot reappear through cached
cumulative state.

Native builds watch `encyclopedia.json` and its declared art. Atomic editor
renames, image-only edits, text edits, removal, and recreation are coalesced at
the content-only refresh boundary; they do not replay simulation/world patches.
Use **Reload Mods** to explicitly rearm after recreating a missing `mods/`
directory. Browser v1 deliberately remains unmodified base-only: it neither
discovers nor watches local mod directories.

Keep base and author content separate. Re-running the staging command updates
only the generated base inventory; it never writes into `mods/`. Conversely,
native reload never writes to `data/base/encyclopedia`. If a supported owned
installation changes, restage and verify the base rather than editing manifest
hashes by hand.

Native loading pairs `manifest.binding_sources` with the exact selected DAT
bytes. With a selected `.../GData` directory it looks for a sibling
`.../encyclopedia`; with a flattened DAT root it looks for a child
`.../encyclopedia`. The lower-level loader accepts an explicit override root,
but the application does **not currently read** the reserved
`REBELLION_ENCYCLOPEDIA_DIR` environment variable. Do not rely on that variable
until its caller wiring lands. A `binding_source_mismatch` means the catalog
belongs to different DAT bytes: restage from the same owned installation rather
than editing a digest or copying a catalog from another install.

## 5. Installing and testing

1. Drop your mod directory under `mods/`.
2. Launch the native build. Mods are discovered on startup.
3. Press **Tab** to open the Mod Manager and enable the mod. The runtime toggle
   persists `mods/config.toml` and immediately rebuilds encyclopedia content.
   Editing `config.toml` behind an already-running process does not toggle its
   live state; restart or use the Mod Manager.
4. Edit `encyclopedia.json` or a declared image. The native watcher coalesces
   text, image-only, and atomic-save rename events into a content-only refresh;
   it never reapplies world patches. A malformed intermediate edit preserves
   the eligible last-good view, and a valid replacement recovers without
   rewriting author or base hashes.
5. Disable the mod in the Mod Manager. The runtime rebuilds from immutable base
   plus the remaining enabled snapshots, so the original content returns and a
   disabled contribution cannot be resurrected from a cumulative cache.

The 14-step native contributor workflow above has passed inspected feature-only
acceptance for both original-parity and faithful-HD precedence, including
malformed-edit recovery, dependency failure, image-only replacement, explicit
null, stable selection, and bounded retained textures/bytes. Player-facing
production encyclopedia routes remain gated pending E32 and final E36 artifact
revalidation. Browser v1 remains immutable base-only and provides no local mod
discovery, toggle, or watcher.

Load order is dependency-first: if mod B depends on mod A, A's patches apply
before B's, and B can override anything A set for the same entity/field.

## 6. Save compatibility

Save files record which mods (name + version) were active when they were
written, as an FNV-1a hash. Loading a save with a different active mod set
prints a mismatch warning — it doesn't block loading, but expect surprises
if the entities the save references have since changed shape.

## Reference

- `agent_docs/modding.md` — manifest/overlay spec
- `agent_docs/mod-runtime.md` — `ModRuntime`/`ModLoader`/`ModWatcher` internals
- `crates/rebellion-data/src/mods.rs` — implementation
- `mods/examples/star-destroyer-rebalance/` — minimal working example
- `docs/reference/asset-library/encyclopedia-schema-decisions.md` — approved
  identity, sorting, localization, budget, and capability decisions
- `docs/reference/asset-library/schemas/encyclopedia-overlay.schema.json` —
  strict author-facing v1 wire schema
- `tests/fixtures/encyclopedia/fixtures/overlays/` — synthetic positive and
  negative examples
- `tools/stage-ui-assets/README.md` — extraction, verification, container, and
  packaging boundaries
