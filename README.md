# gba-dvd
GBA program that is just the DVD bounce screen saver
## Issues
Currently only compiles with opt-level>=1 for debug builds. This is most likely do to a IRQ stack overflow, which is fixed by opt-level 1