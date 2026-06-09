# Rusty GUI

Rusty GUI is the Morphospace lane for portable graphical interaction
descriptors: panels, widgets, inspectors, graph canvases, controls, command
bindings, layout hints, and themes.

This repo is intentionally framework-neutral. Makepad adapters belong in
`rusty-makepad`; Quest-hosted Makepad apps belong in `rusty-quest-makepad`.

## Crates

- `rusty-gui-model`: descriptor types for panels, widgets, controls, and
  command bindings.

## Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

