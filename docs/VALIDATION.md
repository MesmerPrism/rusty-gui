# Rusty GUI Validation

Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

The check runs formatting, tests, and a boundary scan that keeps Makepad,
Quest/OpenXR, and legacy XR naming out of core GUI source.

The Rust tests parse and validate both the basic panel descriptor fixture and
the companion module/workspace fixtures, including damaged cases that reject
empty ids, GUI-owned command authority, UI authority claims, and unknown
workspace module references.
They also prove that the Hostess Makepad validation workspace does not claim a
frontend unless every selected module and required transport descriptor supports
that frontend, and that every module's advertised frontends are closed over its
required transport descriptors.

Transport capability fixture tests also validate ADB, WebSocket, UDP/OSC, LSL,
app-private staging, and media data-plane descriptors. The damaged media
fixture proves that high-rate frame payloads cannot be described as low-rate
JSON command/control traffic.
