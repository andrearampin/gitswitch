# gitswitch

Small util for switching branch within a git repository.

```
$ git clone git@github.com:andrearampin/gitswitch.git
$ cd gitswitch
$ cargo build
$ cp ./target/debug/gitswitch /usr/local/bin/
```

At this point enter a git repository and run `gitswitch`

```
➜  gitswitch git:(master) ✗ gitswitch
? Switch to: ›
  master
❯ test
➜  gitswitch git:(test) ✗
```
