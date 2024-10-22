# ramup


Backup your work on RAMDisk (only for MacOS)

![](https://github.com/mkazutaka/ramup/workflows/CI/badge.svg)

![Feb-08-2020 12-10-28](https://user-images.githubusercontent.com/4601360/74078309-2ff91d80-4a6c-11ea-8ded-dd46ba4fdbda.gif)


## Install

```sh
$ cargo install ramup
```

### Backup
This command backup your specified directory to RAMDisk.
Orignal path is replaced as symbolic link.
(BTW, It is not actual backup. if you shutdown your PC, your file go away)

```shell
$ ramup backup -p <PATH>
```

### Restore
This command restore actual files from symbolic link.

```shell
$ ramup restore -p ~/taret/directory
```

### Config
Ramup also support config file. 
It is useful if you want to change detail option or use application settings existed.

Configuration file's path is `~/.config/ramup/config.toml`
You can change through env value `RAMUP_CONFIG_PATH`.

Example
```toml
# RAMDisk Settings
[ram]
# RAMDisk's name
name = "RAMDiskByRamup"
# RAMDisk's size: 4096 * 2048 (1MB)  = 4GB
size = 8388608

# Application Settings
[[application]]
# see: https://github.com/mkazutaka/ramup/tree/master/applications
name = "google-chrome"

# You can also add your origin settings
[[applications]]
name = "my_browser"
paths = [
  "~/Library/Application Support/MyBrowser",
  "~/Library/Preferences/MyBrowser",
]
```

## License
MIT
