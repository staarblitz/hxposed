# HxPosed - Hypervisor eXposed.
A hypervisor based service provider aiming to expose depths of NT kernel to user mode. In a safe way.
Based on [barevisor](https://github.com/tandasat/barevisor)

Demo: [YouTube](https://www.youtube.com/watch?v=EzxZ9oxnZNE)

## What does HxPosed do?
HxPosed grants you hypervisor and kernel level access to your own computer. So you can do anything. That includes playing with Windows internals. Which you most likely love if you are reading this.

And yes, we mean it. There is no bullshit, no-nonsense. That is right. Here is what you get with HxPosed:
- A safe API written in Rust (available for C too),
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
        // Error source: HxPosed. Error Code: Not Found. Not Found What: Process
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
    )
{
    Ok(_) => println!("Process protection changed!"), // Now you can kill services.exe for whatever reason.
    Err(x) => println!("Error changing process protection: {:?}", x),
}
```
#### Change privileges
- No LUIDs.
- No lookups.
```rust
let process = HxProcess::current().unwrap(); // Open current process
let mut token = process.get_primary_token().unwrap(); // Get the token for current process
println!("Token account name: {}", token.get_account_name().unwrap()); // Admin, User, PC whatever

let system = HxToken::get_system_token(); // Open system token
let system_privs = system.get_enabled_privileges().unwrap();

token.set_enabled_privileges(system_privs).unwrap(); // Now I'm the SYSTEM
```
#### Allocate from nonpaged pool
- No IRPs.
- No manual memory management.
- No pointer type conversions.
```rust
let mut descriptor = HxMemory::alloc::<u64>(MemoryType::NonPagedPool);

{
    let mut guard = descriptor.map(HxProcess:current(), 0x13370000).unwrap();
    let ptr = guard.deref_mut();
    *ptr = 0x2009;

    let value = *ptr; // 0x2009
} // automatically unmapped

// automatically freed
```
### And no, its not just Rust.
It works for C, too.
- All in one header file.
- NT-style naming to feel right at home.
```c
HXR_OPEN_PROCESS open = {
    .Id = 6892,
    .OpenType = HxOpenHandle,
};

PHX_REQUEST_RESPONSE raw = HxpRawFromRequest(HxSvcOpenProcess, &open);

if (HxpTrap(raw) == -1) {
    printf("hv not loaded");
    return 1;
}

HXS_OPEN_OBJECT_RESPONSE process;
HX_ERROR error = HxpResponseFromRaw(raw, &process);
if (HxIsError(&error)) {
    printf("fail");
}

TerminateProcess(process.Address, 0); // op access rights
```

Hope you got our point. We are trying to make things easier, not harder.
From now on, you'll never worry about:
- Memory ownership,
- Undocumented NT functions,
- `STATUS_INVALID_PARAMETER`s,
- Digging out offsets and byte patterns.

It *just works*. Because we know how frustrating it is when it *just doesn't*.

Easy. Powerful. No-nonsense.

> [!IMPORTANT]
> Bindings for C# won't be made manually.

## Features
### Core features
These all come out of the box.

Process services:
- Get/set protection
- Get/set mitigation flags
- Get/set signature levels
- Get token
- Swap token
- Get threads
- Get NT path (Not from PEB)
- Open handle with all access

Thread services:
- Suspend/resume/freeze
- Get impersonation token
- Swap impersonation token
- Check if thread is impersonating
- Kill
- Open handle with all access

Memory services:
- Get/set paging type attributes
- Allocate contiguous physical or from nonpaged pool
- Map memory to arbitrary process context

Security services:
- Get system token
- Get/set present privileges
- Get/set enabled privileges
- Get default enabled privileges
- Get source name
- Get account name
- Open handle with all access

Cpu services:
- Read write arbitrary MSR

Callback services:
- Process creation/termination callbacks

### HxLoader features
- Loads HxPosed to the system automatically
- Bypasses PatchGuard

### Additional features
- HxGuard to filter out invalid callers
- NtGuard (Soon to be made) to harden the system

### Kernel features
HxPosed utilizes object reference counting and handle tables just like Windows NT does to keep stability sith system. Here are a few examples.

#### Opening/creating handles:
```rust
impl<T> NtObject<T> {
    pub const LOW_LEVEL_ENTRIES: u64 = 4096 / 0x80;

    pub fn from_ptr(ptr: *mut T) -> Self {
        unsafe {
            Self::increment_ref_count(ptr as _);
        }
        Self { object_addr: ptr }
    }

    pub unsafe fn increment_ref_count(object: *mut c_void) {
        let header = unsafe { object.byte_offset(-0x30) };
        interlocked_increment(header as _);
    }

    pub unsafe fn decrement_ref_count(object: *mut c_void) {
        let header = unsafe { object.byte_offset(-0x30) };
        interlocked_decrement(header as _);
    }

    pub unsafe fn increment_handle_count(object: *mut c_void) {
        let header = unsafe { object.byte_offset(-0x28) };
        interlocked_increment(header as _);
    }

    pub unsafe fn decrement_handle_count(object: *mut c_void) {
        let header = unsafe { object.byte_offset(-0x28) };
        interlocked_decrement(header as _);
    }

    pub fn from_handle(handle: HANDLE, table: PHANDLE_TABLE) -> Result<NtObject<T>, ()> {
        let handle = handle as u64;
        let exhandle = _EXHANDLE { Value: handle };
        let handle_table_entry = unsafe { ExpLookupHandleTableEntry(table, exhandle) };
        if handle_table_entry.is_null() {
            return Err(());
        }

        let object_pointer = unsafe { *(handle_table_entry) }.get_bits(20..64);
        let object_header = (object_pointer << 4 | 0xffff000000000000) as *mut c_void; // decode bitmask to get real ptr

        // object body is always after object header. so we add sizeof(OBJECT_HEADER) which is 0x30 to get object itself
        let object_body = unsafe { get_object_body(object_header) } as *mut T;

        // increment ref count manually so when handle is closed, the object won't be dropped.

        Ok(Self::from_ptr(object_body))
    }

    pub fn create_handle(object: *mut T, table: PHANDLE_TABLE) -> Result<HANDLE, ()> {
        let handle = unsafe { ExCreateHandle(table, get_object_header(object as _) as _) };

        let exhandle = _EXHANDLE { Value: handle as _ };

        // so we need to look up the entry again, because ExCreateHandle grants 0 accesses.
        let entry = unsafe { ExpLookupHandleTableEntry(table, exhandle) };
        if entry.is_null() {
            return Err(());
        }

        let entry = unsafe { &mut *entry };

        // give all access
        entry.set_bits(0..25, 0x1FFFFFF);

        let object_pointer = entry.get_bits(20..64);
        let object_header = (object_pointer << 4 | 0xffff000000000000) as *mut c_void;
        let object = unsafe { get_object_body(object_header) };

        // we have to increment BOTH handle and pointer count since ExCreateHandle does none of that
        unsafe {
            Self::increment_handle_count(object);
            Self::increment_ref_count(object);
        }

        Ok(handle as _)
    }
}
```

#### Using registry
```rust
impl NtKey {
    pub fn open(path: &str) -> Result<NtKey, NtStatus> {
        let unicode_string = UnicodeString::new(path);
        let mut str = unicode_string.to_unicode_string();
        let mut attr = init_object_attributes(
            &mut str,
            ObjectAttributes::KernelHandle,
            Default::default(),
            Default::default(),
        );
        let mut handle = HANDLE::default();

        match unsafe { ZwOpenKey(&mut handle, KeyAccessRights::All, &mut attr) } {
            NtStatus::Success => {}
            err => return Err(err),
        }

        let mut return_size = u32::default();
        let mut info = KEY_FULL_INFORMATION::alloc_sized(512);

        match unsafe {
            ZwQueryKey(
                handle,
                KeyInformationClass::KeyFullInformation,
                as_pvoid!(info),
                512,
                &mut return_size,
            )
        } {
            NtStatus::Success => {}
            err => return Err(err),
        }

        Ok(Self {
            path: String::from_str(path).unwrap(),
            handle: HandleBox::new(handle),
            num_values: info.Values as _,
        })
    }

    pub fn get_value_string(&self, value: &str) -> Result<UnicodeString, NtStatus> {
        let value_ptr = self.get_value::<u16>(value)?;
        Ok(UnicodeString::from_raw(value_ptr))
    }

    pub fn get_value<T>(&self, value: &str) -> Result<&mut T, NtStatus> {
        let mut return_size = u32::default();
        let mut info: Box<KEY_VALUE_FULL_INFORMATION>;
        let string = UnicodeString::new(value);
        let mut value_name = string.to_unicode_string();

        match unsafe {
            ZwQueryValueKey(
                self.handle.get_danger(),
                &mut value_name,
                KeyValueInformationClass::KeyValueFullInformation,
                null_mut(),
                0,
                &mut return_size,
            )
        } {
            NtStatus::BufferTooSmall => {}
            err => return Err(err),
        };

        info = KEY_VALUE_FULL_INFORMATION::alloc_sized(return_size as usize);

        match unsafe {
            ZwQueryValueKey(
                self.handle.get_danger(),
                &mut value_name,
                KeyValueInformationClass::KeyValueFullInformation,
                as_pvoid!(info),
                return_size,
                &mut return_size,
            )
        } {
            NtStatus::Success => {}
            err => return Err(err),
        };

        Ok(unsafe { &mut *get_data!(info, T) })
    }
}
```

#### Using undocumented fields:
```rust
///
/// # Get NT Procedure
///
/// Gets the function at ntosrkrnl.
///
/// ## Arguments
/// * `proc` - Procedure to get pointer of. See [NtProcedure]
///
/// ## Panic
/// * This function panics if the NT version is not supported.
///
/// ## Return
/// * An absolute pointer to [`T`], if found.
///
pub(crate) unsafe fn get_nt_proc<T>(proc: NtProcedure) -> *mut T {
    let build = NT_BUILD.load(Ordering::Relaxed);
    let ubr = NT_UBR.load(Ordering::Relaxed);
    let base = NT_BASE.load(Ordering::Relaxed) as *mut u8;
    unsafe {
        base.add(match (build, ubr) {
            (26100, 6584) /* 25H2 */ => {
                match proc {
                    NtProcedure::PsTerminateProcessProc => 0x91f3d4,
                    NtProcedure::PspGetContextThreadInternal => 0x909940,
                    NtProcedure::PspSetContextThreadInternal => 0x9095f0,
                    NtProcedure::PspTerminateThreadByPointer => 0x8f48f0,
                    NtProcedure::ExpLookupHandleTableEntry => 0x850180,
                    NtProcedure::ExCreateHandle => 0xa1b200,
                }
            }
            _ => panic!("Unknown NT build {}, {}", build, ubr)
        }) as *mut T
    }
}

///
/// # Get `EPROCESS` Field
///
/// Gets pointer to field of `EPROCESS` depending on NT version.
///
/// ## Arguments
/// * `field` - Field you want to acquire pointer to. See [`EProcessField`]
/// * `process` - Process object to get pointer from.
///
/// ## Panic
/// - This function panics if the NT version is not supported.
///
/// ## Returns
/// - Absolute **pointer** to the field, in [`T`].
pub(crate) unsafe fn get_eprocess_field<T: 'static>(
    field: EProcessField,
    process: PEPROCESS,
) -> *mut T {
    let build = NT_BUILD.load(Ordering::Relaxed);
    let ubr = NT_UBR.load(Ordering::Relaxed);
    unsafe {
        process.byte_offset(match (build, ubr) {
            (26100, 6584) /* 25H2 */ => {
                match field {
                    EProcessField::CreateTime => 0x1f8,
                    EProcessField::Token => 0x248,
                    EProcessField::SectionObject => 0x2f8,
                    EProcessField::SectionBaseAddress => 0x2b0,
                    EProcessField::Peb => 0x2e0,
                    EProcessField::SeAuditProcessCreationInfo => 0x350,
                    EProcessField::VadRoot => 0x558,
                    EProcessField::ExitTime => 0x5c0,
                    EProcessField::Protection => 0x5fa,
                    EProcessField::SignatureLevels => 0x5f8,
                    EProcessField::MitigationFlags1 => 0x750,
                    EProcessField::MitigationFlags2 => 0x754,
                    EProcessField::MitigationFlags3 => 0x7d8,
                    EProcessField::ThreadListHead => 0x370,
                    EProcessField::Lock => 0x1c8,
                    EProcessField::ObjectTable => 0x300,
                    EProcessField::Pad => 0xc0,
                    EProcessField::DirectoryTableBase => 0x28,
                    EProcessField::UserDirectoryTableBase => 0x158,
                }
            }
            _ => {
                panic!("Unknown NT build {}, {}", build, ubr)
            }
        }) as *mut T
    }
}
```


## Technical Details
Interested in how HxPosed works?
- Refer to [my blog](https://staarblitz.github.io/)
- Refer to [wiki](https://github.com/staarblitz/hxposed/wiki)

The source code is also extremely descriptive. Here is an example:
```asm
.align 16
.global hx_gp_handler
hx_gp_handler:
    cmp r9, 0x2009          # check if called by us
    je handle_fail

    hlt                     # access violation in hypervisor! bug!

