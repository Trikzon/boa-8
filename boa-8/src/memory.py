from typing import List


PROGRAM_OFFSET = 512


class Memory:
    def __init__(self, offset=PROGRAM_OFFSET):
        self.offset = offset

        """
        4,096 bytes of RAM usable by the CHIP-8 program.
        The first 512 bytes go unused because they were reserved for the
        original CHIP-8 interpreter in the old days.
        """
        self.ram: List[int] = list([0] * 4096)

        """
        16 general purpose 8-bit registers often named Vx, where x is a
        hexadecimal digit (0 through F).
        VF is reserved for some instructions and programs don't use it.
        """
        self.registers: List[int] = list([0] * 16)

        """
        A 16-bit register generally used to store 12-bit memory addresses.
        An array is used to represent it so it is constrained to 16-bits.
        """
        self.I: int = 0

    # # Writes to ram using the program's address offset
    # def write_ram(self, address: int, value: int):
    #     self.ram[PROGRAM_OFFSET + address] = value
    
    # # Reads from ram using the program's address offset
    # def read_ram(self, address: int) -> int:
    #     return self.ram[PROGRAM_OFFSET + address]
    
    # Writes to ram starting at 0x000000
    def write_ram(self, address: int, value: int):
        self.ram[address] = value
    
    # Reads from ram starting at 0x000000
    def read_ram(self, address: int) -> int:
        return self.ram[address]
    
    def V(self, x: int) -> int:
        return self.registers[x]
    
    def set_V(self, x: int, value: int):
        self.registers[x] = value
    
    def VF(self) -> int:
        return self.registers[0xF]
    
    def set_VF(self, flag: bool):
        if flag:
            self.set_V(0xF, 0x1)
        else:
            self.set_V(0xF, 0x0)
    
    def load_rom(self, path: str):
        file = open(path, "rb")
        rom = list(file.read())
        file.close()

        address = self.offset
        for byte in rom:
            self.write_ram(address, byte)
            address += 1
