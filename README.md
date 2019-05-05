# rmoji

A simple Rust program to parse Unicode Emoji data and dump them to the terminal,
currently in development.

## Features

This is in very early stages of development so most of these features are not
implemented yet.

`[x]` marks a (reasonably) complete feature.

### Modules

- [x] Main program: output a list of known emojis with their names and groups
  for integration with other tools
- [x] Parse module: parse `emoji-test.txt`
- [ ] Cache module: cache parsed data so future startups are faster
- [ ] Update module: download and update `emoji-test.txt` so future emoji
  updates can be performed independently of software updates
- [ ] Picker module: interactively browse and select an emoji, filtering by
  group, subgroup or title
- [ ] Preferences module: select preferred skin color and sort by most used
  emojis

### Integrations

- [ ] fzf integration: use ye olde fzf to easily select an emoji you want to
  input
- [ ] Readline integration (?): Bind a key to input an emoji on the command line
  (similar to macOS Cmd-Ctrl-Space)
- [ ] Vim integration: Use a command to pick an emoji and input it to the editor
