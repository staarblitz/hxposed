# hxposed
A hypervisor based service provider aiming to expose depths of NT kernel to user mode. In a safe way.
Based on [barevisor](https://github.com/tandasat/barevisor)

## HxPosed (Hypervisor eXposed)
HxPosed gives you full control over your computer by breaking free from the limitations imposed by traditional user-mode access. With HxPosed, you can:
- Protect or unprotect processes at will
- Create or delete files anywhere on your system
- Build custom computer management software
- And much more...

### The Problem
In the standard user-mode environment, you're restricted in what you can do. For example, you can't:
- Delete system-protected files
- Kill processes that the OS restricts
- Create files in system-protected directories
- Or anything that is fun!

In short, you're not fully in control of your own machine.

### HxPosed - The Solution
HxPosed changes that by giving you direct access to both the kernel and user mode. It’s a hypervisor-based kernel driver that uses VMCALLs (CPUID traps, actually) to manage both the kernel and user space.

Since hypervisors operate in Ring -1 (a theoretical level even more privileged than the kernel), HxPosed enables you to leverage powers beyond the standard kernel-level access. This isn't just about optimizing Windows; it’s about owning it.

And yes, we purposefully mean it. There is no bullshit, no-nonsense. That is right. Here is what you get with HxPosed:
- A safe API written in Rust (available for C# and C too),
- A beautifully documented hypervisor interface,
- A no-nonsense "it just works" functionality.

## See It In Action
This is just a fraction of what HxPosed can offer to you.
### The Interface
```rust
    // Open a process via help of the kernel.
    let mut process = match HxProcess::open(id) {
        Ok(x) => x, // Good. Now we own *full* rights to the process.
        Err(e) => {
            println!("Error opening process: {:?}", e); // Gracefully explains error source, error code and reason.
        // Error source: HxPosed. Error Code: Not Found. Error Reason: No extra information.
            return;
        }
    };

    // Want to set its protection level? No problem.
    match process
        .set_protection(
            ProcessProtection::new()
                .with_audit(false)
                .with_protection_type(ProtectionType::None)
                .with_signer(ProtectionSigner::None),
        )
        .await
    {
        Ok(_) => println!("Process protection changed!"), // Now you can kill services.exe for whatever reason.
        Err(x) => println!("Error changing process protection: {:?}", x),
    }

    // Has anxiety problems? No problem.
    let protection = match process.get_protection() {
        Ok(x) => x, // Now you know services.exe has no chance to escape your task manager.
        Err(e) => {
            println!("Error getting process protection: {:?}", e);
            return;
        }
    };
```

It's a bit unsafe at the core, but thats the price you pay for the power. Don't worry, you, as a plugin developer, will never have to worry about those.

It *just works*. Because we know how frustrating it is when it *just doesn't*.


### The Documentation
We are documenting whatever we are doing. We guarantee you, you will never have questions about what you can expect from HxPosed. Here is an example:
```rust
    ///
    /// # Get Protection
    ///
    /// Gets the internal process protection object. The `_PS_PROTECTION`.
    ///
    /// ## Panic
    /// - This function panics if hypervisor returns anything else than [`GetProcessFieldResponse::Protection`]. Which it SHOULD NOT.
    /// - Issue a bug report if you observe a panic.
    ///
    /// ## Permissions
    /// - [`PluginPermissions::PROCESS_EXECUTIVE`]
    ///
    /// ## Returns
    /// * [`ProcessProtection`] - [`ProcessProtection`] object.
    /// * [`HypervisorError`] - Most likely an NT side error.
    ///
```

Easy. Powerful. No-nonsense.

![the demo](assets/prev.gif)

> [!IMPORTANT]
> Bindings for C# and C are *outdated*! They are not the main concern until core functionality is implemented

## Repo structure
`src` contains the code written in Rust.
- `hvcore` the hypervisor core.
- `hxposed_core` core API providing access to hypervisor.
- `uefi` UEFI driver. Unusued.
- `windows` Windows driver of hxposed.

`HxPosed.GUI` contains the code written in C#.
- `HxPosed.Core` wrapper over libhxposed providing C# layer access to hypervisor.
- `libhxposed` native library providing access to hypervisor. Written in C and asm.
- `HxPosed.Plugins` plugin managing code.
- `HxPosed.GUI` GUI manager for HxPosed. Written in WPF.

## Get me to the point
### How to use?
- Grab the latest release (I'll put it Soon™️)
- Unpack it. (7-zip is nice)
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
- Use OSR driver loader. Or if you like some fantasy you can use `sc` like I do.
- The driver should be loaded and running.
- You can now do your stuff and see if it works.
- Feature requests, bug reports are always welcome.

## What we have so far?
- [x] GetState service.
- [x] Authorization service.
- [x] Async message sending receiving (works with your favourite runtime).
- [x] Plugin permission management.
- [x] Cool fluent UI that fits Windows 11 design.
- [x] Support for AMD and Intel.
- [x] Libraries in different languages (C#, C and Rust) to interact with hypervisor.
- [x] Registry filtering to allow access to \Software\HxPosed only to HxPosed manager.
- [x] Adding plugin loading functionality in UI.

## What we need?
- [ ] Implementing the services. (See [issue tracker](https://github.com/staarblitz/hxposed/issues/1))

## Contact
[Telegram](https://t.me/staarblitz)
