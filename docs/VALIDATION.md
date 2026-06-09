# Rusty GUI Validation

Run:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\tools\check_all.ps1
```

The check runs formatting, tests, and a boundary scan that keeps Makepad,
Quest/OpenXR, and legacy XR naming out of core GUI source.

