# Rusty GUI

Rusty GUI is the Morphospace lane for portable graphical interaction
descriptors: panels, widgets, inspectors, graph canvases, controls, command
bindings, layout hints, and themes.

This repo is intentionally framework-neutral. Makepad adapters belong in
`rusty-makepad`; Quest-hosted Makepad apps belong in `rusty-quest-makepad`.

## Crates

- `rusty-gui-model`: descriptor types for panels, widgets, controls, and
  command bindings. It also contains companion module/workspace descriptors
  used by WPF, Makepad, CLI, and future frontends to render shared capability
  modules without becoming command or runtime authority.

## Current Descriptor Families

- Panel descriptors: `rusty.gui.panel_descriptor.v1`
- Companion module descriptors: `rusty.gui.companion.module_descriptor.v1`
- Companion workspace descriptors:
  `rusty.gui.companion.workspace_descriptor.v1`
- Companion transport capability descriptors:
  `rusty.gui.companion.transport_capability.v1`

See `docs/COMPANION_MODULE_DESCRIPTORS.md` for the companion module taxonomy
and fixture map.

## Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```
