from memory import Memory

def load(memory: Memory):
        # Font Character: 0
        memory.write_ram(0x00, 0b11110000) # ****
        memory.write_ram(0x01, 0b10010000) # *  *
        memory.write_ram(0x02, 0b10010000) # *  *
        memory.write_ram(0x03, 0b10010000) # *  *
        memory.write_ram(0x04, 0b11110000) # ****

        # Font Character: 1
        memory.write_ram(0x05, 0b00100000) #   *
        memory.write_ram(0x06, 0b01100000) #  **
        memory.write_ram(0x07, 0b00100000) #   *
        memory.write_ram(0x08, 0b00100000) #   *
        memory.write_ram(0x09, 0b01110000) #  ***

        # Font Character: 2
        memory.write_ram(0x0A, 0b11110000) # ****
        memory.write_ram(0x0B, 0b00010000) #    *
        memory.write_ram(0x0C, 0b11110000) # ****
        memory.write_ram(0x0D, 0b10000000) # *
        memory.write_ram(0x0E, 0b11110000) # ****

        # Font Character: 3
        memory.write_ram(0x0F, 0b11110000) # ****
        memory.write_ram(0x10, 0b00010000) #    *
        memory.write_ram(0x11, 0b11110000) # ****
        memory.write_ram(0x12, 0b00010000) #    *
        memory.write_ram(0x13, 0b11110000) # ****

        # Font Character: 4
        memory.write_ram(0x14, 0b10010000) # *  *
        memory.write_ram(0x15, 0b10010000) # *  *
        memory.write_ram(0x16, 0b11110000) # ****
        memory.write_ram(0x17, 0b00010000) #    *
        memory.write_ram(0x18, 0b00010000) #    *

        # Font Character: 5
        memory.write_ram(0x19, 0b11110000) # ****
        memory.write_ram(0x1A, 0b10000000) # *
        memory.write_ram(0x1B, 0b11110000) # ****
        memory.write_ram(0x1C, 0b00010000) #    *
        memory.write_ram(0x1D, 0b11110000) # ****

        # Font Character: 6
        memory.write_ram(0x1E, 0b11110000) # ****
        memory.write_ram(0x1F, 0b10000000) # *
        memory.write_ram(0x20, 0b11110000) # ****
        memory.write_ram(0x21, 0b10010000) # *  *
        memory.write_ram(0x22, 0b11110000) # ****

        # Font Character: 7
        memory.write_ram(0x23, 0b11110000) # ****
        memory.write_ram(0x24, 0b00010000) #    *
        memory.write_ram(0x25, 0b00100000) #   *
        memory.write_ram(0x26, 0b01000000) #  *
        memory.write_ram(0x27, 0b01000000) #  *

        # Font Character: 8
        memory.write_ram(0x28, 0b11110000) # ****
        memory.write_ram(0x29, 0b10010000) # *  *
        memory.write_ram(0x2A, 0b11110000) # ****
        memory.write_ram(0x2B, 0b10010000) # *  *
        memory.write_ram(0x2C, 0b11110000) # ****

        # Font Character: 9
        memory.write_ram(0x2D, 0b11110000) # ****
        memory.write_ram(0x2E, 0b10010000) # *  *
        memory.write_ram(0x2F, 0b11110000) # ****
        memory.write_ram(0x30, 0b00010000) #    *
        memory.write_ram(0x31, 0b11110000) # ****

        # Font Character: A
        memory.write_ram(0x32, 0b11110000) # ****
        memory.write_ram(0x33, 0b10010000) # *  *
        memory.write_ram(0x34, 0b11110000) # ****
        memory.write_ram(0x35, 0b10010000) # *  *
        memory.write_ram(0x36, 0b10010000) # *  *

        # Font Character: B
        memory.write_ram(0x37, 0b11100000) # ***
        memory.write_ram(0x38, 0b10010000) # *  *
        memory.write_ram(0x39, 0b11100000) # ***
        memory.write_ram(0x3A, 0b10010000) # *  *
        memory.write_ram(0x3B, 0b11100000) # ***

        # Font Character: C
        memory.write_ram(0x3C, 0b11110000) # ****
        memory.write_ram(0x3D, 0b10000000) # *
        memory.write_ram(0x3E, 0b10000000) # *
        memory.write_ram(0x3F, 0b10000000) # *
        memory.write_ram(0x40, 0b11110000) # ****

        # Font Character: D
        memory.write_ram(0x41, 0b11100000) # ***
        memory.write_ram(0x42, 0b10010000) # *  *
        memory.write_ram(0x43, 0b10010000) # *  *
        memory.write_ram(0x44, 0b10010000) # *  *
        memory.write_ram(0x45, 0b11100000) # ***

        # Font Character: E
        memory.write_ram(0x46, 0b11110000) # ****
        memory.write_ram(0x47, 0b10000000) # *
        memory.write_ram(0x48, 0b11110000) # ****
        memory.write_ram(0x49, 0b10000000) # *
        memory.write_ram(0x4A, 0b11110000) # ****

        # Font Character: F
        memory.write_ram(0x4B, 0b11110000) # ****
        memory.write_ram(0x4C, 0b10000000) # *
        memory.write_ram(0x4D, 0b11110000) # ****
        memory.write_ram(0x4E, 0b10000000) # *
        memory.write_ram(0x4F, 0b10000000) # *
