# This makefile is used to build the Arduino project without the IDE or CLI. It
# is intended to be used inside the docker container that gets automatically
# created by `Makefile`
#
# Stole some tricks from this article: https://ismail.badawi.io/blog/automatic-directory-creation-in-make/

#############
# Variables #
#############

ARDUINO_ROOT=/root/.arduino15/packages/arduino
ARCHIVE=$(ARDUINO_ROOT)/tools/arm-none-eabi-gcc/7-2017q4/bin/arm-none-eabi-ar
BOSSAC=$(ARDUINO_ROOT)/tools/bossac/1.9.1-arduino5/bossac
CC=$(ARDUINO_ROOT)/tools/arm-none-eabi-gcc/7-2017q4/bin/arm-none-eabi-gcc
CPP=$(ARDUINO_ROOT)/tools/arm-none-eabi-gcc/7-2017q4/bin/arm-none-eabi-g++
OBJCOPY=$(ARDUINO_ROOT)/tools/arm-none-eabi-gcc/7-2017q4/bin/arm-none-eabi-objcopy

RENESAS=$(ARDUINO_ROOT)/hardware/renesas_uno/1.1.0
ARDUINO=$(RENESAS)/cores/arduino
UNOWIFI=$(RENESAS)/variants/UNOWIFIR4

MAIN=main
BUILD=build
ARDUINO_OUT=$(BUILD)/arduino

B_FLAGS=-mcpu=cortex-m4 -mfloat-abi=hard -mfpu=fpv4-sp-d16
A_FLAGS=$(B_FLAGS) -fsigned-char -ffunction-sections -fdata-sections -DARDUINO=10607 -DARDUINO_UNOWIFIR4 -DARDUINO_ARCH_RENESAS_UNO -DARDUINO_ARCH_RENESAS -DARDUINO_FSP -D_XOPEN_SOURCE=700 -I$(ARDUINO)/api/deprecated -I$(ARDUINO)/api/deprecated-avr-comp -I$(ARDUINO) -I$(UNOWIFI) -iprefix$(RENESAS) @$(UNOWIFI)/includes.txt -I$(RENESAS)/libraries/WiFiS3/src
INO_FLAGS=-c -w -Os -g3 -nostdlib -DF_CPU=48000000 -DNO_USB -DBACKTRACE_SUPPORT -DARDUINO_UNOR4_WIFI $(A_FLAGS) -fmessage-length=0 -fno-builtin -mthumb @$(UNOWIFI)/defines.txt -DCFG_TUSB_MCU=OPT_MCU_RAXXX -I$(ARDUINO)/tinyusb
CMN_FLAGS=$(INO_FLAGS) -fno-use-cxa-atexit -fno-rtti -fno-exceptions -std=gnu++17

ASMFLAGS=$(A_FLAGS) -c -g -x assembler-with-cpp -Os
CCFLAGS=$(INO_FLAGS) -std=gnu11 -MMD
CPPFLAGS=$(CMN_FLAGS) -MMD
LNKFLAGS=-Wl,--gc-sections --specs=nosys.specs -w $(B_FLAGS) -o $(BUILD)/$(MAIN).elf -L$(BUILD) -L$(UNOWIFI) -T$(UNOWIFI)/fsp.ld $(BUILD)/$(MAIN).cpp.o $(ARDUINO_OUT)/tmp_gen_c_files/common_data.c.o $(ARDUINO_OUT)/tmp_gen_c_files/main.c.o $(ARDUINO_OUT)/tmp_gen_c_files/pin_data.c.o $(ARDUINO_OUT)/variant.cpp.o -Wl,--whole-archive -Wl,--start-group $(UNOWIFI)/libs/libfsp.a $(ARDUINO_OUT)/core.a -Wl,--no-whole-archive --specs=nano.specs -lstdc++ -lsupc++ -lm -lc -lgcc -lnosys -Wl,--end-group -Wl,-Map,$(BUILD)/$(MAIN).map
SKETCHFLAGS=$(CMN_FLAGS) -x c++ -E -CC

#########
# Files #
#########

