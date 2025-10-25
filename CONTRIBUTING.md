# Contributing to Aula Assistant

Thanks for your interest in contributing! Your help is greatly appreciated.

## Prerequisites

Ensure you have the latest version of [Rust](https://www.rust-lang.org/tools/install) and [Node](https://nodejs.org/en/download) installed.
[Git](https://git-scm.com/downloads) and [pnpm]https://pnpm.io/installation) are needed as well.

## How to Contribute

1. [Fork](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/fork-a-repo) this repository.
2. [Clone] (https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository) it to your local machine.
3. Have fun coding! Make your changes and commit them with clear messages.
4. Ensure your code passes all tests and adheres to the project's coding standards.
5. Create a [pull request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request-from-a-fork) to [this](https://github.com/grb-technik/aula-assistant) repository. Describe your changes clearly and reference any relevant issues or discussions.

## Additional Notes

### Target Hardware and Environment

This software is primarily designed to run on a Raspberry Pi equipped with a 7" capacitive touch display (resolution: 800×480) in tablet mode. The default window size is set accordingly to reflect the constraints of the Raspberry Pi display.
However, the interface shall be responsive and should work well on laptops or other screen sizes too.

Because the target environment relies on touch input (often less precise than a mouse) all ui elements must be designed to function without the need for a keyboard or an external mouse.

### Network

The application communicates with other devices exclusively over the local network.
It is designed to be a passive network participant, meaning:

- It only initiates outgoing connections and must not accept any incoming connections from other devices.
- It must not block or interfere with any network traffic, including connections initiated by other devices on the same network.

This design allows third-party devices to continue managing or interacting with other components on the network—even if this software becomes unresponsive or fails.

### Hardware Details

For reference, here is a complete list of all hardware components used as part of the interface or as part of the target environment:

- [Raspberry Pi 4 Model B (8GB RAM)](https://www.berrybase.de/raspberry-pi-4-computer-modell-b-8gb-ram)
- [Raspberry Pi 7" capacitive touch display](https://www.berrybase.de/offizielles-raspberry-pi-7-display-mit-kapazitiven-touchscreen)
- [PureLink PT-MA-HDBT42 4x2 4K 18Gbps HDMI HDBaseT Matrix with Scaler](https://www.purelink.de/en/switcher-matrices/hdbaset/3433/4x2-4k-18gbps-hdmi-hdbaset-matrix-with-scaler)
- Panasonic PT-EX12K
- ENTTEC ODE Mk2 Ethernet to DMX converter
