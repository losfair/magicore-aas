CC := riscv64-unknown-elf-gcc -O2 -ffreestanding -nostdlib -march=rv64ima -mabi=lp64

all: libmagicore_primitives.a

libmagicore_primitives.a: magicore_atomic.o
	ar rcs libmagicore_primitives.a magicore_atomic.o

clean:
	rm -f *.o *.a

.PHONY: clean
