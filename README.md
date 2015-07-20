# tellstickreport
Small application to report status from a tellstick device and handle actions from a client.

## Dependencies
To install on the raspberry pi, follow instructions at https://github.com/Ogeon/rust-on-raspberry-pi
libtelldus-core-dev
libssl-dev

Extra instructions for raspberry pi.
Some files has been added to lib/arm-unknown-linux-gnueabihf in order to comile on raspberry pi.
Note that the .so files are from an rpi, they must be native.
libtelldus.so
libcrypto.so
libssl.so
libssl header files (requires environment variables. see https://github.com/sfackler/rust-openssl. Requires download of libssl-dev)
