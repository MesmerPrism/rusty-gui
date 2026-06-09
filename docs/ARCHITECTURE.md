# Rusty GUI Architecture

Rusty GUI owns renderer-neutral and toolkit-neutral graphical interaction
descriptors.

## Ownership

- panels and inspectors;
- widget descriptors;
- graph canvas descriptors;
- layout and theme hints;
- command binding descriptors.

## Non-Ownership

- command/session authority;
- Makepad widget implementations;
- renderer or platform frame lifecycle;
- Quest/OpenXR settings;
- downstream app workflows.

Adapters consume GUI descriptors and bind them to a toolkit. They do not move
toolkit dependencies into this repo.

