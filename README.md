[![crates.io](https://img.shields.io/crates/v/monger.svg)](https://crates.io/crates/monger)

# monger - MongoDB Version Manager

monger is a command-line version manager of MongoDB. It downloads and stores version of MongoDB to
the directory ~/.monger and facilitates running different mongodb binaries (`mongod`, `mongo`, etc.)
by version.

## Supported platforms

`monger` currently only supports Linux and OS X/MacOS

## Installation

Assuming that you have Rust installed, simply run `cargo install monger`. Note that you'll need to
have `~/.cargo/bin` on your PATH to run monger.

## Usage

To print help info:

```
monger --help
```

To print the version:

```
monger --version
```

### Listing installed versions

The command `monger list` will print out all versions of MongoDB being managed by your installation
of monger, as well as an entry listed as "system" if you have a version of `mongod` installed on
your PATH (e.g. from a package manager installation of mongodb).

### Download MongoDB versions

To download a version of MongoDB, use the command `monger get <VERSION>`, where <VERSION> can be
a full semantic version, a release candidate, a major and minor version (which will download the
latest non-release candidate version with the given major and minor version), or the word "latest"
(which will download the latest stable release of MongoDB):

```
monger get 3.5.12
monger get 3.4.8-rc1
monger get 3.4
monger get latest
```

By default, this will do nothing if the version of MongoDB is already installed. To force monger to
download and install the verison of MongoDB from scratch, add `--force`:

```
monger get 3.4.7 --force
```

### Starting mongod

To start mongod, run `monger start <VERSION>`, where <VERSION> can be a full semantic version,
a release candidate, a major and minor version (which will start the latest non-release candidate
version with the given major and minor version), or the word "system" if a version of `mongod` is
present in the user's PATH:

```
monger start 3.5.12
monger start 3.4.8-rc1
monger start 3.4
monger start system
```

To specify additional arguments to `mongod`, simply append `--`:

```
monger start 3.4.7 -- --fork --syslog
```

NOTE: Currently, --dbpath will be set to the directory `~/.monger/db/<VERSION>`. This can't
currently be overridden, although in the future this will be fixed so that users can specific a
non-default path.

### Running a MongoDB binary

To run a MongoDB binary, run `monger run <VERSION> <BIN>`, where <VERSION> can be a full semantic
version, a release candidate, a major and minor version (which will start the latest non-release
candidate version with the given major and minor version), or the word "system" if a version of
`mongod` is present in the user's PATH:

```
monger run 3.5.12 mongo
monger run 3.4.8-rc1 mongotop
monger run 3.4 mongo
monger run system mongotop
```

To specify additional arguments to `mongod`, simply append `--`:

```
monger run 3.4.7 mongo -- --host 1.2.3.4 --port 1234
```

### Deleting MongoDB versions

To delete a version of MongoDB managed by monger, run `monger delete <VERSION>`, where <VERSION> is
a full semantic version of an installed MongoDB version:

```
monger delete 3.5.12
monger delete 3.4.8-rc1
```

### Pruning outdated MongoDB versions

Often after downloading a new release, older patch releases of the same version aren't needed. To
delete these, run `monger prune`. For example, given the versions listed below installed, the marked
versions would be deleted:

```
3.0.14      (deleted)
3.0.15
3.2.10      (deleted)
3.2.16
3.4.6-rc0   (deleted)
3.4.7
3.4.8-rc1
3.5.10      (deleted)
3.5.11      (deleted)
3.5.12
```

Note that release candidates won't be used to determine the newest stable version installed, but
release candidates older than the newest stable release will still be deleted.

## Future work

* Ensure that all errors give proper feedback
* Improve test coverage
* Properly detect SSL libraries on MacOS
* Windows support (?)
