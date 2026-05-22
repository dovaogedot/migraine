# Migraine

Migraine is a tool that restores pixel art from upscaled and/or compressed state into its original state preserving size and colors as close as possible.

## Introduction

My girlfriend loves crocheting. And she loves to crochet pixel art. Unfortunately, most pixel art on the internet is not "pixel-perfect" in a sence that it's upscaled and pixel edges are a blurry mess. But each column in crochet has to correspond to a single pixel with defined color.

Finally, I got tired of counting pixel art resolution, resizing, cropping, posterizing, fixing colors and so on. So decided to create a simple CLI tool that will do all that work for me with a single instruction.

Initial version was written in Scala, but the startup time was greater than the image processing part... So I switched to Rust and got almost 20x boost in execution speed.

## Usage

Binary has only one required argument - path to the image.

```sh
migraine ./image.jpg
```

This will try its best of inferring all aspects of the image and write restored pixel art into `./image.jpg_downsampled.bmp`.

You can also provide optional arguments if you know exact dimensions or number of colors in original pixel art. All of the options can be viewed by running:

```sh
migraine -h
```

## Examples
|||
|-|-|
|![Before](./samples/angel_200x200_5.4.webp)|![After](./samples/angel_200x200_5.4.webp_downsampled.bmp)|
|![Before](./samples/sailor_160x144_4.png)|![After](./samples/sailor_160x144_4.png_downsampled.bmp)|
|![Before](./samples/skull_167x174_6.67.png)|![After](./samples/skull_167x174_6.67.png_downsampled.bmp)|
|![Before](./samples/sunset_252x142_7.62.jpg)|![After](./samples/sunset_252x142_7.62.jpg_downsampled.bmp)|

## References

- [AMDF (Average Magnitude Difference Function)](http://notedetection.weebly.com/amdf.html)
- [YIN, a fundamental frequency estimator](http://audition.ens.fr/adc/pdf/2002_JASA_YIN.pdf)
