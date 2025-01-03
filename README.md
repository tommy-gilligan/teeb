# Teeb: The Terminal Keeb

Forked from [System 76's Launch keyboard](https://github.com/system76/launch) ([OSHWA UID US001062](https://certification.oshwa.org/us001062.html)) to lean on a strong, production-ready keyboard design that is also open source.

- [Mechanical Design](#mechanical-design)
- [Electrical Design](#electrical-design)
- [Firmware and Software](#firmware-and-software)

## Raison D'être

This is a hobby project motivated by the bizarre idea of combining:

- typewriter
- keyboard
- terminal

Personally, I've never been great with pen-and-paper.  And touchscreens remain cumbersome for long-form text.  Inputting text by dictation is also kinda weird IMHO.  So let's bring back the typewriter.  Except let's update it for the 21st century.  Of course, that means it should be exceedingly portable.  If it's a device I'm going to be taking everywhere with me, then why not make sure it is capable of functioning as a keyboard as well.  And if it has a display, why not make it capable of functioning as a dumb terminal?  (Scope creep much?)  What I'm ending up with is a peculiar love-letter to an (imagined?) simpler past.  This is definitely a device that I intend on using regularly that will be functional (and simple) but I may as well ham up the campiness of its nostalgic aspects.

## Mechanical Design

### Open Source Chassis

The Teeb chassis is licensed CC-BY-SA-4.0 and can be viewed in the
[chassis](./chassis/) folder using [FreeCAD](https://www.freecadweb.org/).

### Milled Aluminum

The chassis is milled from two solid blocks of aluminum and powder coated to
provide excellent fit and finish. Each pocket, port, and hole is designed and
precisely machined so that swapping switches and plugging in cables is easy and
secure for the user.

### Detachable Lift Bar

The included lift bar can be magnetically secured to add 15 degrees of angle to
your keyboard for ergonomics.

### Swappable Keycaps

The keycaps are PBT material with a dye sublimation legend and XDA profile to
provide excellent feel and lifespan. Extras are provided for common replacements
and color preference. An included keycap puller can be used to move and replace
the keycaps.

### Swappable Switches

The switches are mounted in sockets that support any RGB switch with an MX
compatible footprint. Examples are the Cherry MX RGB switches and the Kailh
BOX switches. Switches can be removed easily at any time with the included
switch puller.

## Electrical Design

### Open Source PCB

The Teeb PCB is licensed GPLv3 and can be viewed in the [pcb](./pcb/) folder using [KiCad](https://kicad.org/).

### N-Key Rollover

The keyboard matrix uses diodes on all intersections, providing full independent scanning of each key position.

### Teeb Prototype Status

Instead of using on-board RP235xB, right now targeting 2 separate Pico 2.  Upon transitioning to RP235xB, much more space should be freed up on PCB.  Could add back some keys that I've deleted and/or make PCB fit chassis better.  Display driver should be on-board but it isn't right now.  Same goes for bell, RS232 connector and extra FRAM (could be for redundancy, could be for high-trust separation of data).

## Firmware and Software

Existing QMK firmware used by Launch has been thrown out.  In its place, some new Embassy based firmware targeting two separate RP235xA:

- [a top-side 'application'](./firmware/examples/rp23/src/bin/top.rs)
- [a bottom-side matrix scanner](./firmware/examples/rp23/src/bin/bottom.rs)

Firmware spread across multiple RP235xA due to limited RP235xB availaibility (I need the extra GPIO).  Ultimately these firmware will run on the same MCU someday but the separation of firmware here seems prudent from a software architecture stand-point.  The bottom matrix scanner sends keyboard events (using HID encoding) via UART to top-side application.  Responsibilities of top-side application include:

- updating EPD display
- signalling bell
- persisting everything to FRAM and/or SD card
- connection to external devices: USB keyboard, UART, RS232

There is also a separate crate for [the terminal](./terminal/).  This is not yet being used by the firmware.  An SDL2 example application is included to assist with testing/prototyping but the intent of this crate is to only provide building-blocks for a no_std terminal.  It is *not* intended to be a fully-functionaly virtual terminal targeting desktop OS.

The inclusion of FRAM is core to typewriter functionality but runs counter to terminal functionality: passwords are a thing.  I considered adding persistence-suppresion mode but even with clear visual indicators I think it's too easy to forget to switch suppression on or off.  At the risk of creeping scope further, it will probably be necessary to include a password manager at some point (and to force its use).
