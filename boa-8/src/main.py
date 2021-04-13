from cpu import Cpu
from display import Display
from keyboard import Keyboard
from memory import Memory
import pygame


FPS = 60


def main():
    memory = Memory()
    # memory.load_rom("../roms/test/BC_test.ch8")         # Passed
    # memory.load_rom("../roms/test/corax89_test.ch8")    # Failed
    # memory.load_rom("../roms/test/metteo_test.ch8")     # Passed

    # memory.load_rom("../roms/BLINKY")    # OLD
    # memory.load_rom("../roms/INVADERS")  # INVALID INSTRUCTION 0x9292

    # memory.load_rom("../roms/15PUZZLE")  # BROKEN

    # memory.load_rom("../roms/BLITZ")     # KEYBOARD
    # memory.load_rom("../roms/CONNECT4")  # KEYBOARD
    # memory.load_rom("../roms/VERS")      # KEYBOARD
    # memory.load_rom("../roms/WIPEOFF")   # KEYBOARD
    # memory.load_rom("../roms/MISSILE")   # KEYBOARD

    # memory.load_rom("../roms/KALEID")    # SUB-ROUTINE
    # memory.load_rom("../roms/MERLIN")    # SUB-ROUTINE
    # memory.load_rom("../roms/PONG")      # SUB-ROUTINE
    # memory.load_rom("../roms/PONG2")     # SUB-ROUTINE
    # memory.load_rom("../roms/PUZZLE")    # SUB-ROUTINE
    # memory.load_rom("../roms/SYZYGY")    # SUB-ROUTINE
    # memory.load_rom("../roms/TANK")      # SUB-ROUTINE
    # memory.load_rom("../roms/TETRIS")    # SUB-ROUTINE
    # memory.load_rom("../roms/TICTAC")    # SUB-ROUTINE
    # memory.load_rom("../roms/UFO")       # SUB-ROUTINE
    # memory.load_rom("../roms/VBRIX")     # SUB-ROUTINE
    # memory.load_rom("../roms/GUESS")     # SUB-ROUTINE

    # memory.load_rom("../roms/BRIX")      # WORKING
    # memory.load_rom("../roms/MAZE")      # WORKING

    display = Display()

    keyboard = Keyboard()

    cpu = Cpu(memory, display, keyboard)

    clock = pygame.time.Clock()

    while display.is_running:
        clock.tick(FPS)

        cpu.cycle()
    

if __name__ == "__main__":
    main()
