Output demo-cmd.gif
Set Shell "bash"
Set FontSize 14
Set FontFamily "Noto Sans Mono"
Set Width 880
Set Height 520
Set Framerate 30
Set Theme "Catppuccin Mocha"
Set TypingSpeed 30ms
Set Padding 15

Hide
Type "export XDG_CONFIG_HOME=/tmp/proj-demo-config" Enter
Type "proj-core shell func > /tmp/proj-func.sh" Enter
Type "source /tmp/proj-func.sh" Enter
Sleep 500ms
Show

Type "clear" Enter
Sleep 300ms
Type "proj sync" Enter
Sleep 2s

Type "clear" Enter
Sleep 300ms
Type "proj go mycli" Enter
Sleep 2s

Type "clear" Enter
Sleep 300ms
Type "proj mv mycli develop" Enter
Sleep 2s

Type "clear" Enter
Sleep 300ms
Type "proj config --project-dir" Enter
Sleep 2s
