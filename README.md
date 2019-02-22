# Meshlight

This is an artistic demonstration of mesh routing, which is often used in sensor arrays - a staple of applied IoT technology.

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain
- `cargo`
- `stm321fxx-hal`
- `cortex_m`
- `cortex_m_rt`
- `openocd`

## Hardware

- `n` Black Pill boards (it's a mesh network, add as many as you like!)
- `n * 2` individually addressable LED strips
- `n` toggle buttons
- `n * 3` wires, to connect each board

To get the Rust environment up and running, please refer to [this blog post by Tim](https://cs.anu.edu.au/courses/china-study-tour/news/2019/11/01/Tim-Rust-Discoboard-guide/)

## Further Reading

- [STM32 HAL library](https://docs.rs/stm32f1xx-hal/0.2.0/stm32f1xx_hal/)
- [Realtime for the masses (Cortex M)](https://github.com/japaric/cortex-m-rtfm) - Embedded concurrency framework
- [Embedded Rust eBook](https://rust-embedded.github.io/book/)

# License

MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution

This repository will be sunsetted after the 22 Feb, 2019. If you're looking to develop this further, I recommend either forking (or cloning) the repo, and working on it seperately.

## Contact

Primary maintainers:

- [Timothy Lee](github.com/tztimlee)
- [Harrison Turton](github.com/tztimlee)

If you're looking at getting in contact, please email us! (On our profiles)
