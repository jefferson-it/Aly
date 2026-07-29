# Cocoa GUI Backend in Aly

The Cocoa backend maps Aly widgets directly to AppKit classes (`NSWindow`, `NSButton`, etc.) on macOS systems.

---

## 1. Activation

To force native Apple AppKit rendering:

```aly
gui.useBackend("cocoa")
```

---

## 2. Platform Compliance

By utilizing standard AppKit runtime selectors, Cocoa windows automatically comply with macOS layout conventions, Apple Menu bars, and system font renderings.
