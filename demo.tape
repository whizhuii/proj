Output demo.gif
Set FontSize 16
Set FontFamily "Noto Sans Mono"
Set Width 880
Set Height 520
Set Framerate 30
Set Theme "Catppuccin Mocha"
Set Shell "bash"
Set TypingSpeed 30ms

# ---- setup ----
Type "export XDG_CONFIG_HOME=/tmp/proj-demo-config" Enter
Sleep 100ms
Type "proj-core shell func > /tmp/proj-func.sh" Enter
Sleep 100ms
Type "source /tmp/proj-func.sh" Enter
Sleep 200ms

# ============================================
# Part 1 — Tree view (pass style)
# ============================================

Type "proj" Enter
Sleep 3s

Type "proj list stable" Enter
Sleep 2.5s

Type "proj find api" Enter
Sleep 2.5s

# ============================================
# Part 2 — Fzf filter demo
# ============================================

Type "proj-core list --flat" Enter
Sleep 2s

Type "proj-core list --flat | fzf --filter=bl" Enter
Sleep 2s

Type "proj-core list --flat | fzf --filter=tool" Enter
Sleep 2s
