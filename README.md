# runix

Run [nix](https://nixos.org/) software without installing nix!

Nix has tremendous benefits for building, distributing and running software. As a developer, it's absolutely worth your time to install and learn nix.

But I'd like to distribute my software to everyone, and forcing users to first install nix can be a hurdle.

`runix` is a small utility which can run nix-built software, without the complex parts of nix that make installation nontrivial:

 - nix channel & binary cache configuration
 - build user setup
 - root access

# How it works:

Runix won't help you build software, just run it. It can run anything that's been pulished to a Nix binary cache.

Each nix derivation fetched from a binary cache is an archive with one assumption: each of its dependencies are present in `/nix/store/XXXXXXXXXX-name`. Runix rewrites all software so that it instead looks in `/tmp/runix/XXXXXXXXXX-name`. It installs a symlink to the real store path (which lives under `~/.cache/runix`). And that's it, you can build & distribute software with nix tooling, but users can run it without installing nix.

# Using it:

## Installation:

```bash
curl -sSL https://raw.githubusercontent.com/timbertson/runix/main/bootstrap.sh | bash
```

This will fetch runix into `~/.cache/runix`, then install a `runix` symlink into the first writeable entry on `$PATH`.

## Interactive use:

To run an arbitrary derivation, you need to know its name:

**Note**: these examples use a version of `jq` for macOS x86_64, other platforms will have different derivations.

```console
$ runix --require dfnijzy9vy1zk0waj47vvx27ffc36lbz-jq-1.6-bin jq --help
```

To figure out the name for your current platform and nixpkgs version, you can run e.g.:

```console
$ nix-instantiate --eval -A jq.outPath '<nixpkgs>'
"/nix/store/wdyfn985sx8001qnsb525fbgm151wm2r-jq-1.6-bin"
```

## Creating & using runscripts:

Typically, you'll want to generate a runscript for convenience:

```console
$ runix --save jq --entrypoint dfnijzy9vy1zk0waj47vvx27ffc36lbz-jq-1.6-bin bin/jq
```

Users with `runix` installed can execute this like any other executable:

```console
$ ./jq --help
```

You can commit this into source control, anyone running it is guaranteed to run the exact same executable and all of its dependencies.

### Creating a runscript from a nix expression:

```console
$ runix --save jq --expr --entrypoint '(import <nixpkgs> {}).jq' bin/jq
```

**Note:** this will not build an expression, only compute its identity. The resulting script will fail unless the derivation has been pushed to the nixos binary cache.

### Self-bootstrapping runscripts:

You can pass `--auto-bootstrap` when saving a runscript. This will make a slightly less efficient wrapper script which first installs runix itself if it's not already installed. The upside is of course you don't need to instruct your users to first install runix.

### Multiplatform runscripts:

There are a few ways to create a multiplatform runscript.

#### Cross-compilation:

You can utilize nix's cross compilation support to build all platforms at once (and save a runscript using `--multiplatform`). This is extremely convenient compared to building across different machines, but cross-compilation is generally harder to get things building correctly. Runix itself is built this way, see `./nix/all-platforms.nix` and `./nix/runix.nix` for details.

#### Multiplatform evaluation:

If you don't want to use cross-compilation, you can still build a multiplatform nix expression. See `./build/multiplatform.nix` for an example. The resulting expressions will not be buildable on a single system - you will need to arrange for each platform's expression to be built on that platform (and pushed to a binary cache). But you can still _evaluate_ all expressions locally to produce a multiplatform runscript.

#### Merge runscripts from multiple platforms:

The most manual way to create a cross-platform runscript is to create individual runscripts per-platform (e.g. as part of a multiplatform CI build), then use `--merge-into` to merge them into a single file. This is conceptually simple, but logistically complex since it requires you to build & evaluate on each platform.

## Distributing arbitrary software:

If your derivation is not already on the public nixos cache, you will need to push it to a binary cache. Any nix-compatible binary cache will do, I recommend [cachix](https://www.cachix.org/).

When using a binary cache, be sure to pass e.g. `--with-cache https://CACHE_NAME.cachix.org` when generating a runscript so that runix will be able to find the published artifacts.

## Fixing cache issues:

You shouldn't need to, but if you need to dig into the cache and remove entries manually you should set `$RUNIX_CHECK=1` afterwards. By default runix doesn't check the dependencies of an existing cache entry, since it never writes a store path without first writing dependencies. With this environment variable set runix will traverse all dependencies and make sure they exist, even dependencies behind already-cached store paths.

---

# Roadmap / currently missing features:

### Integrity checking:

 - Check nar files retrieved from caches
 - Include cache signatures in runscripts
 - Allow users to lock down allowed keys / caches?

### Garbage collection:

 - GC all paths not required in the last `n` days.

### Private cache:

 - Users will need to store keys in config somewhere to access a private cache

### Multi-user:

Runix uses a hardcoded remplacement path of `/tmp/runix`. If you have multi users, that won't work. One day the path may be a hash of `runix#username` for example, which would support concurrent users. It can only be 5 characters long though, since it must replace `store` without changing the length of modified files.

### Download improvements:

Support parallel downloads and add retries for transient HTTP / network errors.

### Self-update:

 - provide a builtin way to update from the current bootstrap release's runscript
