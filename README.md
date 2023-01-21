# zx - unpack archives

Making extracting archives plain and simple.


## Setup

```
$ brew tap jondot/tap && brew install zx
```
Otherwise, grab a release from [releases](https://github.com/rusty-ferris-club/recon/releases).

## Usage

Any of these will work:

```
$ zx files.tar.gz
$ zx files.tar.zst
$ zx files.tar.bz2
$ zx files.gz
$ zx files.txz
$ zx files.zip
.. and more ..
```

Extract to a folder:

```
$ zx files.tar.gz out
```

Strip one component (remove wrapper folder) from all paths:

```
$ zx --strip 1 files.tar.gz out
```

Filter only matching entries (given a regex):

```
$ zx --filter 'foo/.*' files.tar.gz out/
```

List an archive:

```
$ zx --list files.tar.gz
```

# Contributing

We are accepting PRs. Feel free to [submit PRs](https://github.com/jondot/zx/pulls).

To all [Contributors](https://github.com/jondot/zx/graphs/contributors) - you make this happen, thanks!

# License

Copyright (c) 2023 [@jondot](http://twitter.com/jondot). See [LICENSE](LICENSE.txt) for further details.
