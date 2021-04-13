import pygame
from typing import List


class Keyboard:
    def __init__(self):
        """
        The computers originally running the CHIP-8 had a 16-key hexadecimal
        keypad with the following layout:
        1 2 3 C
        4 5 6 D
        7 8 9 E
        A 0 B F
        """
        self.keys: List[bool] = [False] * 16

    def process_keys(self):
        keys = pygame.key.get_pressed()

        """
        1 2 3 4
        Q W E R
        A S D F
        Z X C V
        """
        self.keys = [
            keys[pygame.K_x],
            keys[pygame.K_1],
            keys[pygame.K_2],
            keys[pygame.K_3],
            keys[pygame.K_q],
            keys[pygame.K_w],
            keys[pygame.K_e],
            keys[pygame.K_a],
            keys[pygame.K_s],
            keys[pygame.K_d],
            keys[pygame.K_z],
            keys[pygame.K_c],
            keys[pygame.K_4],
            keys[pygame.K_r],
            keys[pygame.K_f],
            keys[pygame.K_v],
        ]
