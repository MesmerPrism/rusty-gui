//! Frontend-neutral companion module and workspace descriptors.

use serde::{Deserialize, Serialize};

/// Schema id for companion module descriptors.
pub const COMPANION_MODULE_DESCRIPTOR_SCHEMA: &str = "rusty.gui.companion.module_descriptor.v1";

/// Schema id for companion workspace descriptors.
pub const COMPANION_WORKSPACE_DESCRIPTOR_SCHEMA: &str =
    "rusty.gui.companion.workspace_descriptor.v1";

/// Schema id for companion transport capability descriptors.
pub const COMPANION_TRANSPORT_CAPABILITY_SCHEMA: &str =
    "rusty.gui.companion.transport_capability.v1";

/// Stable companion module families used across WPF, Makepad, and future frontends.
pub const COMPANION_MODULE_FAMILIES: &[CompanionModuleFamily] = &[
    CompanionModuleFamily::ReadinessPreconditions,
    CompanionModuleFamily::QuestDeviceLink,
    CompanionModuleFamily::QuestInstallLaunchProfile,
    CompanionModuleFamily::RuntimeEffectiveState,
    CompanionModuleFamily::ManifoldCommandFeedback,
    CompanionModuleFamily::StreamTelemetryState,
    CompanionModuleFamily::MediaRemoteCameraPlane,
    CompanionModuleFamily::ArtifactSessionEvidence,
    CompanionModuleFamily::OperatorStudyShellAdapter,
    CompanionModuleFamily::MakepadAdapterSettings,
    CompanionModuleFamily::QuestAppPanelBridge,
];

/// Reusable companion module families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionModuleFamily {
    /// Host, dependency, toolchain, device, and runtime preconditions.
    ReadinessPreconditions,
    /// Serial-scoped Quest device discovery and host/device link status.
    QuestDeviceLink,
    /// APK, package, activity, launch, foreground, and runtime profile operations.
    QuestInstallLaunchProfile,
    /// Consumer-side effective state, applied settings, markers, and receipts.
    RuntimeEffectiveState,
    /// Low-rate command request, receipt, and applied/rejected feedback.
    ManifoldCommandFeedback,
    /// Low-rate timestamped telemetry and stream freshness.
    StreamTelemetryState,
    /// High-rate media and frame-session status kept outside command JSON.
    MediaRemoteCameraPlane,
    /// Pulled artifacts, evidence bundles, hashes, and redaction state.
    ArtifactSessionEvidence,
    /// Study-specific app roles, session ledgers, and workflow composition.
    OperatorStudyShellAdapter,
    /// Makepad settings controls and effective-state views.
    MakepadAdapterSettings,
    /// Quest-side app panels, questionnaires, and panel receipts.
    QuestAppPanelBridge,
}

/// Transport families exposed as frontend-readable capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionTransportFamily {
    /// Android Debug Bridge over USB or an explicit serial endpoint.
    Adb,
    /// Manifold broker WebSocket command or event route.
    Websocket,
    /// UDP/OSC best-effort telemetry or control route.
    UdpOsc,
    /// Lab Streaming Layer marker or timestamped sample stream.
    Lsl,
    /// App-private file staging and receipt pull route.
    AppPrivateFileStaging,
    /// High-rate media or frame-session data plane.
    MediaDataPlane,
}

/// Coarse plane a transport capability belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionTransportPlane {
    /// Setup, install, launch, and environment checks.
    Setup,
    /// Low-rate command/control requests.
    Control,
    /// Low-rate telemetry and marker streams.
    Telemetry,
    /// High-rate media/frame payloads.
    Media,
    /// Evidence pull or artifact export.
    Artifact,
}

/// Delivery semantics advertised to frontends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliverySemantics {
    /// Ordered reliable request/receipt path.
    OrderedReliable,
    /// Best-effort packet path with freshness/sequence requirements.
    BestEffort,
    /// Timestamped stream path.
    TimestampedStream,
    /// App-private file write/read receipt path.
    AppPrivateFile,
    /// Frame-session path with counters and drop metrics.
    FrameSession,
}

