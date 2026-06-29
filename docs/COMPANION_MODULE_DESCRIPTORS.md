# Companion Module Descriptors

Rusty GUI owns frontend-neutral descriptors for companion app modules and
workspace composition. These descriptors let WPF, Makepad, CLI, and later
frontends render the same capability map without becoming command, stream, or
runtime authority.

## Schemas

- `rusty.gui.companion.module_descriptor.v1`
- `rusty.gui.companion.workspace_descriptor.v1`
- `rusty.gui.companion.transport_capability.v1`

## Module Families

The stable companion module families are:

- `readiness_preconditions`
- `quest_device_link`
- `quest_install_launch_profile`
- `runtime_effective_state`
- `manifold_command_feedback`
- `stream_telemetry_state`
- `media_remote_camera_plane`
- `artifact_session_evidence`
- `operator_study_shell_adapter`
- `makepad_adapter_settings`
- `quest_app_panel_bridge`

## Authority Boundary

GUI descriptors can be `descriptor`, `inspector`, `requester`, or `adapter`
surfaces. They cannot claim `authority`.

State-changing actions bind to external command ids and authority lanes, usually
Manifold for command/session/stream work. Readiness and host execution reports
are produced by Hostess. Runtime effective state is reported by the consuming
runtime or settings owner, not by the GUI.

The readiness fixture advertises ADB as the required setup transport and the
Manifold WebSocket broker as an optional transport/state precondition. Hostess
reports whether the broker package, process, ADB forward mapping, forwarded
local socket, and direct host socket are ready; GUI descriptors only tell
frontends that this state exists.

The current bridge-probe command fixture includes both the generic Manifold
WebSocket route and the Hostess bring-up route
`bridge_route.command.android_broker_authorized_app_private.applied`. The
second route still names Manifold as command authority: Hostess may stage the
app-private runtime request only after broker acceptance, and the Quest runtime
must still emit applied evidence.

The Hostess Makepad validation workspace is intentionally shared by WPF,
Makepad, and CLI. Every module selected by that workspace, and every transport
required by those modules, must advertise the same frontend support before the
workspace can claim Makepad parity. This lets a Makepad panel render readiness,
command receipt, and compact evidence status from the same Hostess/Manifold
reports without becoming command, setup, or runtime authority.
The same closure rule applies at the module level: any module that advertises a
frontend must only require transport descriptors that advertise that frontend.
Otherwise frontend-filtered catalog reports can include modules while omitting
their required transport evidence.

## Current Fixtures

Valid module/workspace fixtures:

- `fixtures/descriptors/companion-module-readiness.json`
- `fixtures/descriptors/companion-module-command.json`
- `fixtures/descriptors/companion-module-stream.json`
- `fixtures/descriptors/companion-module-evidence.json`
- `fixtures/descriptors/companion-module-study-workspace-adapter.json`
- `fixtures/descriptors/companion-workspace-hostess-makepad-validation.json`

Valid transport capability fixtures:

- `fixtures/descriptors/transport-capability-adb.json`
- `fixtures/descriptors/transport-capability-websocket.json`
- `fixtures/descriptors/transport-capability-udp-osc.json`
- `fixtures/descriptors/transport-capability-lsl.json`
- `fixtures/descriptors/transport-capability-app-private-staging.json`
- `fixtures/descriptors/transport-capability-media-plane.json`

Damaged fixtures:

- `fixtures/damaged/companion-module-empty-id.json`
- `fixtures/damaged/companion-module-authority-claim.json`
- `fixtures/damaged/companion-module-gui-command-authority.json`
- `fixtures/damaged/companion-workspace-unknown-module.json`
- `fixtures/damaged/transport-capability-media-low-rate-json.json`

## Transport Capabilities

Transport capabilities are frontend-readable route descriptions. They name the
family, plane, delivery semantics, payload rate, external route ids, expected
evidence stages, strengths, costs, and suitable or unsuitable use cases.

The first families are:

- `adb`
- `websocket`
- `udp_osc`
- `lsl`
- `app_private_file_staging`
- `media_data_plane`

High-rate media capabilities must use `high_rate_binary` payloads and the
`media` plane. A media data-plane descriptor that claims `low_rate_json` is
invalid, because command JSON is not a frame transport.

Composite bring-up routes may reference more than one transport capability.
For example, the broker-authorized Android probe uses WebSocket for Manifold
authority review and app-private staging for Quest runtime delivery. The GUI
descriptor does not collapse those stages into one UI-local authority.

## Adapter Rule

Adapters render descriptors and submit requests through backend routes. They do
not define new semantics for command acceptance, readiness status, transport
success, runtime acceptance, applied behavior, evidence validity, or redaction.
