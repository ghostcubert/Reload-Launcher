# Reload Launcher

![Imgur](https://i.imgur.com/S6aU55L.jpeg)

Reload Launcher is a Fortnite Launcher written in [Tauri](https://en.wikipedia.org/wiki/Tauri_(software_framework))

Created by [Ghostcubert](https://github.com/Ghostcubert) to work with [Reload](https://github.com/Project-Reload/Reload-Backend) or [Better Reload](https://github.com/31Benzi/Better-Reload), backend credits go to [31Benzi](https://github.com/31Benzi), [Burlone](https://github.com/burlone0), and [Lawin](https://github.com/Lawin0129) 

## Features
* Login:
    * [ ] Email and Password.
    * [ ] Customizable Background.
* Dashboard:
    * [ ] News.
    * [ ] Launch.
    * [ ] Online/Offline
* Library:
    * [ ] Add builds.
    * [ ] How many builds can be imported.
    * [ ] What specific version can be imported.
    * [ ] **Maybe** downloadable version.
* Settings:
    * [ ] Customizable Settings.
* Discord:
    * [ ] Discord RPC.
* Builds:
    * [ ] XX - XX is supported.

## How to use Reload Launcher
1) Install [Rust](https://rust-lang.org/tools/install/), [C++ Build Tools](https://visualstudio.microsoft.com/downloads/?q=build+tools), and [Bun](https://bun.com/docs/installation).
2) **Download** and **Extract** Reload Launcher to a safe location.
3) Run **"launcher_install.bat"** to install all the required modules.
4) Go to **src-tauri/src/utils/config.rs** in the directory you extracted Reload Launcher into.
5) Open it, set your backend ip, bot client id, and season number.
6) Run **"launcher_test.bat"**, to test things.
7) Run **"launcher_build.bat"** if everything looks ready for release.
8) The release launcher will be in **src-tauri/target/release/bundle/msi/*.msi** or **src-tauri/target/release/bundle/nsis/*.exe**

## License
This **launcher** is licensed under the **BSD 2-Clause License.**