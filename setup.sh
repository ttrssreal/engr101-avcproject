# Install's cross-compilation toolchain
# - linux x86-64 host system
# - target rpi 0

toolchain_location="https://master.dl.sourceforge.net/project/raspberry-pi-cross-compilers/Raspberry Pi GCC Cross-Compiler Toolchains/Stretch/GCC 10.3.0/Raspberry Pi 1, Zero/cross-gcc-10.3.0-pi_0-1.tar.gz"
toolchain_fname="cross-gcc-10.3.0-pi_0-1.tar.gz"

mkdir cross && cd cross
wget "${toolchain_location}"
tar xvf "$toolchain_fname"
rm "$toolchain_fname"

echo "Done."