# stm32f3-rust-hello-world
A serial loop-back project to use as a template for the Rust [RTFM framework](http://blog.japaric.io/rtfm-v2/) on the STM32F3 mcu. This project was developed using the STM323 Discovery Board, using the tutorials at [blog.japaric.io](http://blog.japaric.io) for guidance.


This project is and embedded project, meaning `rust-std` is not applicable on our system. To build this project without `rust-std`, use [Xargo](https://github.com/japaric/xargo) instead of Rust's native Cargo.

# Usage
Build:

```$ xargo build```

Connect to the STM32 with openocd over ST-Link:

```$ openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg```

Attach a GDB debug session::
>Note, this application uses semi-hosting to print "Hello, World!" to the console. To allow this, semi-hosting should be enabled in your GDB session, as shown below.

```
$ arm-none-eabi-gdb
>>> target remote :3333
>>> monitor arm semihosting enable
>>> load
>>> tb main
>>> step
```

You can now step through the program with GDB, or just run the program to see it pring "Hello, World!".

For convenience to anyone using VS Code, these steps have been added as custom tasks / launch config for VS Code.

# License
Licensed under the [MIT](/LICENSE) open source license.