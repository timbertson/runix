# runix (working title)

Run [nix][https://nixos.org/] software without installing nix!

Nix has tremendous benefits for building, distributing and running software. As a developer, it's absolutely worth your time to install and learn nix.

But I'd like to distribute my software to everyone, and forcing users to first install nix can be a hurdle.

`runix` is a small utility which can run nix-built software, without the complex parts of nix that make installation nontrivial:

 - nix channel & binary cache configuration
 - build user setup
 - root access

# How it works:

Runix won't help you build software, just run it. It can run anything that's been pulished to a Nix binary cache.

Each nix derivation fetched from a binary cache is a tarball with one assumption: each of its dependencies are present in `/nix/store/XXXXXXXXXX-name`. Runix rewrites all software so that it instead looks in `/tmp/runix/XXXXXXXXXX-name`. It installs a symlink to the real store path (which lives under `~/.cache/runix`). And that's it, you can build & distribute software with nix tooling, but users can run it without installing nix.

# Using it:

## Installation:

```bash
curl -sSL https://raw.githubusercontent.com/timbertson/runix/main/bootstrap.sh | bash
```

This will fetch runix into `~/.cache/runix`, then install a `runix` symlink into the first writeable entry on `$PATH`.

## Interactive use:

To run an abitraty derivation, you need to know its name:

**Note**: these examples use a version of `jq` for macOS x86_64, other platforms will have different derivations.

**TODO**: multiplatform instructions

```bash
$ runix --require dfnijzy9vy1zk0waj47vvx27ffc36lbz-jq-1.6-bin jq --help
```

## Creating & using runscripts:

Alternatively, you can generate a runscript:

```bash
$ runix --save jq --entrypoint dfnijzy9vy1zk0waj47vvx27ffc36lbz-jq-1.6-bin bin/jq
```

Users with `runix` installed can execute this like any other executable:

```bash
$ ./jq --help
```

You can commit this into source control, anyone running it is guaranteed to run the exact same executable and all of its dependencies.

---

# Roadmap / possible features:

### Integrity checking:

 - Check nar files retrieved from caches
 - Allow users to lock down allowed keys / caches?

### Garbage collection:

 - Track metadata about usage & dependencies, then GC all paths not required in the last `n` days.

### Multi-user:

Runix uses a hardcoded remplacement path of `/tmp/runix`. If you have multi users, that won't work. One day the path may be a hash of `runix#username` for example, which would support concurrent users. It can only be 5 characters long though, since it must replace `store` without changing the length of modified files.

### Concurrent:

Make downloads robust to concurrent runix processes with some sort of locking.

### Figuring out store identities:

It'd be nice to say I want "python" and have that calculated from e.g. `nixpkgs-stable.python`.

 - something like channels?
 - call hydra's API?
 - embed an actual nix evaluator? This seems heavyweight

For publication of runscripts, we can assume nix is installed. It'd be great to be able to evaluate the derivation path for a different platform without actually executing nix on that platform. Obviously you still need to build on that platform, but that can be a separate procss (github action or hydra builder), whereas script generation is convenient if you can simply calculate details for all platforms locally.

## Self-update:

 - store a runscript in the release, then provide a way to fetch this and make it current

## Caches

Need to list the binary caches you trust, plus some kind of keys.

If we share a single store and let the distributable provide its own cache, it can inject malicious entries elsewhere in the cache.
But you're running the sotware anyway, it could just do Bad Things directly.

How to specify caches? When you run? Or in a config file.

# Garbage collection:

 - how do we manage roots? A user-specific config file would do, but we surely want to support anonymous targets.
 - maintain access timestamps, and cleanup targets not accessed within a given time?

# Caveats:

**Statically compiled binaries** (typically golang) will not be modified nor be subject to runtime inection. This is typiclly fine, as the primary purpose for both of these. The main issue is likely to be hardcoded paths in the compiled binary. The software will run, but default paths may not be found.

TODO: can / should we rewtite all /nix/store references, not just the ones istall_name_tool can edit? This would fix e.g. wrapper scripts.

**LC_ID_DYLIB**: This can't be edited, so software compiled against runix-provided dependencies may link against missing paths. There will be workarounds, but this is not a common enough use case to think about yet.