handle_fail:
    xor r9, r9              # signal that it failed
    add rsp, 8              # ignore the error code
    add qword ptr [rsp], 2  # wrmsr/rdmsr is 2 bytes long. since this is a fault, we need to increment rip manually.
    iretq                   # where we were?

.align 16
.global rdmsr_failsafe_naked
# ms x64 calling convention
# rcx is msr id
# rax is returned msr value
# rdx defines if msr exists. -1 if not, 0 if exists.
rdmsr_failsafe_naked:
    mov r9, 0x2009     # put our beloved
    rdmsr
    cmp r9, 0          # check if this triggered a #GP
    jz fail
    shl rdx, 32
    or rax, rdx         # combine with some bitshift
    xor rdx, rdx        # beautiful
    jmp end
fail:
    mov rdx, -1         # no such msr
end:
    ret
```

## Repo structure
`src` contains the code written in Rust.
- `hvcore` the hypervisor core.
- `hxloader` a "bootkit" that patches the Windows boot process so you can load HxPosed.
- `hxposed_core` core API providing access to hypervisor.
- `windows` Windows driver of HxPosed. All the deal happens here.

`HxPosed.GUI` contains the code written in C#.
- `HxPosed.GUI` GUI manager for HxPosed. Written in WPF.
- `HxPosed.Core` wrapper over libhxposed providing C# layer access to hypervisor.
- `libhxposed` native library providing access to hypervisor. Written in C and asm.
- `pocman` simple piece of code demonstrating usage of `libhxposed`.

## Get me to the point
### How to use?
Visit the [wiki page](https://github.com/staarblitz/hxposed/wiki/Setup).

### How to contribute?
There is 2 ways to help me:
1. Give feature requests and test the stuff.
2. Code them yourself.

Of course, coding them yourself would be nicer. But if you are just an everyday guy who enjoys hxposed, the first option will work well too.

Build instructions are given in the wiki.

## What we have so far?
- [x] GetState service.
- [x] Authorization service.
- [x] Async message sending receiving (works with your favourite runtime).
- [x] Plugin permission management.
- [x] Cool fluent UI that fits Windows 11 design.
- [x] Support for AMD and Intel.
- [x] Libraries in different languages (C#, C and Rust) to interact with hypervisor.
- [x] HxGuard to prevent abuse.
- [x] Automated installer for ease.

## What are you waiting for?

## Contact
[Telegram](https://t.me/staarblitz)
