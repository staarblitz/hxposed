# hxposed
A hypervisor based service provider aiming to expose depths of NT kernel to user mode. In a safe way.
Based on [barevisor](https://github.com/tandasat/barevisor)

![the demo](assets/prev.gif)

## Repo structure
`src` contains the code written in Rust.
- `hvcore` the hypervisor core.
- `hxposed_core` core API providing access to hypervisor.
- `uefi` UEFI driver. Unusued.
- `windows` Windows driver of hxposed.

`HxPosed.GUI` contains the code written in C#.
- `HxPosed.Core` wrapper over libhxposed providing C# layer access to hypervisor.
- `libhxposed` native library providing access to hypervisor.
- `HxPosed.Plugins` plugin managing code.

## Get me to the point
### How to use?
- Grab the latest release (I'll put it Soon™️)
- Unpack it.
- Disable DSE through some bootkit or whatever. Or use the bootposed bootkit I plan to make just for that purpose.
- Load the driver.
- Idk? Load plugins and enjoy them.
### How to contribute?
There is 2 ways to help me:
1. Give feature requests and test the stuff.
2. Code them yourself.

Of course, coding them yourself would be nicer. But if you are just an everyday guy who enjoys hxposed, the first option will work well too.

Build instructions are given below, do them and code the stuff (don't touch the UI).
### How to test?
Glad you asked.

- The building process is simple. You need to go to `src\windows` directory and run `cargo make`. The output will be in `target\debug`. (first time build may take time (approx 1 min), be patient)
- Then, build the GUI (if you want) by simply opening up the solution in visual studio and clicking that little tiny build button. The outputt will be in `bin\Debug\net10.0-windows`.
- The driver is not signed. Fire up a vm, connect WinDbg through kdnet [here is how to do that](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/setting-up-a-network-debugging-connection-automatically)
- Use OSR driver loader. Or if you like some fantasy you can use sc as I do.
- The driver should be loaded and running.
- You can now do your stuff and see if it works.
- Feature requests, bug reports are always welcome.

## What we have so far?
- [x] GetState service.
- [x] Authorization service.
- [x] Plugin permission management.
- [x] Cool fluent UI that fits Windows 11 design.
- [x] Support for AMD and Intel.
- [x] Libraries in different languages (C#, C and Rust) to interact with hypervisor.
- [x] Registry filtering to allow access to \Software\HxPosed only to HxPosed manager.

## What we need?
- [ ] Implementing the services.
- [ ] Adding plugin loading functionality in UI. (already implemented in code.)

## Contact
[Telegram](https://t.me/staarblitz)