/// Payload rate and shape class for a transport capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayloadRateClass {
    /// Low-rate JSON command or report.
    LowRateJson,
    /// Low-rate timestamped marker or scalar sample.
    LowRateMarker,
    /// High-rate binary frame, packet, or buffer payload.
    HighRateBinary,
    /// File, bundle, or pulled artifact.
    FileArtifact,
}

/// Owning Rusty lane for a descriptor or adapter-facing module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleOwnerLane {
    /// Portable GUI descriptors and layout bindings.
    Gui,
    /// Manifold command, session, stream, and broker authority.
    Manifold,
    /// Host-side execution, setup, launch, staging, and evidence.
    Hostess,
    /// Quest platform runtime profile and device behavior.
    Quest,
    /// Makepad settings and toolkit adapter surfaces.
    Makepad,
    /// Lattice situated-relation descriptors.
    Lattice,
    /// Studio and operator workflow composition.
    Studio,
    /// Private product or study adapter outside clean reusable cores.
    PrivateAdapter,
}

/// Role a GUI descriptor may play relative to runtime or command authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleAuthorityRole {
    /// Descriptor-only module with no side effects.
    Descriptor,
    /// Read-only UI or report inspector.
    Inspector,
    /// Requester that submits commands to an external authority.
    Requester,
    /// Adapter surface over an external authority or backend.
    Adapter,
    /// Invalid for Rusty GUI descriptors; used to reject accidental authority.
    Authority,
}

/// UI frontend families that can render a descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrontendKind {
    /// Windows Presentation Foundation shell.
    Wpf,
    /// Makepad shell or panel.
    Makepad,
    /// Command-line or agent-facing inspector.
    Cli,
    /// Future web or browser-based frontend.
    Web,
    /// Any frontend that can render the descriptor contract.
    Generic,
}

/// Sensitivity tag for module descriptors and evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SensitivityTag {
    /// Public-safe reusable descriptor or example.
    PublicSafe,
    /// Contains local machine, path, or install assumptions.
    LocalMachine,
    /// Contains private study or product composition.
    PrivateStudy,
    /// Contains device-specific identity or evidence.
    DeviceSpecific,
    /// Contains operator/session evidence that may require redaction.
    Evidence,
}

/// A host-side tool or dependency required by a module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolRequirement {
    /// Stable tool requirement id.
    pub id: String,
    /// Human-facing tool name.
    pub name: String,
    /// Whether a missing tool blocks the module.
    pub required: bool,
    /// Optional readiness check id that proves this requirement.
    pub readiness_check_id: Option<String>,
}

/// A device state required by a module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceStateRequirement {
    /// Stable requirement id.
    pub id: String,
    /// Human-facing state label.
    pub label: String,
    /// Whether this state blocks the module.
    pub required: bool,
}

/// A transport required by a module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportRequirement {
    /// Stable transport requirement id.
    pub id: String,
    /// Transport family id, such as adb, websocket, lsl, udp_osc, or media_plane.
    pub family: String,
    /// Whether this transport blocks the module.
    pub required: bool,
}

/// Frontend-neutral descriptor for one transport capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionTransportCapabilityDescriptor {
    /// Schema id for this descriptor.
    pub schema: String,
    /// Stable transport capability id.
    pub transport_id: String,
    /// Human-facing transport title.
    pub title: String,
    /// Transport family.
    pub family: CompanionTransportFamily,
    /// Coarse plane.
    pub plane: CompanionTransportPlane,
    /// Delivery semantics.
    pub delivery: DeliverySemantics,
    /// Payload rate and shape.
    pub payload_rate: PayloadRateClass,
    /// Role this descriptor plays relative to external authority.
    pub authority_role: ModuleAuthorityRole,
    /// External Manifold or Hostess route ids using this transport.
    pub route_ids: Vec<String>,
    /// Evidence stages expected for this capability.
    pub required_evidence_stages: Vec<String>,
    /// Frontends that can render this descriptor.
    pub supported_frontends: Vec<FrontendKind>,
    /// Short capability strengths.
    pub strengths: Vec<String>,
    /// Short capability costs or limitations.
    pub costs: Vec<String>,
    /// Good use cases for this transport.
    pub suitable_for: Vec<String>,
    /// Use cases this transport should not be used for.
    pub not_suitable_for: Vec<String>,
    /// Public/private sensitivity tags.
    pub sensitivity: Vec<SensitivityTag>,
}

