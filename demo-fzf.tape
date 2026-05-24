Output demo-fzf.gif
Set Shell "zsh"
Set FontSize 14
Set FontFamily "Noto Sans Mono"
Set Width 880
Set Height 520
Set Framerate 30
Set Theme "Catppuccin Mocha"
Set TypingSpeed 30ms
Set Padding 15

Type "sed -i 's/use_fzf: false/use_fzf: true/' $XDG_CONFIG_HOME/proj/config.yaml" Enter
Sleep 200ms

Type "clear" Enter
Sleep 300ms

Type "proj" Enter
Sleep 2.5s
Type "my" Sleep 2.5s
Enter
Sleep 2s

Type "pwd" Enter
Sleep 2s

Type "sed -i 's/use_fzf: true/use_fzf: false/' $XDG_CONFIG_HOME/proj/config.yaml" Enter
Sleep 200ms
