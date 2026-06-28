# Rusty GUI Architecture

Rusty GUI owns renderer-neutral and toolkit-neutral graphical interaction
descriptors.

## Ownership

- panels and inspectors;
- widget descriptors;
- graph canvas descriptors;
- layout and theme hints;
- command binding descriptors.
- companion module and workspace descriptors for frontend-neutral operator app
  composition.
- companion transport capability descriptors for frontend-neutral route choice
  and transport cost/benefit display.

## Non-Ownership

- command/session authority;
- Makepad widget implementations;
- renderer or platform frame lifecycle;
- Quest/OpenXR settings;
- downstream app workflows.

Adapters consume GUI descriptors and bind them to a toolkit. They do not move
toolkit dependencies into this repo.

## Companion Descriptors

The companion descriptor slice defines reusable module/workspace composition for
Windows companion apps, Makepad panels, CLI inspectors, and future frontends.
It is descriptor-only:

- Manifold owns command, session, stream, route, broker, and authority ids.
- Hostess owns host-side readiness, execution, launch, staging, and evidence
  reports.
- Quest and Makepad runtimes own effective-state receipts.
- GUI descriptors name those external reports and command routes, but do not
  accept, apply, or validate runtime behavior.

The companion descriptor map lives in `docs/COMPANION_MODULE_DESCRIPTORS.md`.
