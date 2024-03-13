# Windows terminal wallpaper tool
Tool for setting background image properties and terminal opacity for Windows Terminal.
Tested mostly on Preview version of Windows terminal.  
WARNING: the tool currently works only for one standard of the configuration JSON. It updates 'profiles' -> 'default' clause.

## Features:
- changing the background image
- changing the properties of the background image:
	- opacity
	- alignment
	- stretch mode
- changing opacity of the terminal
- choosing which terminal version should be updated (if none chosen, it will try to update the first settings file it sees):
	- stable
	- preview
	- unpackaged

Read more about those properties in Microsoft docs - https://learn.microsoft.com/en-us/windows/terminal/customize-settings/profile-appearance#background-images-and-icons

## Possible todos
- [ ] better error handling
- [ ] change message level flag into flags with practial names and make them exclusionary to each other
- [ ] feature: choose image at random from the list or a folder specified
- [ ] feature: for flag without argument - show the respective property current value
- [ ] feature: saving and loading properties of an image (opacity, stretch mode, alignment) used in the past in order to remove the need for repeated property adjustment
- [ ] make the tool handle any Windows Terminal config standard
- [ ] more test coverage
