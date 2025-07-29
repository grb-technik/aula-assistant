# ATec Aula Assistant

Aula Assistant is a user-friendly media control system in the Gymnasium Riedberg auditorium.
It lets both trained staff and non-technical users operate the projector, sound system, and lighting,
while also offering advanced controls for technical staff.

## Table of Contents

- [Build from Source](#build-from-source)
    - [Prequisites](#prequisites)
- [Contributing](#contributing)
- [License](#license)

## Build from Source

### Prequisites

- [git](https://git-scm.com/downloads)
- [nodejs](https://nodejs.org/en/download) v24.4.1
- [pnpm](https://pnpm.io/installation) v10.13.1
- [rust](https://www.rust-lang.org/tools/install) v1.87.0

### Steps

> [!NOTE]
> By default, the `pnpm tauri build` command will create a binary in release mode and the following installers:
> - msi (Windows)
> - deb (Linux)

```bash
git clone https://github.com/grb-technik/aula_assistant.git
cd aula_assistant
pnpm install
pnpm tauri build
# binary and installers can be found at `src-tauri/target/release`
```

## Contributing

If you'd love to help improving this tool checkout the [contributing guide](CONTRIBUTING.md).

## License

This project is licensed under the [GNU GPL-3.0](LICENSE.txt).
