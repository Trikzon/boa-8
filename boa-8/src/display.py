import pygame
from typing import List


WIDTH, HEIGHT = 64, 32
DEFAULT_SCALE = 16

BACKGROUND_COLOR = (0, 0, 0)
FOREGROUND_COLOR = (255, 255, 255)


class Display:
    def __init__(self, scale = DEFAULT_SCALE):
        self.display: pygame.Display = pygame.display.set_mode((WIDTH * scale, HEIGHT * scale))
        self.scale = scale

        self.vram: List[bool] = [False] * WIDTH * HEIGHT

        self.is_running: bool = True
    
    def clear(self):
        pygame.draw.rect(self.display, BACKGROUND_COLOR, (0, 0, WIDTH, HEIGHT))
        self.vram: List[bool] = [False] * WIDTH * HEIGHT
    
    def update(self):
        pygame.display.update()

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                self.close()
    
    # Draws a list of bytes onto the screen. Each byte being a row.
    # Returns true if drawing over a sprite.
    def draw_sprite(self, x: int, y: int, sprite: List[int]) -> bool:
        has_collision: bool = False

        for byte in sprite:
            for i in reversed(range(8)):
                x += 1
                bit = (byte >> i) & 0b00000001
                if bit == 1:
                    if self.draw_pixel(x, y):
                        has_collision = True
            y += 1
            x -= 8
        
        return has_collision
                    
        
    # Draws a pixel onto the screen.
    # Returns true if drawing collides.
    def draw_pixel(self, x: int, y: int) -> bool:
        while x >= WIDTH:
            x -= WIDTH
        while y >= HEIGHT:
            y -= HEIGHT

        self.vram[y * WIDTH + x] ^= True

        if self.vram[y * WIDTH + x] == False:  # If pixel collided.
            pygame.draw.rect(
                self.display,
                BACKGROUND_COLOR,
                (x * self.scale, y * self.scale, self.scale, self.scale)
            )
            return True
        else:
            pygame.draw.rect(
                self.display,
                FOREGROUND_COLOR,
                (x * self.scale, y * self.scale, self.scale, self.scale)
            )
            return False

    
    def close(self):
        self.is_running = False
    