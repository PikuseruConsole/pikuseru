import math
import random

# Cart Data

# Graphics

def mode_width():
    return pikuseru_graphic.mode_width()

def mode_height():
    return pikuseru_graphic.mode_height()


def camera(x=-1, y=-1):
    pikuseru_graphic.camera(flr(x), flr(y))


def circ(x, y, r, color=-1):
    pikuseru_graphic.circ(math.floor(x), math.floor(y),
                         math.floor(r), math.floor(color))


def circfill(x, y, r, color=-1):
    pikuseru_graphic.circfill(math.floor(x), math.floor(y),
                             math.floor(r), math.floor(color))


def clip(x=-1, y=-1, w=-1, h=-1):
    pikuseru_graphic.clip(math.floor(x), math.floor(y),
                         math.floor(w), math.floor(h))


def cls(value=-1):
    pikuseru_graphic.cls(value)


def color(col):
    pikuseru_graphic.color(col)


def ellipse(x, y, rx, ry, color=-1):
    pikuseru_graphic.ellipse(math.floor(x), math.floor(y),
                            math.floor(rx), math.floor(ry),
                            math.floor(color))


def ellipsefill(x, y, rx, ry, color=-1):
    pikuseru_graphic.ellipsefill(math.floor(x), math.floor(y),
                                math.floor(rx), math.floor(ry),
                                math.floor(color))


def fget(idx_sprite, flag=-1):
    if flag == -1:
        pikuseru_graphic.fget_all(idx_sprite)
    return pikuseru_graphic.fget(idx_sprite, flag)


def font(name="pico8"):
    pikuseru_graphic.font(name)


def fset(idx_sprite, flag, value=-1):
    if value == -1:
        pikuseru_graphic.fset_all(flag)
    else:
        pikuseru_graphic.fset(idx_sprite, flag, value)

def fillp(pat=0, transparent=False):
    pikuseru_graphic.fillp(pat, transparent)

def line(x1, y1, x2, y2, color=-1):
    pikuseru_graphic.line(math.floor(x1), math.floor(y1),
                         math.floor(x2), math.floor(y2),
                         math.floor(color))


def pal(c0=-1, c1=-1, p=0):
    pikuseru_graphic.pal(math.floor(c0), math.floor(c1))


def palt(c=-1, t=0xFF):
    if type(t) is bool:
        if t:
            t = 0x0
        else:
            t= 0xFF
            
    pikuseru_graphic.palt(math.floor(c), t)


def pget(x, y):
    return pikuseru_graphic.pget(math.floor(x), math.floor(y))


def pset(x, y, color):
    pikuseru_graphic.pset(math.floor(x), math.floor(y), math.floor(color))


def pikuseru_print(str, x=-1, y=-1, col=-1):
    return pikuseru_graphic.print(str, math.floor(x), math.floor(y), math.floor(col))


def rect(x1, y1, x2, y2, color=-1):
    pikuseru_graphic.rect(math.floor(x1), math.floor(y1),
                         math.floor(x2), math.floor(y2),
                         math.floor(color))


def rectfill(x1, y1, x2, y2, color=-1):
    pikuseru_graphic.rectfill(math.floor(x1), math.floor(y1),
                             math.floor(x2), math.floor(y2),
                             math.floor(color))


def sget(x, y):
    return pikuseru_graphic.sget(x, y)


def spr_reg(n, d, width, height):
    return pikuseru_graphic.spr_reg(n, d, width, height)


def spr(n, x, y, w=1, h=1, flip_x=False, flip_y=False, angle=0, zoom=1.0, dynamic=False):
    pikuseru_graphic.spr(math.floor(n), math.floor(x), math.floor(y),
                        math.floor(w), math.floor(h), flip_x, flip_y,
                        angle, zoom, dynamic)


def sset(x, y, c=-1):
    pikuseru_graphic.sset(x, y, c)


def sspr(sx, sy, sw, sh, dx, dy, dw=-1, dh=-1, flip_x=False, flip_y=False):
    if dw == -1:
        dw = sw

    if dh == -1:
        dh = sh

    pikuseru_graphic.sspr(sx, sy, sw, sh, dx, dy, dw, dh, flip_x, flip_y)


def sspr_rotazoom(idx_sprite, sx, sy, sw, sh, dx, dy,
                  angle=0.0, zoom=1.0,
                  flip_x=False, flip_y=False):
    return pikuseru_graphic.sspr_rotazoom(idx_sprite,
                                         sx, sy,
                                         sw, sh,
                                         dx, dy,
                                         angle, zoom,
                                         flip_x, flip_y)


def trigon(x1, y1, x2, y2, x3, y3, color):
    pikuseru_graphic.trigon(math.floor(x1), math.floor(y1),
                           math.floor(x2), math.floor(y2),
                           math.floor(x3), math.floor(y3),
                           color)


def polygon(x, y, color):
    pikuseru_graphic.polygon(x, y, color)


# Palette

