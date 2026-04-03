# TUI Implementation

This document defines how to implement the SMP Terminal User Interface (TUI).

---

## 1. Implementation Goals

- Fast development
- Stable terminal behavior
- Clean separation from core logic
- Easy debugging

---

## 2. Language Choice

Recommended:

- Go → primary
- Rust → optional alternative

---

## 3. Library Selection

---

### Go (Recommended)

Use:

- tview (UI components)
- tcell (terminal backend)

### Rust (Alternative)

Use:

- ratatui (UI framework)
- crossterm (terminal handling)

---

## 4. Architecture

```text
TUI Layer
   ↓
Controller Layer
   ↓
Client Core (State + Actions)
```

### Layers

| **Layer**  | **Responsibility** |
| ---------- | ------------------ |
| TUI        | Rendering + input  |
| Controller | Maps UI → actions  |
| Core       | Business logic     |

---

## 5. Main Loop

```go
for {
    event := ReadInput()
    HandleInput(event)
    UpdateUI()
}
```

Behavior:

- Event-driven rendering
- No blocking operations

---

## 6. Screen Management

Use a screen manager:

```go
type Screen interface {
    Render()
    HandleInput(key KeyEvent)
}
```

### Screens

- InboxScreen
- MessageScreen
- RequestScreen
- ProfileScreen

---

## 7. Input Handling

Capture keys:

```go
switch key {
case 'i':
    showInbox()
case 'r':
    showRequests()
case 'n':
    showNewMessage()
}
```

Requirements:

- Non-blocking input
- Immediate feedback

---

## 8. Rendering Strategy

Option 1 (Recommended):

- Component-based rendering (tview)
- Lists
- Text views
- Input fields

---

## 9. State Binding

Controller interacts with:

- StateProvider
- ActionHandler

Example:

```go
func SendMessageUI() {
    err := actionHandler.SendMessage(to, msg)
    if err != nil {
        showError(err)
    }
}
```

---

## 10. Event Subscription

TUI listens to events:

```go
func OnEvent(e Event) {
    switch e.Type {
    case NewMessage:
        refreshInbox()
    }
}
```

---

## 11. Concurrency Model

Use goroutines:

- Network operations
- Event listening

Rule:

- UI thread must remain responsive

---

## 12. Error Display

Errors shown in:

- Status bar
- Popup
- Inline messages

---

## 13. Debug Mode

Optional flag:

```text
--debug
```

Enables:

- Session logs
- Event logs
- Internal state view

---

## 14. Project Integration

Add under:

```text
cmd/client/
```

Structure:

```text
cmd/client/
├── main.go
├── tui/
│   ├── screens/
│   ├── controller/
│   └── components/
```

---

## 15. Testing

Test:

- Navigation
- Input handling
- State updates

### Manual Testing Required:

- Message send/receive
- Request handling
- Trust display

---

## 16. Performance Considerations

- Avoid full redraws
- Use diff-based updates if possible

---

## 17. Security Considerations

- Do not log sensitive data
- Mask identity details where needed
- Clearly show trust warnings

---

## 18. Summary

The TUI implementation:

- Uses established libraries
- Follows modular architecture
- Integrates cleanly with client core

It provides a practical interface for interacting with SMP.

---
