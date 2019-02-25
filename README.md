# Meshlight

This item – our *artefact* – is an interactive gallery piece. It features the behaviour of the famous distance-vector routing algorithm in an approachable, interesting way.

**Why?**

The average person doesn’t understand the structures that enable our modern internet.

Distance-Vector routing is a foundational algorithm, one which laid the foundation for the modern internet. Upon this, we have modern protocols that support everything from LAN parties to huge, sweeping sensor arrays.

Of course, the words “distance vector routing” don’t spark interest and excitement. It is not easy to bridge the gap between layman & practitioner – that's what this gallery piece tries to do.

The blinking LEDs and interactive "nodes" let the viewer see, in realtime, how the network topology changes under node churn (coming online, going offlien). This is really interesting, and looks *really cool*, but it needs to be approachable for general audiences.

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain
- `cargo`
- `stm321fxx-hal`
- `cortex_m`
- `cortex_m_rt`
- `openocd`

## Hardware

Since this is a mesh network, it can have any number of nodes! That's basically the point - it is resistant to changing network topologies. I'll list out the exact products we used, but this can easily be changed to suit larger (or smaller!) displays.

- `9x` STM32F103 Black Pill Microcontrollers
- `1x` 20M strip of Adafruit neopixel LED strips (using the WS2811 protocol)
- `1x` 12v power supply
- `1x` 3v power supply *OR* `1x` powered USB hub (with `9x` USB -> MicroUSB cables)
- Wires (a lot)
- Soldering equipment

The amount of LED strips will vary depending on the number of black pill boards you want to use and how far apart you have them. You are also free to use as many boards as you want!

### Setup Instructions

Each board makes use of all three usarts, enabling you to connect each board to up to three other ones and daisy chaining them together in any configuration you want.

To connect a board to another, connect one of the usarts of a board to the usart of another, then lay out the LED strip from each board towards the other. To do this, connect TX and RX lines of any USART to the RX and TX lines of another board respectively (that is to say TX connects to RX and visa versa) using the jumper cables.
Make sure that the TX and RX pair belong to the same USART (denoted by the number after TX and RX).

For the LED strip, i can be any length you wish. Lay it out in the direction of the connecting board and connect the data wire to the corresponding LED pin, and connect the ground pins together across the entire network. Assuming you bought a long reel LED strip, you will need to solder new wires onto the connection points You can connect multiple boards together in a larger network of boards and it should work regardless of the number of boards.

To power all the LED strips, you’ll need to wire them all together into the 12 volt power supply. It’s easiest to connect the grounds of all the LED strips and boards together into the power supply so that they share a common ground. The easiest way to power the boards themselves is to plug a microUSB cable into them and run them all into a powered usb hub.
Alternatively if you have access to a 3 volt power supply you can power them through the 3.3 volt pins on the board.

## Software

To get started you’ll need: 

Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain
openOCD & GDB

To install rust, follow the instructions [here](https://www.rust-lang.org/tools/install)
To install and switch to the nightly branch of rust, type in the following commands:

``` console
$ rustup toolchain install nightly
$ rustup default nightly
```

To install openOCD and GDB, go to the bottom of [this](https://rust-embedded.github.io/book/intro/install.html) page and follow the instructions for the platform you are on. 

Once that’s done go ahead and clone the repo to your local machine. If you are running linux you will need to make a small edit to one of the files. In the root folder of the project, go to the .cargo folder and open the config file. You should find a block of code looking like this: 
``` toml
# uncomment ONE of these three option to make `cargo run` start a GDB session
# which option to pick depends on your system
runner = "arm-none-eabi-gdb -q -x openocd.gdb"
# runner = "gdb-multiarch -q -x openocd.gdb"
# runner = "gdb -q -x openocd.gdb"
```
Comment out ‘runner = "arm-none-eabi-gdb -q -x openocd.gdb"’ and uncomment ‘runner = "gdb-multiarch -q -x openocd.gdb"’ 

### Flashing code to the boards
Once that’s done plug in one of the boards to upload the code onto. Then run the cargo build and run commands to automatically load the code onto the board:

``` console
$ cargo build
$ cargo run
```
Once the the command has finished running, the code will be on the board. Unplug it from your computer and connect the next board. Repeat this process until all the boards in the array have code uploaded to them and are plugged into your network of boards.

## Further Reading

- [STM32 HAL library](https://docs.rs/stm32f1xx-hal/0.2.0/stm32f1xx_hal/)
- [Realtime for the masses (Cortex M)](https://github.com/japaric/cortex-m-rtfm) - Embedded concurrency framework
- [Embedded Rust eBook](https://rust-embedded.github.io/book/)

# License

All code here is licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution

This project was sunsetted on the 22 Feb, 2019. If you're looking to develop this further, I recommend either forking (or cloning) the repo, and working on it seperately. Please contact us for any questions!

## Contact

Primary maintainers:

- [Timothy Lee](github.com/tztimlee)
- [Harrison Turton](github.com/tztimlee)

If you're looking at getting in contact, please email us! (On our profiles)
