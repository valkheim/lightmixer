# lightmixer

Lightmixer is a toy project allowing you to adjust backlight brightness.

## Usage

```console
Lightmixer -- adjust backlight brightness

Usage: lightmixer [MODE]

Modes:
  tui ........... The default terminal user interface mode.
                  navigate using the HJKL keys
  dummy ......... The console interactive mode.

```

## TUI mode

<img src="./assets/demo.png">

Controls:

* `<j>`, `<k>`: select light controller
* `<h>`, `<l>`: decrease/increase backlight
* `<q>`: quit

## Dummy mode

```console
[00] /sys/class/backlight/intel_backlight -> 2500 / 24242
[01] /sys/class/leds/platform::mute -> 0 / 1
[02] /sys/class/leds/phy0-led -> 0 / 1
[03] /sys/class/leds/tpacpi::power -> 0 / 255
[04] /sys/class/leds/input3::numlock -> 0 / 1
[05] /sys/class/leds/tpacpi::lid_logo_dot -> 1 / 255
[06] /sys/class/leds/tpacpi::standby -> 0 / 255
[07] /sys/class/leds/input3::capslock -> 0 / 1
[08] /sys/class/leds/tpacpi::thinkvantage -> 0 / 255
[09] /sys/class/leds/input3::scrolllock -> 0 / 1
[10] /sys/class/leds/tpacpi::kbd_backlight -> 1 / 2
[11] /sys/class/leds/platform::micmute -> 0 / 1
> 05:0
update controller /sys/class/leds/tpacpi::lid_logo_dot with value 0
```