/// External report a frontend may read to render module state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalReportBinding {
    /// Stable report binding id.
    pub id: String,
    /// External report schema id.
    pub schema: String,
    /// Owning lane for the report.
    pub owner_lane: ModuleOwnerLane,
}

/// Command request a frontend may submit through an external authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandRequestBinding {
    /// Stable command binding id.
    pub id: String,
    /// External command id.
    pub command_id: String,
    /// External route id or route descriptor id.
    pub route_id: String,
    /// Owning lane for command authority.
    pub authority_lane: ModuleOwnerLane,
    /// Whether the command can run without an explicit user action.
    pub safe_to_auto_run: bool,
}

/// Evidence artifact a frontend may display or export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceArtifactBinding {
    /// Stable evidence binding id.
    pub id: String,
    /// Evidence schema id.
    pub schema: String,
    /// Owning lane that validates or produces the evidence.
    pub owner_lane: ModuleOwnerLane,
    /// Whether this artifact may require redaction before export.
    pub redaction_required: bool,
}

/// Optional remediation action exposed by a readiness or adapter module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemediationAction {
    /// Stable action id.
    pub id: String,
    /// Human-facing action title.
    pub title: String,
    /// External command id that performs the remediation.
    pub command_id: Option<String>,
    /// Whether the action requires explicit confirmation.
    pub manual_confirmation_required: bool,
}

/// Action policy for a companion module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionPolicy {
    /// Whether read-only checks may run automatically.
    pub auto_run_checks: bool,
    /// Whether state-changing actions require explicit confirmation.
    pub state_changes_require_confirmation: bool,
    /// Whether the module is allowed to run destructive operations.
    pub destructive_actions_allowed: bool,
}

/// Frontend-neutral descriptor for one companion app capability module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionModuleDescriptor {
    /// Schema id for this descriptor.
    pub schema: String,
    /// Stable module id.
    pub module_id: String,
    /// Human-facing module title.
    pub title: String,
    /// Module family.
    pub family: CompanionModuleFamily,
    /// Owning Rusty lane.
    pub owner_lane: ModuleOwnerLane,
    /// Role this descriptor plays relative to external authority.
    pub authority_role: ModuleAuthorityRole,
    /// Frontends that can render this descriptor.
    pub supported_frontends: Vec<FrontendKind>,
    /// Required tools or dependencies.
    pub required_tools: Vec<ToolRequirement>,
    /// Required device states.
    pub required_device_states: Vec<DeviceStateRequirement>,
    /// Required transports.
    pub required_transports: Vec<TransportRequirement>,
    /// Reports the module can read.
    pub readable_reports: Vec<ExternalReportBinding>,
    /// Command requests this module can submit.
    pub command_requests: Vec<CommandRequestBinding>,
    /// Evidence artifacts this module can display.
    pub evidence_artifacts: Vec<EvidenceArtifactBinding>,
    /// Remediation actions exposed by this module.
    pub remediation_actions: Vec<RemediationAction>,
    /// Safety policy for automatic and manual actions.
    pub action_policy: ActionPolicy,
    /// Public/private sensitivity tags.
    pub sensitivity: Vec<SensitivityTag>,
}

/// A module selected by a workspace with optional prominence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceModuleSelection {
    /// Module id referenced by the workspace.
    pub module_id: String,
    /// Whether the workspace requires this module.
    pub required: bool,
    /// Whether this module should be prominent in the workspace UI.
    pub prominent: bool,
}

/// Frontend-neutral workspace composition over companion modules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionWorkspaceDescriptor {
    /// Schema id for this descriptor.
    pub schema: String,
    /// Stable workspace id.
    pub workspace_id: String,
    /// Human-facing workspace title.
    pub title: String,
    /// Frontends that can render this workspace.
    pub supported_frontends: Vec<FrontendKind>,
    /// Selected modules for this workspace.
    pub modules: Vec<WorkspaceModuleSelection>,
    /// Sensitivity tags for the workspace composition.
    pub sensitivity: Vec<SensitivityTag>,
}

