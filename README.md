# ATec Aula Assistant

Aula Assistant is a user-friendly media control system in the Gymnasium Riedberg auditorium.
It lets both trained staff and non-technical users operate the projector, sound system, and lighting,
while also offering advanced controls for technical staff.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Building from Source](#building-from-source)
- [Contributing](#contributing)
- [License](#license)

## Features

<!-- TODO -->

## Installation

<!-- TODO -->

## Building from Source

To build Aula Assistant from source, follow these steps:

### Prerequisites

Make sure you have the following installed on your system:

- [Git](https://git-scm.com/downloads)
- [Rust](https://www.rust-lang.org/tools/install) v1.90.0 or higher
- [Node.js](https://nodejs.org/en/download) v24.4.1 or higher
- [pnpm](https://pnpm.io/installation)
- [Tauri Prerequisites](https://tauri.app/start/prerequisites) for your platform

### Steps

```bash
# clone the repository
git clone https://github.com/grb-technik/aula-assistant.git; cd aula-assistant
# install dependencies
pnpm install
cargo check
# build the project
pnpm tauri build # --no-bundle if you just want the executable
# navigate to the output
cd target/release
```

After building, you'll find:

- The executable (`aula-assistant` or `aula-assistant.exe`) in `target/release`.
- The platform-specific installer in `target/release/bundle` if your platform supports one of the following formats:
    - msi (Windows)
    - deb (Debian-based Linux)
    - other formats as supported by Tauri need to be explicitly enabled by running `pnpm tauri build --bundles <BUNDLES>`

## Contributing

If you'd love to help improving this tool checkout the [contributing guide](CONTRIBUTING.md).

## License

This project is licensed under the [GNU GPL-3.0](LICENSE.txt).
