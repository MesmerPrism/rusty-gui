# Rusty GUI Fixtures

Fixtures document descriptor shapes for panels, widgets, and command bindings.
They are stable examples for adapters and tests.

Companion fixtures in `fixtures/descriptors` define reusable module/workspace
composition for readiness, commands, streams, evidence, and study adapters.
Damaged companion fixtures in `fixtures/damaged` prove that GUI descriptors
cannot claim command/runtime authority or reference unknown modules.

Transport capability fixtures describe ADB, WebSocket, UDP/OSC, LSL,
app-private staging, and media data-plane route families for frontend route
selection. Damaged transport fixtures prove that high-rate media cannot be
collapsed into low-rate JSON command payloads.
