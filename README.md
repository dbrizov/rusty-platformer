# rusty-platformer
A hobby project with the sole purpose of learning the [Rust Programming Language](https://rust-lang.org/).

# Requirements
## Rust
The code is written using `rustc 1.92.0`, but any version above that should work.

## SDL2.0 development libraries
`SDL2 >= 2.0.26` is recommended to use the `rust-sdl2` bindings
### GNU/Linux
Install via `apt`.
```
sudo apt install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev libsdl2-mixer-dev
```
### macOS
Install via [homebrew](https://brew.sh/).
```
brew install sdl2 sdl2_image sdl2_ttf sdl2_mixer
```
Add this line to your `~/.zprofile` or `~/.bashrc` depending on whether you use `zsh` or `bash`.
```
export LIBRARY_PATH="$LIBRARY_PATH:$HOMEBREW_PREFIX/lib"
```
### Windows
The easiest way is to download the SDL2 libraries and dynamically link them.<br>
You will also need to install `cmake`. You can install it via `windows-build-tools` or `Visual Studio Installer`.

Here are links to the SDL2 libraries used in the project.
- [SDL2-devel-2.32.10-VC](https://github.com/libsdl-org/SDL/releases/tag/release-2.32.10)
- [SDL2_image-devel-2.8.8-VC](https://github.com/libsdl-org/SDL_image/releases/tag/release-2.8.8)
- [SDL2_ttf-devel-2.24.0-VC](https://github.com/libsdl-org/SDL_ttf/releases/tag/release-2.24.0)
- [SDL2_mixer-devel-2.8.1-VC](https://github.com/libsdl-org/SDL_mixer/releases/tag/release-2.8.1)

Extract the zip files in a folder of your choosing. Depending on your OS you need to use the `x86` or `x64` libraries.
```
C:\SDL2\
  include\*.h
  lib\SDL2.lib
  lib\SDL2_image.lib
  lib\SDL2_ttf.lib
  lib\SDL2_mixer.lib
  bin\SDL2.dll
  bin\SDL2_image.dll
  bin\SDL2_ttf.dll
  bin\SDL2_mixer.dll

```
Add these values to the environment variables.
```
INCLUDE: C:\SDL2\include
LIB: C:\SDL2\lib
PATH: C:\SDL2\bin
```
Restart your PC.

