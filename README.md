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
This is the way it was supposed to be all along. Here it comes:

#### Open a process. Easy as it should be
```rust
let mut process = match HxProcess::open(id) {
    Ok(x) => x, // Good. Now we own *full* rights to the process.
    Err(e) => {
        println!("Error opening process: {:?}", e); // Gracefully explains error source, error code and reason.
        // Error source: HxPosed. Error Code: Not Found. Error Reason: No extra information.
        return;
    }
};
```
#### Change its internals
- No offsets.
- No structure definitions.
- No NT version checks.
```rust
match process
    .set_protection(
        ProcessProtection::new()
            .with_audit(false)
            .with_protection_type(ProtectionType::None)
            .with_signer(ProtectionSigner::None),
    ).await
{
    Ok(_) => println!("Process protection changed!"), // Now you can kill services.exe for whatever reason.
    Err(x) => println!("Error changing process protection: {:?}", x),
}
```
#### Change privileges
- No LUIDs.
- No lookups.
```rust
let token = process.get_primary_token().await.unwrap();
println!("Token account name: {}", token.get_account_name().await.unwrap()); // Admin, User, PC whatever

let system_set = HxToken::get_system_present_privileges().unwrap(); // Gets the privilege bitmask of SYSTEM user.
token.set_enabled_privileges(system_set).await; // Overpowered now.
```
#### Allocate from nonpaged pool
- No IRPs.
- No manual memory management.
- No pointer type conversions.
```rust
let mut allocation = match HxMemory::alloc::<u64>(MemoryPool::NonPaged).await.unwrap()

{
    let mut _guard = allocation.map(None).unwrap(); // now _guard is a "&mut u64" we can safely use.
    *_guard = u64::MAX;
} // automatically unmapped "allocation.unmap()"

allocation.free().await;
```

### And no, its not just Rust.
It works for C, too.
```c
HXR_OPEN_PROCESS open = {
    .Id = 6892,
    .OpenType = HxOpenHandle,
};

HX_ASYNC_INFO async = {
    .Handle = HxpCreateEventHandle()
};

PHX_REQUEST_RESPONSE raw = HxpRawFromRequest(HxSvcOpenProcess, &open); // get raw request type
HxpTrap(raw, &async); // call the hypervisor

WaitForSingleObject(async.Handle, INFINITE); // wait for async task to complete

HXS_OPEN_OBJECT_RESPONSE process;
HX_ERROR error = HxpResponseFromAsync(&async, &process); // get result from async task
if (HxIsError(&error)) {
    printf("fail");
}

TerminateProcess(process.Address, 0); // now Address is handle with full op access rights
```

Hope you got our point. We are trying to make things easier, not harder.
From now on, you'll never worry about:
- Memory ownership,
- Undocumented NT functions,
- `STATUS_INVALID_PARAMETER`s,
- Digging out offsets and byte patterns.

It *just works*. Because we know how frustrating it is when it *just doesn't*.

Easy. Powerful. No-nonsense.

![the demo](assets/prev.gif)

> [!IMPORTANT]
> Bindings for C# and C are on the way!

## Technical Details
Here is a diagram of how a guest (the plugin) makes a call.
![Diagram showing how it works](assets/Diagram.png)

And here is a diagram how HxPosed processes it.
![Diagram showing how it works](assets/Diagram2.png)

Refer to [wiki](https://github.com/staarblitz/hxposed/wiki)

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
