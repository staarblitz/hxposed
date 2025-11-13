# hxposed
A hypervisor based service provider aiming to expose depths of NT kernel to user mode. In a safe way.

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

- The building process is simple. You need to go to `src\windows` directory and run `cargo make`. The output will be in `target\debug`.
- Then, build the GUI (if you want) by simply opening up the solution in visual studio and clicking that little tiny build button.
- The driver is not signed. Fire up a vm, connect WinDbg through kdnet [here is how to do that](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/setting-up-a-network-debugging-connection-automatically)
- Use OSR driver loader. Or if you like some fantasy you can use sc as I do.
- The driver should be loaded and running.
- You can now do your stuff and see if it works.
- Feature requests, bug reports are always welcome.

## Have you ever thought you don't "own" your computer?
Have you ever thought that you need "more" of it? That you need to be in more control?

Well, yes. You should be, indeed. Why you should be stuck to user-mode counterparts of some programs and ask your own computer for doing an action?

## HxPosed - Hypervirtualizer-based NT API Exposer.
HxPosed has a simple goal, give a hypervisor service that exposes various hidden, unstandardized parts of the NT kernel.

### Ask yourself...
- Aren't you bored of checking the build number of the system?
- Aren't you bored of jumping around struct fields that change position per build?
- Didn't you just once think a *single* kernel API export to user-mode make the life a lot easier?

You were right.

## Meet hxposed.
- ✅ Written in Rust.
- ✅ Everythingg is documented.
- ✅ No-nonsense. It just works.
- ✅ Different languages, same API with respect to programming style.

What are you waiting for? Grab your GUID and send us a CPUID, asking for authorization free of charge today!

### Contact
[Telegram](https://t.me/staarblitz)
