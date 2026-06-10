# psychotic

# Requirements

It is required install `sdl2` and `sdl2_image` packages and to have Rust installed.

# Installation on Arch Linux

```
sudo pacman -S sdl2 sdl2_image
```

# Installation on MacOS

```
brew install sdl2 sdl2_image
```

# Building and running

In order to run the game it is recommended to run it in release mode.

```
cargo run --release
```

Otherwise there will be heavy lag.

# Documentation

To generate documentation run the following command:

```
cargo doc
```
