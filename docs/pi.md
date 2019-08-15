# Raspberry Pi 4 Installation

_Provides instructions for installing Lighthouse on a Raspberry Pi 4. May be
applicable to other models._

## Perquisites

Install [Raspbian](https://www.raspberrypi.org/downloads/raspbian/). The
simplest way is by [downloading a Rasbian
image](https://www.raspberrypi.org/downloads/raspbian) then following the
[installation
guide][https://www.raspberrypi.org/documentation/installation/installing-images/README.md].
We used "Rasbian Buster Lite" because we're comfortable without a GUI and it's
less resource hungry.

This guide uses the terminal.

## Steps

1. Install dependencies with `$ sudo apt-get install git git-lfs clang cmake`
1. Install Rust via [rustup](https://rustup.rs/).
1. Configure the current shell after installing Rust `$ source $HOME/.cargo/env`.
1. Clone the Lighthouse repository with `$ git clone https://github.com/sigp/lighthouse.git`
1. Move into the newly create repository `$ cd lighthouse`.
1. Initialize git submodules (important):
  - `$ git submodules init`
  - `$ git submodules update`
1. Build the project `$ cargo build --all --release`
