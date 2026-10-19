# Mobile and Touchscreen Guide

FilmCraft includes dedicated touchscreen compatibility, responsive mobile workspaces, and high-performance optimizations for smartphones, tablets, and touch-enabled desktops.

## 1. Touchscreen Gestures

FilmCraft supports native multi-touch gestures across key surfaces:

### Timeline Gestures
- **Pinch to Zoom**: Two-finger pinch in or out anywhere on the timeline smoothly scales the zoom level (`target_pps`) centered around the gesture focal point. Thumbnail layout freezing during active multi-touch avoids CPU churn.
- **Two-Finger Pan / Scroll**: Two-finger dragging horizontally scrolls the timeline across time, while dragging vertically scrolls video and audio track stacks.
- **Single-Finger Playhead Scrubbing**: Tapping or dragging directly on the time ruler or playhead handle immediately positions the playback head with zero input lag.
- **Finger-Friendly Trimming**: In touch mode or on mobile displays, clip cut edge hitboxes automatically expand from 7.0 px (mouse precision) to 20.0 px (`TOUCH_EDGE_PX`). This allows fingers to easily grab cut points to initiate Ripple, Roll, and standard trims.

### Program & Source Monitor Gestures
- **Pinch to Zoom**: Two-finger pinch zooms the video preview between Fit (10%) and 1600%, centered at the finger midpoint.
- **Two-Finger Pan**: Two-finger drag smoothly pans across a magnified video frame.
- **Double Tap to Fit / 100%**: Double-tapping the monitor toggles between full frame (`Fit`) and pixel-accurate 100% magnification.

---

## 2. Mobile Workspace & Layout

When the viewport width is below 800 pt (or when the **Mobile** workspace is selected):

1. **Top Monitor View**:
   - The Program Monitor automatically docks at the top, maintaining exact sequence aspect ratios with letterboxing.
2. **Touch Control & Jog Bar**:
   - High-contrast SMPTE timecode readout (`00:00:00:00`).
   - Quick action touch buttons (44+ pt touch targets):
     - **Split / Add Edit** (`sequence.addEdit`): Instantly cuts clips at the playhead.
     - **Step Backward / Forward** (`<|`, `|>`): Single-frame stepping.
     - **Central Play/Pause Toggle**: Prominent toggle button.
     - **Ripple Delete** (`edit.rippleDelete`): Deletes the selected clip and ripples remaining clips forward.
     - **Mark In / Out**: Sets sequence edit range.
   - **Jog Shuttle Scrubber**: Tactile horizontal scrub ribbon with graduated tick marks and a center needle. Dragging with a thumb scrubs the playhead with micro-frame precision.
3. **Tabbed Lower Workspace**:
   - A full-width lower pane displays the active mobile tab:
     - **Timeline**: Finger-friendly tracks with prominent clip borders and handles.
     - **Media**: Project bin and media browser with thumbnail cards.
     - **Inspector**: Clip motion (position, scale, rotation), opacity, and volume controls.
     - **Effects**: Video/audio transitions and creative filters.
     - **Export**: Quick export presets and render settings.
4. **Bottom Navigation Bar**:
   - Thumb-friendly persistent bottom navigation tabs (`Timeline`, `Media`, `Inspector`, `Effects`, `Export`).
5. **Mobile Header**:
   - Hamburger drawer button for the full command menu (File, Edit, Sequence, View, Window, Help).
   - Project name with dirty indicator (`- Edited`).
   - Dedicated **Undo** and **Redo** touch buttons.
   - **Touch Mode** toggle button.
   - Quick Export share button.

---

## 3. High Performance & Adaptive Quality

Editing 4K and 1080p video on mobile devices requires careful thermal and battery conservation:

1. **Adaptive Scrubbing Resolution**:
   - During continuous multi-touch pinches, timeline drags, and playback on mobile viewports, the preview pipeline dynamically downscales playback resolution (1/2 or 1/4) to guarantee uninterrupted 60 FPS interaction.
2. **Thumbnail Layout Pacing**:
   - During multi-touch gestures, tile generation uses frozen layout geometry (`thumb_pps`), preventing hundreds of redundant frame decode jobs while zooming.
3. **Lazy Repaint Cycle**:
   - While playback is stopped and no gestures are active, egui repaints only on input events or timers rather than burning continuous GPU cycles.

---

## 4. Mobile Web & PWA Deployment

FilmCraft runs in mobile web browsers (iOS Safari, Android Chrome):
- **Viewport meta tag**: `width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no, viewport-fit=cover` ensures edge-to-edge canvas rendering without letterboxing around the notch or home indicator.
- **CSS Touch Action**: `touch-action: none` on both body and canvas disables browser interference (such as pull-to-refresh or native page pinching).
- **WebAudio AudioContext**: WebAudio resumes automatically on the first user touch gesture (`pointerdown`).
- **PWA Installation**: Adding FilmCraft to the home screen launches in standalone fullscreen mode (`apple-mobile-web-app-capable`).
