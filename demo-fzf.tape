Output demo-fzf.gif
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
Type "sed -i s/use_fzf: false/use_fzf: true/ $XDG_CONFIG_HOME/proj/config.yaml" Enter
Sleep 500ms
Show

Type "clear" Enter
Sleep 300ms

Type "proj" Enter
Sleep 2s
Type "my" Sleep 2s
Enter
Sleep 2s

Type "pwd" Enter
Sleep 2s

Hide
Type "sed -i s/use_fzf: true/use_fzf: false/ $XDG_CONFIG_HOME/proj/config.yaml" Enter
