import pygame
import random
import math

pygame.init()

WIDTH = 640
HEIGHT = 640
SIZE = pygame.Vector2(WIDTH, HEIGHT)

screen = pygame.display.set_mode(SIZE)
clock = pygame.time.Clock()

# ---------------------------
# Initialize global variables

circle_x = 200
circle_y = 200

# ---------------------------

running = True
dt = 0.01

frame_i = 0
f = open("simulations/50k.txt", 'r')
file_lines = f.readlines()
f.close()

n_lines = len(file_lines)

def glob_to_screen(pos):

    norm_coord = (pos + pygame.Vector2(1, 1)) / 2

    return pygame.Vector2(norm_coord.x * SIZE.x, (1 - norm_coord.y) * SIZE.y)

def screen_to_glob(pos):
    norm_coord = pygame.Vector2(pos.x / SIZE.x, -pos.y / SIZE.y) 

    return (norm_coord - pygame.Vector2(0.5, 0.5)) * 2

def can_cull(pos):
    return pos.x < -1 or pos.x > 1 or pos.y < -1 or pos.y > 1
        

is_paused = False

random_colors = [(random.randint(0, 255),random.randint(0, 255), random.randint(0, 255)) for i in range(20000)]
color_track = 0

zoom_val = 0.0
true_zoom_val = 1.0

camera_offset = pygame.Vector2(0.0, 0.0) 
camera_move_speed = 0.1

mouse_is_down = False
thing_pressed = False
space_pressed = False
just_scrolled = False
time_step_pressed = False
s_button_pressed = False

n_display_modes = 3
display_mode = 0

marked_pos = pygame.Vector2(0, 0)
temp_camera_pos = pygame.Vector2(0, 0)

circle_radius = 0.0001

default_size = max(WIDTH * circle_radius * 0.5, 1.0)


while running:
    keys = pygame.key.get_pressed()
    mouse_pos = screen_to_glob(pygame.Vector2(pygame.mouse.get_pos()))


    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

        if event.type == pygame.MOUSEWHEEL:
            # Accumulate the vertical scroll value
            zoom_val += event.y * 0.1


    screen.fill((0, 0, 0))    

    if pygame.mouse.get_pressed()[0]:

        if thing_pressed:
            mouse_is_down = True
            temp_camera_pos = camera_offset
            marked_pos = mouse_pos
        else:
            mouse_is_down = False

        camera_offset = temp_camera_pos + (marked_pos - mouse_pos) / true_zoom_val

        thing_pressed = False

    else:
        thing_pressed = True

    if keys[pygame.K_LSHIFT] or keys[pygame.K_RSHIFT]:
        time_step_pressed = True
   
    if keys[pygame.K_LEFT]:
        if time_step_pressed:
            frame_i -= 1
            frame_i %= n_lines
        time_step_pressed = False

    elif keys[pygame.K_RIGHT]:
        if time_step_pressed:
            frame_i += 1
            frame_i %= n_lines
        time_step_pressed = False
    else:
        time_step_pressed = True

    if keys[pygame.K_p]:
        frame_i += 10
    if keys[pygame.K_o]:
        frame_i -= 10

    if keys[pygame.K_r]:
        frame_i = 0

    if keys[pygame.K_SPACE]:
        if space_pressed:
            is_paused = not is_paused
        space_pressed = False
    else:
        space_pressed = True

    if keys[pygame.K_q]:
        if s_button_pressed:
            display_mode += 1
            display_mode %= n_display_modes
        s_button_pressed=  False
    else:
        s_button_pressed = True

    if not is_paused:
        frame_i += 1
    frame_i %= n_lines

    values = file_lines[frame_i].strip().split(",")

    if keys[pygame.K_UP]:
        zoom_val += 0.1

    if keys[pygame.K_DOWN]:
        zoom_val -= 0.1

    if keys[pygame.K_w]:
        camera_offset += pygame.Vector2(0.0, 1.0) * camera_move_speed / true_zoom_val
    if keys[pygame.K_s]:
        camera_offset -= pygame.Vector2(0.0, 1.0) * camera_move_speed / true_zoom_val
    if keys[pygame.K_a]:
        camera_offset -= pygame.Vector2(1.0, 0.0) * camera_move_speed / true_zoom_val
    if keys[pygame.K_d]:
        camera_offset += pygame.Vector2(1.0, 0.0) * camera_move_speed / true_zoom_val

    true_zoom_val = math.exp(zoom_val)

    for counter, value in enumerate(values):
        if value == "":
            continue
        pos = value.split()

        color_draw = (200, 20, 20)

        if display_mode == 1:
            speed_gradient = min(float(pos[2]) * 255 * 10000, 225) + 30
            color_draw = (speed_gradient, ) * 3
        elif display_mode == 2:
            collision_gradient = min(float(pos[3]) / 128 * 255, 225) + 30
            color_draw = (collision_gradient, ) * 3

        # print(color_draw)
        draw_pos = (pygame.Vector2(float(pos[0]), float(pos[1])) - camera_offset) * true_zoom_val
        if can_cull(draw_pos):
            continue
        # if glob_to_screen() 
        draw_size = max(default_size, WIDTH * circle_radius * 0.5 * true_zoom_val)

        if draw_size == 1:
            # print(color_draw, glob_to_screen(draw_pos))
            screen_draw_pos = glob_to_screen(draw_pos)
            screen.set_at((int(screen_draw_pos.x), int(screen_draw_pos.y)), color_draw)

        else:
            pygame.draw.circle(screen, color_draw, glob_to_screen(draw_pos), draw_size)

    line_sz=  0.02
    line_thick = 2
    pygame.draw.line(screen, (200, 200, 200), glob_to_screen(pygame.Vector2(0, -line_sz))- pygame.Vector2(line_thick/2, 0), glob_to_screen(pygame.Vector2(0, line_sz)) - pygame.Vector2(line_thick / 2, 0), line_thick)
    pygame.draw.line(screen, (200, 200, 200), glob_to_screen(pygame.Vector2(-line_sz, 0))- pygame.Vector2(0, line_thick/2), glob_to_screen(pygame.Vector2(line_sz, 0)) - pygame.Vector2(0, line_thick/2), line_thick)

    pygame.display.flip()
    clock.tick(30)


pygame.quit()
