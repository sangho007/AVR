# MCU 설정
MCU = atmega2560
F_CPU = 16000000UL
PROGRAMMER = -c wiring -P /dev/tty.usbmodem11301 -b 115200

# 디렉토리 설정
SRC_DIR = src
LIB_DIR = lib
BUILD_DIR = build

# 모든 소스 파일 자동 검색
# src 디렉토리와 lib 디렉토리의 모든 하위 디렉토리에서 .c 파일을 찾음
SRC = $(shell find $(SRC_DIR) $(LIB_DIR) -name '*.c')

# 모든 헤더 파일 디렉토리 자동 검색
# lib 디렉토리의 모든 하위 디렉토리를 include 경로에 추가
INCLUDES = $(shell find $(LIB_DIR) -type d -exec echo "-I{}" \;)

# 컴파일러 설정
CC = avr-gcc
OBJCOPY = avr-objcopy
AVRDUDE = avrdude

# 컴파일러 플래그
CFLAGS = -Wall -Os -mmcu=$(MCU) -DF_CPU=$(F_CPU) $(INCLUDES)

# 오브젝트 파일 경로 설정
# 소스 파일의 경로를 build 디렉토리 내부 경로로 변환
OBJ = $(SRC:%=$(BUILD_DIR)/%.o)

# 최종 타겟
TARGET = $(BUILD_DIR)/firmware

# 기본 타겟
all: $(TARGET).hex

# 빌드 디렉토리 생성 및 오브젝트 파일 컴파일
$(BUILD_DIR)/%.c.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

# 링킹
$(TARGET).elf: $(OBJ)
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -o $@ $^

# HEX 파일 생성
$(TARGET).hex: $(TARGET).elf
	$(OBJCOPY) -O ihex -R .eeprom $< $@

# 업로드
flash: $(TARGET).hex
	./flash.sh $(TARGET).hex

# 청소
clean:
	rm -rf $(BUILD_DIR)

# 디버그용 정보 출력
debug:
	@echo "Source files:"
	@echo $(SRC)
	@echo
	@echo "Include paths:"
	@echo $(INCLUDES)
	@echo
	@echo "Object files:"
	@echo $(OBJ)

.PHONY: all clean flash debug