impl CompanionModuleDescriptor {
    /// Validate descriptor invariants required by frontends and adapters.
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != COMPANION_MODULE_DESCRIPTOR_SCHEMA {
            return Err(format!(
                "unsupported companion module schema {}",
                self.schema
            ));
        }
        require_non_empty("module_id", &self.module_id)?;
        require_non_empty("title", &self.title)?;
        if self.authority_role == ModuleAuthorityRole::Authority {
            return Err(format!(
                "module {} may not claim command or runtime authority",
                self.module_id
            ));
        }
        if self.supported_frontends.is_empty() {
            return Err(format!(
                "module {} must list at least one supported frontend",
                self.module_id
            ));
        }
        if self.sensitivity.is_empty() {
            return Err(format!(
                "module {} must declare at least one sensitivity tag",
                self.module_id
            ));
        }
        for tool in &self.required_tools {
            require_non_empty("tool requirement id", &tool.id)?;
            require_non_empty("tool requirement name", &tool.name)?;
        }
        for state in &self.required_device_states {
            require_non_empty("device state requirement id", &state.id)?;
            require_non_empty("device state requirement label", &state.label)?;
        }
        for transport in &self.required_transports {
            require_non_empty("transport requirement id", &transport.id)?;
            require_non_empty("transport requirement family", &transport.family)?;
        }
        for report in &self.readable_reports {
            require_non_empty("external report id", &report.id)?;
            require_external_schema("external report schema", &report.schema)?;
        }
        for command in &self.command_requests {
            require_non_empty("command binding id", &command.id)?;
            require_non_empty("command id", &command.command_id)?;
            require_non_empty("route id", &command.route_id)?;
            if command.command_id.starts_with("rusty.gui.") {
                return Err(format!(
                    "module {} command {} uses GUI as command authority",
                    self.module_id, command.command_id
                ));
            }
            if command.authority_lane == ModuleOwnerLane::Gui {
                return Err(format!(
                    "module {} command {} uses GUI lane as command authority",
                    self.module_id, command.command_id
                ));
            }
            if command.safe_to_auto_run
                && self.action_policy.state_changes_require_confirmation
                && !self.action_policy.auto_run_checks
            {
                return Err(format!(
                    "module {} command {} cannot auto-run under manual-only policy",
                    self.module_id, command.command_id
                ));
            }
        }
        for evidence in &self.evidence_artifacts {
            require_non_empty("evidence binding id", &evidence.id)?;
            require_external_schema("evidence schema", &evidence.schema)?;
        }
        for action in &self.remediation_actions {
            require_non_empty("remediation action id", &action.id)?;
            require_non_empty("remediation action title", &action.title)?;
            if let Some(command_id) = &action.command_id {
                require_non_empty("remediation command id", command_id)?;
                if command_id.starts_with("rusty.gui.") {
                    return Err(format!(
                        "module {} remediation {} uses GUI as command authority",
                        self.module_id, action.id
                    ));
                }
            }
        }
        if self.action_policy.destructive_actions_allowed {
            return Err(format!(
                "module {} may not allow destructive actions in a portable GUI descriptor",
                self.module_id
            ));
        }
        Ok(())
    }
}

impl CompanionWorkspaceDescriptor {
    /// Validate workspace descriptor invariants.
    pub fn validate(&self, known_module_ids: &[&str]) -> Result<(), String> {
        if self.schema != COMPANION_WORKSPACE_DESCRIPTOR_SCHEMA {
            return Err(format!(
                "unsupported companion workspace schema {}",
                self.schema
            ));
        }
        require_non_empty("workspace_id", &self.workspace_id)?;
        require_non_empty("title", &self.title)?;
        if self.supported_frontends.is_empty() {
            return Err(format!(
                "workspace {} must list at least one supported frontend",
                self.workspace_id
            ));
        }
        if self.modules.is_empty() {
            return Err(format!(
                "workspace {} must select at least one module",
                self.workspace_id
            ));
        }
        if self.sensitivity.is_empty() {
            return Err(format!(
                "workspace {} must declare at least one sensitivity tag",
                self.workspace_id
            ));
        }
        for module in &self.modules {
            require_non_empty("workspace module id", &module.module_id)?;
            if !known_module_ids.contains(&module.module_id.as_str()) {
                return Err(format!(
                    "workspace {} references unknown module {}",
                    self.workspace_id, module.module_id
                ));
            }
        }
        Ok(())
    }
}