ARDUINO_OUT_SOURCES=$(shell find $(ARDUINO) -name '*.cpp' -o -name '*.S' -o -name '*.c')
ARDUINO_OUT_TARGETS=$(patsubst $(ARDUINO)/%, $(ARDUINO_OUT)/%.o, $(ARDUINO_OUT_SOURCES))
WIFI_SOURCES=$(wildcard $(RENESAS)/libraries/WiFiS3/src/*.cpp)
WIFI_TARGETS=$(patsubst $(RENESAS)/libraries/WiFiS3/src/%.cpp,$(ARDUINO_OUT)/wifi/%.cpp.o,$(WIFI_SOURCES))
TMP_GEN_C_SOURCES=$(wildcard $(UNOWIFI)/tmp_gen_c_files/*.c)
TMP_GEN_C_TARGETS=$(patsubst $(UNOWIFI)/tmp_gen_c_files/%.c,$(ARDUINO_OUT)/tmp_gen_c_files/%.c.o,$(TMP_GEN_C_SOURCES))

###########
# Folders #
###########

.PRECIOUS: $(BUILD_DIR)/. $(BUILD_DIR)%/.

$(BUILD)/.:
	mkdir -p $@

$(BUILD)%/.:
	mkdir -p $@

###########
# Compile #
###########

.SECONDEXPANSION:

$(BUILD)/sketch_merged.cpp: $(MAIN).cpp | $$(@D)/.
	$(CPP) $(SKETCHFLAGS) $< -o $@

$(BUILD)/$(MAIN).cpp.o: $(MAIN).cpp | $$(@D)/.
	$(CPP) $(CPPFLAGS) $< -o $@

$(ARDUINO_OUT)/variant.cpp.o: $(UNOWIFI)/variant.cpp | $$(@D)/.
	$(CPP) $(CPPFLAGS) $< -o $@

$(ARDUINO_OUT)/wifi/%.cpp.o: $(RENESAS)/libraries/WiFiS3/src/%.cpp | $$(@D)/.
	$(CPP) $(CPPFLAGS) $< -o $@

$(ARDUINO_OUT)/%.cpp.o: $(ARDUINO)/%.cpp | $$(@D)/.
	$(CPP) $(CPPFLAGS) $< -o $@

$(ARDUINO_OUT)/%.S.o: $(ARDUINO)/%.S | $$(@D)/.
	$(CPP) $(ASMFLAGS) $< -o $@

$(ARDUINO_OUT)/tmp_gen_c_files/%.c.o: $(UNOWIFI)/tmp_gen_c_files/%.c | $$(@D)/.
	$(CC) $(CCFLAGS) $< -o $@

$(ARDUINO_OUT)/%.c.o: $(ARDUINO)/%.c | $$(@D)/.
	$(CC) $(CCFLAGS) $< -o $@

$(ARDUINO_OUT)/core.a: $(ARDUINO_OUT_TARGETS) $(WIFI_TARGETS) | $$(@D)/.
	$(foreach d, $^, $(ARCHIVE) rcs $@ $(d);)

$(BUILD)/$(MAIN).elf $(BUILD)/$(MAIN).map: \
		$(ARDUINO_OUT)/core.a \
		$(ARDUINO_OUT)/variant.cpp.o \
		$(BUILD)/$(MAIN).cpp.o \
		$(BUILD)/sketch_merged.cpp \
		$(TMP_GEN_C_TARGETS)
	$(CPP) $(LNKFLAGS)

$(BUILD)/$(MAIN).bin: $(BUILD)/$(MAIN).elf $(BUILD)/$(MAIN).map
	$(OBJCOPY) -O binary -j .text -j .data $(BUILD)/$(MAIN).elf $(BUILD)/$(MAIN).bin

$(BUILD)/$(MAIN).hex: $(BUILD)/$(MAIN).elf $(BUILD)/$(MAIN).map
	$(OBJCOPY) -O ihex -j .text -j .data $(BUILD)/$(MAIN).elf $(BUILD)/$(MAIN).hex

##########
# Others #
##########

.DEFAULT_GOAL := all
all: \
		$(BUILD)/$(MAIN).bin \
		$(BUILD)/$(MAIN).hex

upload:
	stty -F /dev/ttyACM0 1200
	$(BOSSAC) -a -d --port=ttyACM0 -U -e -w "$(BUILD)/$(MAIN).bin" -R

clean:
	rm -rf build/
