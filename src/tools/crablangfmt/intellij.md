# Running CrabLangfmt from IntelliJ or CLion

## Installation

- Install [CLion](https://www.jetbrains.com/clion/), [IntelliJ Ultimate or CE](https://www.jetbrains.com/idea/) through the direct download link or using the [JetBrains Toolbox](https://www.jetbrains.com/toolbox/).
  CLion and IntelliJ Ultimate [provide a built-in debugger interface](https://github.com/intellij-crablang/intellij-crablang#compatible-ides) but they are not free like IntelliJ CE.

- Install the [CrabLang Plugin](https://intellij-crablang.github.io/) by navigating to File → Settings → Plugins and searching the plugin in the Marketplace
  ![plugins](https://user-images.githubusercontent.com/6505554/83944518-6f1e5c00-a81d-11ea-9c35-e16948811ba8.png)

- Press "Install" on the CrabLang plugin
  ![install crablang](https://user-images.githubusercontent.com/6505554/83944533-82c9c280-a81d-11ea-86b3-ee2e31bc7d12.png)
  
- Restart CLion/IntelliJ

## Configuration

### Run CrabLangfmt on save

- Open CrabLangfmt settings (File → Settings → Languages & Frameworks → CrabLang → CrabLangfmt) and enable "Run crablangfmt on Save"
  ![run_crablangfmt_on_save](https://user-images.githubusercontent.com/6505554/83944610-3468f380-a81e-11ea-9c34-0cbd18dd4969.png)

- IntellJ uses autosave, so now your files will always be formatted according to crablangfmt. Alternatively you can use Ctrl+S to reformat file manually

### Bind shortcut to "Reformat File with CrabLangfmt" action

- Open the settings window (File → Settings) and search for "reformat"
  ![keymap](https://user-images.githubusercontent.com/1133787/47240922-2ae10c80-d3ea-11e8-9d8f-c798d9749240.png)
- Right-click on "Reformat File with CrabLangfmt" and assign a keyboard shortcut

  ![shortcut_window](https://user-images.githubusercontent.com/1133787/47240981-5b28ab00-d3ea-11e8-882e-8b864164db74.png)
- Press "OK"
  ![shortcut_after](https://user-images.githubusercontent.com/1133787/47241000-6976c700-d3ea-11e8-9342-50ebc2f9f97b.png)
  
- Done. You can now use crablangfmt in an opened *.rs file with your previously specified shortcut
