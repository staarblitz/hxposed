# hxposed
The dream-come-true, user mode kernel driver framework, the NT kernel standardizer...

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
