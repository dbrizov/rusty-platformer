# rusty-platformer
A hobby project with the sole purpose of learning the [Rust Programming Language](https://rust-lang.org/).

# Requirements
## Rust
The code is written using `rustc 1.92.0`, but any version above that should work.

## SDL2.0 development libraries
`SDL2 >= 2.0.26` is recommended to use the `rust-sdl2` bindings

### Windows
Install via `vcpkg`.
1. Clone the `vcpkg` git repository.
```
git clone https://github.com/microsoft/vcpkg.git vcpkg
```
2. Run the `vcpkg` boostrap script.
```
cd vcpkg
.\bootstrap-vcpkg.bat
```
3. Set the `VCPKG_ROOT` environment variable and add it to `PATH`.
```
$env:VCPKG_ROOT = "C:\path\to\vcpkg"
$env:PATH = "$env:VCPKG_ROOT;$env:PATH"
```
> [!NOTE]
> Setting environment variables in this manner only affects the current terminal session. To make these changes permanent across all sessions, set them through the Windows System Environment Variables panel.
4. Restart your computer so that `VCPKG_ROOT` is recognized.
5. Open a new terminal to verify.
```
vcpkg --version
```
6. Run the `vcpkg_setup.bat` that is located in the project's root directory.
```
.\vcpkg_setup.bat
```

### GNU/Linux
Install via `apt`.
```
sudo apt install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev
```

### macOS
Install via [homebrew](https://brew.sh/).
```
brew install sdl2 sdl2_image sdl2_ttf
```
Add this line to your `~/.zprofile` or `~/.bashrc` depending on whether you use `zsh` or `bash`.
```
export LIBRARY_PATH="$LIBRARY_PATH:$HOMEBREW_PREFIX/lib"
```
