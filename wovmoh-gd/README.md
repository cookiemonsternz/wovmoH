<p align="center">
    <img width="300" src="extras/images/logo.png" alt="GodotX Toast - Logo">
</p>

# GodotX Toast

A professional toast notification system for Godot 4.5+ with style-driven architecture.

## Features

- **One-line API**: Simple, intuitive global access
- **Style-driven**: Everything customizable via GodotxToastStyle resources
- **7 Built-in styles**: Success, Warning, Info, Error, Light, Dark, Loading
- **9 Spawn origins**: Center, Top, Bottom, Left, Right, and corners
- **3 Animation types**: Slide, Fade, None
- **Smart stacking**: Per-origin stacking with configurable strategies
- **Deck Stack mode**: Card-deck style stacking with swipe to dismiss
- **Swipe gestures**: Swipe horizontal/vertical/both to dismiss
- **Loading Toasts**: Spinners and progress bars, stays until dismissed
- **Flexible content**: Support for null, Texture2D, or PackedScene
- **Responsive**: Auto-resizes with viewport changes
- **UI Scale aware**: Respects Godot's content scale factor

## Installation

1. Copy the `addons/godotx_toast` folder into your Godot project
2. Enable the plugin in **Project Settings → Plugins**
3. The plugin automatically registers `GodotxToast` as an autoload singleton

## Basic Usage

```gdscript
# Simple one-liners
GodotxToast.show("Hello World!")
GodotxToast.success("Operation completed!")
GodotxToast.error("Something went wrong!")
GodotxToast.warning("Please check input")
GodotxToast.info("Did you know?")

# Loading toast
var id = GodotxToast.loading("Processing...")
GodotxToast.update_loading(id, 50.0)
GodotxToast.complete_loading(id, true, "Done!")

# Deck stack mode
var deck_style = GodotxToastStyle.new()
deck_style.stack_strategy = GodotxToastEnums.StackStrategy.DECK
deck_style.swipe_enabled = true
GodotxToast.show("Swipe me!", deck_style, 0)
```

## Demo

Open `scenes/demo/godotx_toast_demo.tscn` to see all features in action.

## API Reference

### Main Methods

```gdscript
# Show a toast
GodotxToast.show(message: String, style = null, time = null, origin = -1, animation = -1) -> String

# Loading toasts
GodotxToast.show_loading(message: String, style = null, origin = -1, animation = -1) -> String
GodotxToast.update_loading(id: String, progress: float, new_message = "") -> bool
GodotxToast.complete_loading(id: String, success: bool = true, final_message = "") -> bool

# Quick helpers
GodotxToast.success(message: String, time = null) -> String
GodotxToast.error(message: String, time = null) -> String
GodotxToast.warning(message: String, time = null) -> String
GodotxToast.info(message: String, time = null) -> String
GodotxToast.loading(message: String, style = null) -> String

# Control
GodotxToast.dismiss(toast_id: String) -> bool
GodotxToast.clear_all()
GodotxToast.clear_by_origin(origin: GodotxToastEnums.ToastOrigin)

# Configuration
GodotxToast.proportion = 30  # Width as % of viewport
GodotxToast.default_origin = GodotxToastEnums.ToastOrigin.BOTTOM
GodotxToast.default_animation = GodotxToastEnums.ToastAnimation.SLIDE
GodotxToast.default_time = GodotxToastEnums.ToastTime.MEDIUM
```

### Signals

```gdscript
GodotxToast.toast_shown.connect(func(id): ...)
GodotxToast.toast_dismissed.connect(func(id, reason): ...)
GodotxToast.toast_clicked.connect(func(id): ...)
GodotxToast.loading_progress_updated.connect(func(id, progress): ...)
GodotxToast.loading_completed.connect(func(id, success): ...)
```

## Custom Styles

```gdscript
var style = GodotxToastStyle.new()
style.background_color = Color.PURPLE
style.font_color = Color.WHITE
style.spinner_type = GodotxToastEnums.SpinnerType.DEFAULT
style.swipe_enabled = true
style.stack_strategy = GodotxToastEnums.StackStrategy.DECK

GodotxToast.show("Custom!", style)
```

## Screenshot

<img width="300" src="extras/images/screenshot1.png" alt="Screenshot 1">
<img width="300" src="extras/images/screenshot2.png" alt="Screenshot 2">

## License

This project is licensed under the [MIT License](LICENSE).

---

Made with ❤️ by Paulo Coutinho
