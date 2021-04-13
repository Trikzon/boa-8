from display import Display
import font
from keyboard import Keyboard
from linked_list import LinkedList
from memory import Memory
from random import randint
from typing import List


SPEED = 10


class Cpu:
    def __init__(self, memory: Memory, display: Display, keyboard: Keyboard):
        self.memory = memory
        font.load(self.memory)

        self.display = display

        self.keyboard = keyboard

        """
        Two special purpose 8-bit registers used as delay and sound timers.
        When these registers are non-zero they auto decrement at 60Hz.
        """
        self.delay_timer: int = 0
        self.sound_timer: int = 0

        # 16-bit program counter that stores the currently executing address.
        self.program_counter: int = self.memory.offset

        """
        A linked-list that stores 16-bit addresses that the interpreter should
        return to when finished with a subroutine.
        CHIP-8 allows for up to 16 levels of nested subroutines, but for
        simplicity we will allow for unlimited.
        """
        self.stack: LinkedList = LinkedList()

        self.is_paused: bool = False
        self.waiting_for_key: bool = False
    
    def cycle(self):
        for _ in range(SPEED):
            if not self.is_paused and not self.waiting_for_key:
                instruction = self.fetch_instruction()
                self.decode_instruction(instruction)
        
        # TODO: Sound

        if not self.is_paused and not self.waiting_for_key:
            self.update_timers()
            self.keyboard.process_keys()

        self.display.update()
    
    def update_timers(self):
        if self.delay_timer > 0:
            self.delay_timer -= 1
        
        if self.sound_timer > 0:
            self.sound_timer -= 1
            
    def fetch_instruction(self) -> int:
        left = self.memory.read_ram(self.program_counter)
        right = self.memory.read_ram(self.program_counter + 1)

        instruction = left << 8 | right

        print(str(self.program_counter) + ": " + str(hex(instruction)))

        # An instruction is 2 bytes, so ready the counter for the next instruction.
        self.program_counter += 2

        return instruction

    def decode_instruction(self, instruction: int):
        x = (instruction & 0x0F00) >> 8
        y = (instruction & 0x00F0) >> 4

        if instruction == 0x000:
            print(f"Hit instruction: {hex(instruction)}; Pausing.")
            return self.PAUSE()

        if instruction & 0xF000 == 0x0000:
            if instruction == 0x00E0:
                return self.CLS()
            elif instruction == 0x00EE:
                return self.RET()
        elif instruction & 0xF000 == 0x1000:
            return self.JP(instruction & 0x0FFF)
        elif instruction & 0xF000 == 0x2000:
            return self.CALL(instruction & 0x0FFF)
        elif instruction & 0xF000 == 0x3000:
            return self.SE_byte(x, instruction & 0x00FF)
        elif instruction & 0xF000 == 0x4000:
            return self.SNE_byte(x, instruction & 0x00FF)
        elif instruction & 0xF000 == 0x5000:
            if instruction & 0x000F == 0x0000:
                return self.SE(x, y)
        elif instruction & 0xF000 == 0x6000:
            return self.LD_byte(x, instruction & 0x00FF)
        elif instruction & 0xF000 == 0x7000:
            return self.ADD_byte(x, instruction & 0x00FF)
        elif instruction & 0xF000 == 0x8000:
            if instruction & 0x000F == 0x0000:
                return self.LD(x, y)
            elif instruction & 0x000F == 0x0001:
                return self.OR(x, y)
            elif instruction & 0x000F == 0x0002:
                return self.AND(x, y)
            elif instruction & 0x000F == 0x0003:
                return self.XOR(x, y)
            elif instruction & 0x000F == 0x0004:
                return self.ADD(x, y)
            elif instruction & 0x000F == 0x0005:
                return self.SUB(x, y)
            elif instruction & 0x000F == 0x0006:
                return self.SHR(x)
            elif instruction & 0x000F == 0x0007:
                return self.SUBN(x, y)
            elif instruction & 0x000F == 0x000E:
                return self.SHL(x)
        elif instruction & 0xF000 == 0x9000:
            if instruction & 0x000F == 0x0000:
                return self.SNE(x, y)
        elif instruction & 0xF000 == 0xA000:
            return self.LD_I(instruction & 0x0FFF)
        elif instruction & 0xF000 == 0xB000:
            return self.JP_V0(instruction & 0x0FFF)
        elif instruction & 0xF000 == 0xC000:
            return self.RND(x, instruction & 0x00FF)
        elif instruction & 0xF000 == 0xD000:
            return self.DRW(x, y, instruction & 0x000F)
        elif instruction & 0xF000 == 0xE000:
            if instruction & 0x00FF == 0x009E:
                return self.SKP(x)
            elif instruction & 0x00FF == 0x00A1:
                return self.SKNP(x)
        elif instruction & 0xF000 == 0xF000:
            if instruction & 0x00FF == 0x0007:
                return self.LD_Vx_DT(x)
            elif instruction & 0x00FF == 0x000A:
                return self.LD_Vx_K(x)
            elif instruction & 0x00FF == 0x0015:
                return self.LD_DT(x)
            elif instruction & 0x00FF == 0x0018:
                return self.LD_ST(x)
            elif instruction & 0x00FF == 0x001E:
                return self.ADD_I(x)
            elif instruction & 0x00FF == 0x0029:
                return self.LD_F(x)
            elif instruction & 0x00FF == 0x0033:
                return self.LD_B(x)
            elif instruction & 0x00FF == 0x0055:
                return self.LD_I_Vx(x)
            elif instruction & 0x00FF == 0x0065:
                return self.LD_Vx_I(x)

        print(f"Invalid Instruction: {hex(instruction)} at pc: {self.program_counter}")
        self.PAUSE()
    
    # Close the program.
    def PAUSE(self):
        self.is_paused = True

    # Clear the display.
    def CLS(self):
        self.display.clear()

    # Return from a subroutine.
    def RET(self):
        self.program_counter = self.stack.pop()

    # Jump to address.
    def JP(self, address: int):
        self.program_counter = address
    
    # Call subroutine at address.
    def CALL(self, address: int):
        self.stack.push(self.program_counter)
        self.program_counter = address
    
    # Skip next instruction if Vx == byte.
    def SE_byte(self, x: int, byte: int):
        if self.memory.V(x) == byte:
            self.program_counter += 2
    
    # Skip next instruction if Vx != byte.
    def SNE_byte(self, x: int, byte: int):
        if self.memory.V(x) != byte:
            self.program_counter += 2
    
    # Skip next instruction if Vx == Vy.
    def SE(self, x: int, y: int):
        if self.memory.V(x) == self.memory.V(y):
            self.program_counter += 2
    
    # Set byte to Vx.
    def LD_byte(self, x: int, byte: int):
        self.memory.set_V(x, byte)
    
    # Add byte to Vx.
    def ADD_byte(self, x: int, byte: int):
        sum = self.memory.V(x) + byte
        self.memory.set_VF(sum > 255)
        self.memory.set_V(x, sum & 0x00FF)
    
    # Set Vy to Vx.
    def LD(self, x: int, y: int):
        self.memory.set_V(x, self.memory.V(x) + self.memory.V(y))

    # Or Vy with Vx.
    def OR(self, x: int, y: int):
        self.memory.set_V(x, self.memory.V(x) | self.memory.V(y))
    
    # And Vy with Vx.
    def AND(self, x: int, y: int):
        self.memory.set_V(x, self.memory.V(x) & self.memory.V(y))
    
    # XOR Vy with Vx.
    def XOR(self, x: int, y: int):
        self.memory.set_V(x, self.memory.V(x) ^ self.memory.V(y))
    
    # Add Vy to Vx.
    def ADD(self, x: int, y: int):
        sum = self.memory.V(x) + self.memory.V(y)

        self.memory.set_VF(sum > 255)
        
        self.memory.set_V(x, sum & 0x00FF)
    
    # Subtract Vy from Vx.
    def SUB(self, x: int, y: int):
        self.memory.set_VF(self.memory.V(x) > self.memory.V(y))
        
        # TODO: Check to see if this works if negative
        self.memory.set_V(x, (self.memory.V(x) - self.memory.V(y)) & 0x00FF)
    
    # Shift Vx right by 1.
    def SHR(self, x: int):
        self.memory.set_VF(self.memory.V(x) & 0b00000001 == 1)

        self.memory.set_V(x, self.memory.V(x) >> 1)
    
    # Subtract Vx from Vy
    def SUBN(self, x: int, y: int):
        self.memory.set_VF(self.memory.V(y) > self.memory.V(x))
        
        # TODO: Check to see if this works if negative
        self.memory.set_V(x, (self.memory.V(y) - self.memory.V(x)) & 0x00FF)

    # Shift Vx left by 1.
    def SHL(self, x: int):
        self.memory.set_VF(self.memory.V(x) & 0b10000000 == 0b10000000)

        self.memory.set_V(x, self.memory.V(x) << 1)
    
    # Skip next instruction if Vx != Vy.
    def SNE(self, x: int, y: int):
        if self.memory.V(x) != self.memory.V(y):
            self.program_counter += 2
    
    # Set I to address.
    def LD_I(self, address: int):
        self.memory.I = address
    
    # Jump to location address + V0.
    def JP_V0(self, address: int):
        self.JP(address + self.memory.V(0x0))
    
    # Set Vx to (random number [0, 255] anded with byte).
    def RND(self, x: int, byte: int):
        self.memory.set_V(x, randint(0, 255) & byte)
    
    # Display n-byte sprite starting at memory location I at (Vx, Vy).
    def DRW(self, x: int, y: int, n: int):
        sprite: List[int] = []

        for i in range(n):
            sprite.append(self.memory.read_ram(self.memory.I + i))

        collision: bool = self.display.draw_sprite(self.memory.V(x), self.memory.V(y), sprite)
        self.memory.set_VF(collision)

    # Skip next instruction if key of Vx is pressed.
    def SKP(self, x: int):
        print("TODO: SKP - Keyboard")

    # Skip next instruction if key of Vx is not pressed.
    def SKNP(self, x: int):
        print("TODO: SKNP - Keyboard")

    # Set Vx to delay timer.
    def LD_Vx_DT(self, x: int):
        self.memory.set_V(x, self.delay_timer)
    
    # Wait for a key press, store the value of the key in Vx.
    def LD_Vx_K(self, x: int):
        print("TODO: LD_Vx_K - Keyboard")

    # Set delay timer = Vx.
    def LD_DT(self, x: int):
        self.delay_timer = self.memory.V(x)
    
    # Set sound timer = Vx.
    def LD_ST(self, x: int):
        self.sound_timer = self.memory.V(x)
    
    # Add Vx to I.
    def ADD_I(self, x: int):
        self.memory.I += self.memory.V(x)
    
    # Set I to the location of the sprite for digit Vx.
    def LD_F(self, x: int):
        self.memory.I = self.memory.V(x) * 5 # Each sprite is 5 bytes long.
    
    # Set memory locations I, I+1, and I+2 to BCD representation of Vx.
    def LD_B(self, x: int):
        value = self.memory.V(x)
        hundreds, value = divmod(value, 100)
        tens, value = divmod(value, 10)
        ones, value = divmod(value, 1)

        self.memory.write_ram(self.memory.I + 0, hundreds)
        self.memory.write_ram(self.memory.I + 1, tens)
        self.memory.write_ram(self.memory.I + 2, ones)
    
    # Store registers v0 through Vx in memory starting at location I.
    def LD_I_Vx(self, x: int):
        for i in range(x + 1):
            self.memory.write_ram(self.memory.I + i, self.memory.V(i))
    
    # Read registers V0 through Vx from memory starting at location I.
    def LD_Vx_I(self, x: int):
        for i in range(x + 1):
            self.memory.set_V(i, self.memory.read_ram(self.memory.I + i))
