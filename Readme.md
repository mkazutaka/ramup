![](https://github.com/mkazutaka/ramup/workflows/CI/badge.svg)


# Concept

```makefile
DISK_NAME = RAMDisk
DISK_SIZE = 4194304 # 2GB
APP = myapp
TARGET = src/github.com/mkazutaka
BACKUP = ~/.backup

create:
	diskutil erasevolume HFS+ $(DISK_NAME) `hdiutil attach -nomount ram://$(DISK_SIZE)`
backup:
	mkdir -p /Volumes/$(DISK_NAME)/$(TARGET)
	mv ~/$(TARGET)/$(APP) /Volumes/$(DISK_NAME)/$(TARGET)
	ln -s /Volumes/$(DISK_NAME)/$(TARGET)/$(APP)/ ~/$(TARGET)/$(APP)
restore:
	unlink ~/$(TARGET)/$(APP)
	mv /Volumes/RAMDisk/$(TARGET)/$(APP) ~/$(TARGET)
	diskutil unmount force $(DISK_NAME)
rsync:
	rsync -r /Volumes/$(DISK_NAME)/ $(BACKUP)
```
