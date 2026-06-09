# Rusty GUI Agent Notes

This is the clean source repository for Rusty GUI. Keep committed content
self-contained and free of local-only planning paths, downstream app names,
platform-specific runtime handles, and historical naming drift.

Rusty Morphospace is the top-level project/platform umbrella. This repo remains
the GUI lane inside that umbrella: morphology of direct graphical operation,
including panels, widgets, inspectors, graph canvases, controls, command
bindings, layout hints, and themes. Do not introduce `rusty.morphospace.*`
schemas here; use `rusty.gui.*` for GUI contracts.

Project-owned source in this repo is licensed `AGPL-3.0-or-later`. Keep
third-party dependencies, assets, generated binaries, and external tools under
their own provenance and notice requirements.

## Purpose

Rusty GUI owns portable graphical interaction descriptors. It should remain
usable without Makepad, renderer backends, Quest/OpenXR tooling, Android
properties, runtime sockets, media stacks, or downstream app crates.

## Read Order

1. `README.md`
2. `docs/ARCHITECTURE.md`
3. `docs/VALIDATION.md`
4. `fixtures/README.md`

## Architecture Rules

- GUI describes graphical controls and layout. It does not own command/session
  authority, leases, accepted revisions, or runtime mutation.
- Every state-changing GUI action must bind to an external command id or route
  owned by the appropriate authority, usually Manifold.
- Makepad, web, native, and other toolkit adapters belong outside this repo.
- Use `rusty.gui.*` schema ids for default GUI contracts.
- Keep app-specific workflows in Studio, Hostess, Quest, or product repos.
- Add fixtures and damaged-input expectations before adding adapter behavior.

## Validation

Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

