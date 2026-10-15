### Added

- **motion:** `motion::physics`, our own spring solver (closed form, frame-rate independent, keeps its speed when retargeted), momentum glides that bounce off bounds or spring to the nearest snap point, a 100 ms pointer velocity tracker, iOS-style rubber banding and zoom maths. Plain Rust, unit tested (experimental).
- **motion:** `use_motion_value` / `MotionValue`: a value driven by that physics; set it while dragging, then `spring_to` or `glide` at the pointer's speed (experimental).
- **motion:** `Draggable`: drag and throw with momentum, rubber-band bounds, snap points, snap back to origin, and arrow keys / Home from the keyboard (experimental).
- **motion:** `PinchZoom`: two-finger pinch and pan with momentum, Ctrl + wheel (trackpad pinch), double click, and `+`, `-`, `0` and arrows from the keyboard; springs back inside its limits and announces the zoom level (experimental).
- **motion:** `LayoutGroup` / `LayoutItem`: position and size changes animate, and items sharing a `layout_id` morph into each other (shared-element transitions such as a sliding tab highlight) (experimental).
- **gallery:** "Drag with momentum", "Snap points", "Pinch to zoom" and "Shared layout" demos; `tests/e2e/motion.mjs` checks them, and reduced motion, on Chromium.