impl CompanionTransportCapabilityDescriptor {
    /// Validate transport capability invariants for frontend route selection.
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != COMPANION_TRANSPORT_CAPABILITY_SCHEMA {
            return Err(format!(
                "unsupported companion transport capability schema {}",
                self.schema
            ));
        }
        require_non_empty("transport_id", &self.transport_id)?;
        require_non_empty("title", &self.title)?;
        if self.authority_role == ModuleAuthorityRole::Authority {
            return Err(format!(
                "transport {} may not claim command or runtime authority",
                self.transport_id
            ));
        }
        if self.route_ids.is_empty() {
            return Err(format!(
                "transport {} must list at least one external route id",
                self.transport_id
            ));
        }
        for route_id in &self.route_ids {
            require_non_empty("route id", route_id)?;
            if route_id.starts_with("rusty.gui.") {
                return Err(format!(
                    "transport {} uses GUI as route authority",
                    self.transport_id
                ));
            }
        }
        if self.required_evidence_stages.is_empty() {
            return Err(format!(
                "transport {} must list required evidence stages",
                self.transport_id
            ));
        }
        for stage in &self.required_evidence_stages {
            require_non_empty("required evidence stage", stage)?;
        }
        if self.supported_frontends.is_empty() {
            return Err(format!(
                "transport {} must list at least one supported frontend",
                self.transport_id
            ));
        }
        if self.sensitivity.is_empty() {
            return Err(format!(
                "transport {} must declare at least one sensitivity tag",
                self.transport_id
            ));
        }
        if self.family == CompanionTransportFamily::MediaDataPlane
            && self.payload_rate == PayloadRateClass::LowRateJson
        {
            return Err(format!(
                "transport {} cannot describe media data-plane payloads as low-rate JSON",
                self.transport_id
            ));
        }
        if self.payload_rate == PayloadRateClass::HighRateBinary
            && self.plane == CompanionTransportPlane::Control
        {
            return Err(format!(
                "transport {} cannot use high-rate binary payloads on the control plane",
                self.transport_id
            ));
        }
        Ok(())
    }
}

fn require_non_empty(field: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{field} must not be empty"));
    }
    Ok(())
}

