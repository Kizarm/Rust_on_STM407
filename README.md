# Rust_on_STM407

Minimal Cortex-M (STM32F4) Rust Example
=======================================

This demonstrates how to write, build, and link a very minimal program for the
STM32F4 microcontroller (a Cortex-M4 using the ARMv7E-M instruction set).

Note that this program is *very* minimal.  It blink an blue LED on Discovery.
Setting hardware is not exactly correct, but work.

This is all tested on Rust nightly (1.22.0-nightly).

I've done my best to comment things like crazy.  Here's a rough tour of the
project:

- The Rust bits:
  - `Cargo.toml` defines the build.  The main interesting bits are the
    dependency on `rust-libcore` (avoiding the standard library) and the use of
    `panic = "abort"` (avoiding the need for some unwinding code).
  - `src/main.rs` is a self-contained program, including definitions for the
    hardware vector table.
  - I've included `Cargo.lock` to nail down the dependencies and help you
    reproduce my results.

- The build environment:
  - `thumbv7em-none-eabi.json` is an LLVM target definition for the Cortex-M4,
    lightly modified from the definition used by [Zinc](http://zinc.rs).  It's
    very cool that we can just plug this in without modifying the toolchain.
  - `.cargo/config` overrides the default linker to use the ARM one.
  - `layout.ld` is a linker script that describes how to map the compiled
    program into an STM32F4 binary image (mapped to ram).

## Building with native Rust and ARM toolchain

This approach has only been tested on Linux.

Install Rust and a toolchain for `arm-none-eabi` (such as [the one packaged for
Ubuntu](https://launchpad.net/gcc-arm-embedded)).

Clone this repository.

Building this project requires the `nightly` toolchain, so (in the clone) run

    $ rustup override add nightly

Now you can just run:

    $ cargo build

The ARM binary will be deposited in `target/thumbv7em-none-eabi/debug/blink`.

Release-mode builds work too; add `--release`.

Mrazíkovy české poznámky k realizaci.

- Vyzkoušení módního jazyka není na škodu. Rust je obecně deklarován jako "bezpečný".
  Tím by mělo odpadnout mnoho pravidel, která je nutné používat v C pro lepší bezpečnost
  a přenositelnost kódu, tady to ohlídá překladač. Bohužel zavedené restrikce v jazyce
  rust jsou pro přímý přístup na hardware dost omezující, takže většina kódu je stejně
  uzavřena do bloku unsafe.
- Pro přímý přístup do registrů je v C zavedena metoda, deklarující registry jako volatile,
  což stačí k tomu, aby zamezil překladači optimalizaci. V rustu to asi taky nějak jde,
  ale jak jsem zkoumal zdrojáky zinc, je to komplikované. Zinc se mi nepodařilo rozchodit,
  takže je vidět, že i s přenositelností to nebude nijak slavné. Nakonec se podařilo
  použít samostatné VolatileCell, což asi pochází také ze zinc, zde to funguje.
- Délka produkovaného kódu je zhruba stejná jako v C, záleží na optimalizaci a ta je také
  záležitostí LLVM. Na tomto jednoduchém příkladu je zřejmé, že je to celkem efektivní,
  je zapnuta optimalizace i v debug módu, takže release se liší jen nepřítomností ladících
  informací. Otázkou zůstává, jak se s tím poradí normální gdb (pro rust se asi nějak upravuje).
- Otázka použitelnosti není na tak malém příkladu relevantní. Příklad zinc je však varováním,
  že to nebude tak jednoduché. Pro mne osobně je zatím C++ lepší variantou, tahle cestička
  je už dost dobře prošlapaná i pro dost velké projekty. Nicméně není vidět žádná překážka,
  která by vážně bránila rust pro Cortex-Mx používat.

Nechme to uzrát, zkoušel jsem jazyk D, taky to fungovalo a skoro nikdo to nepoužívá,
většina lidí je konzervativní a zůstává u čistého C. Tyhle nové jazyky jsou možná dobré
pro normální programovaní na PC, na malých procesorech bývají problémy, které musí řešit
někdo, kdo ovládá jak ten jazyk, tak všechna úskalí vestavných systémů. Ten průnik
bývá malý.
