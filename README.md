![ranbo image](https://raw.githubusercontent.com/joncarr/ranbo/master/extras/ranbo_banner.jpg)
# Ranbo
Ranbo is a simple utility tool that generates palettes for GIMP and Inkscape and automatically imports them into the software.

## Commands
    - ranbo
        -i [path/to/image]
        -c [number of colors (between 4 and 20)]

## Usage
To use, simply call the program and provide a path to the image you wish to pull a color palette from. The tool defaults to pulling 10 colors from an image but can be set to pull up to 20 colors. Pass the `-c` (count) to specify the number of colors you wish to pull. The acceptable range for the count flag is 4-20.

Generate palette with 10 (default) colors:

    `ranbo -i /path/to/image.png`


Generate palette with 16 (default) colors:

    `ranbo -i /path/to/image.png` -c 16


That's it!



Ranbo will generate the color palettes and store them in the appropriate location. If Inkscape or GIMP are ope wile using this tool, be sure to start the software so the software updates the color palettes. Currently this tool is designed to work with LINUX ONLY but may work for Mac as well, I'm not really sure.