fn require_external_schema(field: &str, value: &str) -> Result<(), String> {
    require_non_empty(field, value)?;
    if value.starts_with("rusty.gui.") {
        return Err(format!("{field} must be owned by an external lane"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        CompanionModuleDescriptor, CompanionModuleFamily, CompanionTransportCapabilityDescriptor,
        CompanionWorkspaceDescriptor, COMPANION_MODULE_FAMILIES,
    };

    #[test]
    fn companion_taxonomy_contains_expected_module_families() {
        assert!(COMPANION_MODULE_FAMILIES.contains(&CompanionModuleFamily::ReadinessPreconditions));
        assert!(COMPANION_MODULE_FAMILIES.contains(&CompanionModuleFamily::QuestDeviceLink));
        assert!(COMPANION_MODULE_FAMILIES.contains(&CompanionModuleFamily::ManifoldCommandFeedback));
        assert!(COMPANION_MODULE_FAMILIES.contains(&CompanionModuleFamily::StreamTelemetryState));
        assert!(COMPANION_MODULE_FAMILIES.contains(&CompanionModuleFamily::MediaRemoteCameraPlane));
        assert_eq!(COMPANION_MODULE_FAMILIES.len(), 11);
    }

    #[test]
    fn companion_module_fixtures_validate() {
        for fixture in [
            "../../../fixtures/descriptors/companion-module-readiness.json",
            "../../../fixtures/descriptors/companion-module-command.json",
            "../../../fixtures/descriptors/companion-module-stream.json",
            "../../../fixtures/descriptors/companion-module-evidence.json",
            "../../../fixtures/descriptors/companion-module-study-workspace-adapter.json",
        ] {
            let descriptor: CompanionModuleDescriptor =
                serde_json::from_str(include_str_fixture(fixture)).expect("valid JSON");
            descriptor
                .validate()
                .expect("valid companion module descriptor");
        }
    }

    #[test]
    fn companion_workspace_fixture_validates_against_known_modules() {
        let workspace: CompanionWorkspaceDescriptor = serde_json::from_str(include_str!(
            "../../../fixtures/descriptors/companion-workspace-hostess-makepad-validation.json"
        ))
        .expect("valid JSON");
        workspace
            .validate(&[
                "companion.readiness.preconditions",
                "companion.command.bridge_probe",
                "companion.evidence.bridge_route",
            ])
            .expect("valid companion workspace descriptor");
    }

    #[test]
    fn companion_transport_capability_fixtures_validate() {
        for fixture in [
            include_str!("../../../fixtures/descriptors/transport-capability-adb.json"),
            include_str!("../../../fixtures/descriptors/transport-capability-websocket.json"),
            include_str!("../../../fixtures/descriptors/transport-capability-udp-osc.json"),
            include_str!("../../../fixtures/descriptors/transport-capability-lsl.json"),
            include_str!(
                "../../../fixtures/descriptors/transport-capability-app-private-staging.json"
            ),
            include_str!("../../../fixtures/descriptors/transport-capability-media-plane.json"),
        ] {
            let descriptor: CompanionTransportCapabilityDescriptor =
                serde_json::from_str(fixture).expect("valid JSON");
            descriptor
                .validate()
                .expect("valid transport capability descriptor");
        }
    }

    #[test]
    fn damaged_empty_module_id_is_rejected() {
        let descriptor: CompanionModuleDescriptor = serde_json::from_str(include_str!(
            "../../../fixtures/damaged/companion-module-empty-id.json"
        ))
        .expect("valid JSON");
        assert!(descriptor.validate().is_err());
    }

    #[test]
    fn damaged_authority_claim_is_rejected() {
        let descriptor: CompanionModuleDescriptor = serde_json::from_str(include_str!(
            "../../../fixtures/damaged/companion-module-authority-claim.json"
        ))
        .expect("valid JSON");
        let error = descriptor.validate().expect_err("authority claim rejected");
        assert!(error.contains("may not claim"));
    }

    #[test]
    fn damaged_gui_command_authority_is_rejected() {
        let descriptor: CompanionModuleDescriptor = serde_json::from_str(include_str!(
            "../../../fixtures/damaged/companion-module-gui-command-authority.json"
        ))
        .expect("valid JSON");
        let error = descriptor
            .validate()
            .expect_err("GUI command authority rejected");
        assert!(error.contains("uses GUI"));
    }

    #[test]
    fn damaged_workspace_unknown_module_is_rejected() {
        let workspace: CompanionWorkspaceDescriptor = serde_json::from_str(include_str!(
            "../../../fixtures/damaged/companion-workspace-unknown-module.json"
        ))
        .expect("valid JSON");
        let error = workspace
            .validate(&["companion.readiness.preconditions"])
            .expect_err("unknown module rejected");
        assert!(error.contains("unknown module"));
    }

    #[test]
    fn damaged_transport_media_json_payload_is_rejected() {
        let descriptor: CompanionTransportCapabilityDescriptor = serde_json::from_str(
            include_str!("../../../fixtures/damaged/transport-capability-media-low-rate-json.json"),
        )
        .expect("valid JSON");
        let error = descriptor
            .validate()
            .expect_err("media JSON payload rejected");
        assert!(error.contains("media data-plane payloads"));
    }

    fn include_str_fixture(path: &str) -> &'static str {
        match path {
            "../../../fixtures/descriptors/companion-module-readiness.json" => {
                include_str!("../../../fixtures/descriptors/companion-module-readiness.json")
            }
            "../../../fixtures/descriptors/companion-module-command.json" => {
                include_str!("../../../fixtures/descriptors/companion-module-command.json")
            }
            "../../../fixtures/descriptors/companion-module-stream.json" => {
                include_str!("../../../fixtures/descriptors/companion-module-stream.json")
            }
            "../../../fixtures/descriptors/companion-module-evidence.json" => {
                include_str!("../../../fixtures/descriptors/companion-module-evidence.json")
            }
            "../../../fixtures/descriptors/companion-module-study-workspace-adapter.json" => {
                include_str!(
                    "../../../fixtures/descriptors/companion-module-study-workspace-adapter.json"
                )
            }
            _ => unreachable!("fixture path is listed by the test"),
        }
    }
}
