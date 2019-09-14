# TODO beautify this
# and make sure DevkitPRO is installed

# ==== MODIFY THESE ====
CRATE_LIBRARY_NAME := # set the name of your library here
CRATE_LOCATION := .. # set the folder of your crate here
# ======================

boot.o: boot.c
	powerpc-eabi-gcc boot.c -I${DEVKITPRO}/libogc/include -o boot.c

${CRATE_LIBRARY_NAME}:
	pushd ${CRATE_LOCATION}
		xargo build --target powerpc-eabi
	popd

main.elf: boot.o ${CRATE_LIBRARY_NAME}
	powerpc-eabi-gcc boot.o ${CRATE_LOCATION}/target/powerpc-eabi/debug/lib${CRATE_LIBRARY_NAME}.a \
		-g -DGEKKO -mrvl -mcpu=750 -meabi -mhard-float -L${DEVKITPRO}/libogc/lib/wii \
		-lwiiuse -lbte -logc -m -o main.elf

main.dol: main.elf
	${DEVKITPRO}/tools/bin/elf2dol main.elf main.dol

all: main.dol

# Just for being handy.
# Gotta change this in the future
run: main.dol
	dolphin-emu -b -e main.dol
