# lbsaver
A small application to automatically install beat saber maps from beatsaver on linux. Supports one-click map installation.

### Prerequisites
Install rustup from https://rustup.rs

On Debian: Install libssl-dev using `sudo apt install libssl-dev`

### Installation
Run the installation script using `./install.sh`

This will automatically install lbsaver and set up one click install for you.

### Configuration

The default path to the beat saber installation is `~/.steam/steam/steamapps/common/Beat Saber/`.

If you installed it elsewhere you can change it with `lbsaver --set-path /path/to/Beat Saber`

### Usage
You can use one-click install or manually run it in the commandline with `lbsaver [uri]` e.g `lbsaver beatsaver://dce`
