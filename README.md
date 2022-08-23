# runix (working title)

# Motivation / goals

 - run nix software without installing nix

# Technology:

First idea: LD_PRELOAD and its equivalent on OSX. I want to support linux and mac, I don't care about windows.

This won't support statically-linked binaries, like golang. That might be fine, as they're likely to be relocatable.

runix -c /nix/store/xxx-yyyy foo

 - caches the given path
 - runs `foo` with:
   - PATH set to _cached_ store paths (x/bin forall x in closure)
   - LD_PRELOAD set to nixr inject hook, which we pull out from next to here (realpath shenanigans?)
     - redhook or similar (rust)

Hooking:
 - different syscalls on linux/osx
 - only need _read_ syscalls, which should be much simpler than e.g. `proot` easy. Any mutating syscall is allowed to fail, since they shouldn't be called.
   - if we wanna be fancy we could EPERM modification syscalls when accessing /nix, but it's probably not necesary

# Roadmap / features in order of urgency:

Extremely vague and imaginary...

### Basic sandbox

Minimal `chroot` / `proot` implementation remapping read requests for `/nix/*` to a corresponding path under $RUNIX_ROOT. It's not really a sandbox, it's more like a symlink in a trenchcoat.

### Binary Cache API:

Fetch implementations (recursively) from binary cache. Start with official nix one, then cachix.

### Installer:

Some kind of bash monstrosity, I'm sure.

### Distributable:

Distribute files which run via runix.

Should they auto-bootstrap if it's not installed? It's gross, but could be very convenient.

### Configuration

Figure out what's useful. Do you want to register aliases for specific nix paths? Seems tedious and impossible to update.

Can we hook into nix channels? Hydra? Custom format linking logical names to paths?

Let's say V1 can only run distributables, which list paths expicitly (and may optionallu have an entrypoint, docker style)

# Unanswered questions:

## How do we resolve something logical into a nar hash?

- embed a nix evaluator?
  - pretty heavyweight
  - doesn't do much without channels, which users won't have
- could we get it from hydra? cachix?
- pull it from some artifact store, possibly github
  - I'd like to avoid inventing a new format, but it may be necessary
- bake it into some distributable
  - `nixr --generate` runs on a nix-enabled machine. You give it a bunch of expressions (and maybe even platforms?), and it generates nixr wrapper script(s) which act like a lightweight pointer to a nix shell
  - we have to push this stuff to a cache anyway, it could be part of the same process?
  - probably better to have a standalone JSON format, but this generated script could embed such a thing for distribution
    - or both: a JSON file with a shebang preamble (ilke nix-run or whatever it's called)

## Caches

Need to list the binary caches you trust, plus some kind of keys.

If we share a single store and let the distributable provide its own cache, it can inject malicious entries elsewhere in the cache.
But you're running the sotware anyway, it could just do Bad Things directly.

How to specify caches? When you run? Or in a config file.

# Garbage collection:

 - how do we manage roots? A user-specific config file would do, but we surely want to support anonymous targets.
 - maintain access timestamps, and cleanup targets not accessed within a given time?