def palette(col, r, g, b):
    pikuseru_graphic.set_color_palette(col, r, g, b)


def palette_hexa(col, value):
    r = (value & 0xFF0000) >> 16
    g = (value & 0x00FF00) >> 8
    b = (value & 0x0000FF)
    pikuseru_graphic.set_color_palette(col, r, g, b)


def palette_reset():
    pikuseru_graphic.reset_palette()


def palette_switch(name):
    pikuseru_graphic.switch_palette(name)


globals()["mode_width"] = mode_width
globals()["mode_height"] = mode_height
globals()["palette"] = palette
globals()["palette_hexa"] = palette_hexa
globals()["palette_reset"] = palette_reset
globals()["palette_switch"] = palette_switch

globals()["camera"] = camera
globals()["circ"] = circ
globals()["circfill"] = circfill
globals()["clip"] = clip
globals()["cls"] = cls
globals()["color"] = color
globals()["ellipse"] = ellipse
globals()["ellipsefill"] = ellipsefill
globals()["fget"] = fget
globals()["fset"] = fset
globals()["fillp"] = fillp
globals()["line"] = line
globals()["pal"] = pal
globals()["palt"] = palt
globals()["pset"] = pset
globals()["pget"] = pget
globals()["pikuseru_print"] = pikuseru_print
globals()["rect"] = rect
globals()["rectfill"] = rectfill
globals()["sget"] = sget
globals()["spr"] = spr
globals()["spr_reg"] = spr_reg
globals()["sset"] = sset
globals()["sspr"] = sspr
globals()["sspr_rotazoom"] = sspr_rotazoom
globals()["trigon"] = trigon
globals()["polygon"] = polygon

# Input


def btn(x, p=0):
    return pikuseru_input.btn(x, p)

def btnp(x, p=0):
    return pikuseru_input.btnp(x, p)

def mouse_x():
    return pikuseru_input.btn_mouse(0, 0)

def mouse_y():
    return pikuseru_input.btn_mouse(1, 0)

def mouse_state(p=0):
    return pikuseru_input.btn_mouse_state(p)

def mouse_statep(p=0):
    return pikuseru_input.btn_mouse_statep(p)

def mouse_left_state(p=0):
    return pikuseru_input.btn_mouse_state(p) & 0x000000FF

def mouse_left_statep(p=0):
    return pikuseru_input.btn_mouse_statep(p) & 0x000000FF

globals()["btn"] = btn
globals()["btnp"] = btnp

globals()["mouse_x"] = mouse_x
globals()["mouse_y"] = mouse_y
globals()["mouse_state"] = mouse_state
globals()["mouse_statep"] = mouse_statep
globals()["mouse_left_state"] = mouse_left_state
globals()["mouse_left_statep"] = mouse_left_statep

# Map


def mapdraw(cel_x, cel_y, sx, sy, cel_w, cel_h, layer=0):
    pikuseru_map.mapdraw(cel_x, cel_y, sx, sy, cel_w, cel_h, layer)


def mget(x, y):
    return pikuseru_map.mget(math.floor(x), math.floor(y))


def mset(x, y, v):
    pikuseru_map.mset(math.floor(x), math.floor(y), math.floor(v))


globals()["mapdraw"] = mapdraw
globals()["mget"] = mget
globals()["mset"] = mset

# Audio
def play_note(note, instrument=0, channel=0):
    return pikuseru_audio.play_note(note, instrument, channel)

def trigger_note(note, instrument=0):
    return pikuseru_audio.trigger_note(note, instrument)

def play_phrase(phrase_index, target_bpm=120.0):
    return pikuseru_audio.play_phrase(phrase_index, target_bpm)
    
globals()["play_note"] = play_note
globals()["trigger_note"] = trigger_note
globals()["play_phrase"] = play_phrase

# Info

def pikuseru_time():
    return pikuseru_info.time()

def pikuseru_mtime():
    return pikuseru_info.mtime()

def pikuseru_utime():
    return pikuseru_info.utime()

globals()["pikuseru_time"] = pikuseru_time
globals()["pikuseru_utime"] = pikuseru_utime


# Math


def atan2(x, y):
    v = math.atan2(x,y)
    return (((v - math.pi) / (math.pi * 2)) + 0.25) % 1.0


def cos(x):
    return math.cos((x or 0) * (math.pi * 2))


def sin(x):
    return math.sin(-(x or 0) * (math.pi * 2))


def flr(x):
    return math.floor(x)


def rnd(x):
    return random.random() * x


def srand(x):
    return random.seed(x)


def mid(x,y,z):
    x = x or 0
    y = y or 0
    z = z or 0
    return x > y and x or y > z and z or y


def bxor(a,b):
    return int(a) ^ int(b)


globals()["atan2"] = atan2
globals()["ceil"] = math.ceil
globals()["cos"] = cos
globals()["sin"] = sin
globals()["flr"] = flr
globals()["rnd"] = rnd
globals()["sqrt"] = math.sqrt
globals()["mid"] = mid
globals()["bxor"] = bxor