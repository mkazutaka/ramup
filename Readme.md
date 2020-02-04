# ramup: 
Support your work on RAMDisk. This allows for fast file access and makes your work faster

Currently, only support MacOS

![](https://github.com/mkazutaka/ramup/workflows/CI/badge.svg)

## Install

```sh
$ git clone git@github.com:mkazutaka/ramup.git
$ cd ramup
$ cargo build 
$ cargo run -- backup -p ~/example
```

## Usage

```
USAGE:
    ramup [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    backup     backup path to RAMDisk
    clean      clean RAMDisk
    help       Prints this message or the help of the given subcommand(s)
    init       create init file for ramup
    restore    restore path from RAMDisk
```

### Backup to RAMDisk
This command to backup your specified directory to RAMDisk. 
If you don't create RAMDisk on your PC, The command also create it.

```shell
$ ramup backup -p <PATH>
```

Example
```shell
$ ls -la ~ | grep example
lrwxr-xr-x ... example

$ ramup backup -p ~/example

$ ls -la ~ | grep example
lrwxr-xr-x ... example -> /Volumes/RAMDiskByRamup/Users/mkazutaka/example
```

### Restore from RAMDisk
This command to restore your specified directory from RAMDisk. 

```shell
$ ramup restore -p ~/taret/directory
```

Example
```shell
$ ls -la ~ | grep example
lrwxr-xr-x ... example -> /Volumes/RAMDiskByRamup/Users/mkazutaka/example

$ ramup restore -p ~/example

$ ls -la ~ | grep example
lrwxr-xr-x ... example
```

### Application Support
If you execute backup or restore command without path paramter, ramup load configuration file and backup paths in it.
Or you can set original application paths to configuration file

Configuration file's path is set by env value of `RAMUP_CONFIG_PATH`.
And the default path is `~/.config/ramup/config.toml`.

```toml
# RAMDisk Settings
[ram]
name = "RAMDiskByRamup"
size = 8388608

# Application Settings, If you use google chrome settings, name is google_chrome
[[applications]]
name = "google_chrome"

# You can also add your origin settings
[[applications]]
name = "my_browser"
paths = [
  "~/Library/Application Support/MyBrowser",
  "~/Library/Preferences/MyBrowser",
]
```

If you want to know applications that ramup support, please check it [
applications directory](https://github.com/mkazutaka/ramup/tree/master/applications)
Of course, I welcome contribute of this

## License
MIT
