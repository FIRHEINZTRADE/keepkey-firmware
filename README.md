## KeepKey Build Procedure for Ubuntu 14.04

### Toolchain Installation

Update "apt-get"
```
$ sudo apt-get update
```
Remove gdb package. It causes error during gdb-arm-none-eabi toolchain installation
```
$ sudo apt-get remove gdb
```
Install supporting packages for build
```
$ sudo apt-get install git scons gcc-arm-none-eabi python-protobuf protobuf-compiler fabric exuberant-ctags gdb-arm-none-eabi default-jre
```

Download firmware source code from KeepKey respository (https://github.com/keepkey/keepkey-firmware.git)
```
$ git clone git@gitlab.keepkey.com:embedded-software/keepkey-firmware.git
```

### command line build

First build libopencm3. Change to firmware directory and start make
```
$ cd libopencm3 && make
```
Build final bootloader image. Change to firmware directory and run following command
```
$ ./b -b bldr 
```

Build final application image. Change to firmware directory and run following command
```
$ ./b -b app 
```
The resultant binaries will be located in ./build/arm-none-gnu-eabi/release/bin directory.
