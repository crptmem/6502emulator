rom = [0xea] * 1024  # Initializing ROM with NOP instructions (0xEA)
last_written_addr = 0x0000
print(len(rom))

def write(string):
    global last_written_addr  # Define last_written_addr as a global variable
    string_array = string.encode("ascii")
    for i in range(len(string_array)):
        rom[last_written_addr] = 0xA9  # LDA opcode
        rom[last_written_addr + 1] = string_array[i]  # ASCII byte
        rom[last_written_addr + 2] = 0x8D  # STA opcode
        # Calculate memory address with little-endian ordering
        mem_address = (0x2000 + i)  # Assuming 4 bytes per instruction and address
        rom[last_written_addr + 3] = mem_address & 0xFF  # Low byte of memory address
        rom[last_written_addr + 4] = (mem_address >> 8) & 0xFF  # High byte of memory address
        # Increment the last_written_addr by 5 for each instruction and memory address (5 bytes)
        last_written_addr += 5


write("Hello world!")
rom_file = open("generated.bin", "wb")
rom_file.write(bytearray(rom))
rom_file.close()  # Don't forget to close the file
