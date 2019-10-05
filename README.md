<a name="top"></a>
# Porous Absorber Calculator

Calculates the acoustic absorption curve of a variety of porous absorber devices mounted against a rigid backing such as a brick wall.

The porous absorber is typically made from some material such as Rockwool or glass fibre insulation.  You need to know the flow resistivity of this material in order to get the best results from these calculations.



<!--------------------------------------------------------------------------------------------------------------------->
<a name="usage"></a>
## Usage

When the app starts, the "Rigid Backed Porous Absorber" tab will be selected by default.

![Screen shot](./img/rb_porous_absorber_screen.png)

If this is the first time you have run this calculator, then all calculations will be performed using default values.

If you have used this calculator before, then each of the curves will be plotted using your previous values.

As you move the mouse pointer over the chart, cross hairs will appear and the absorption value will be displayed when the mouse hovers over a plot point.


<!--------------------------------------------------------------------------------------------------------------------->
<a name="device-types"></a>
## Absorber Device Types

Four absorber device types have been implemented:

* [Rigid Backed Porous Absorber](./docs/rb_porous_absorber.md)
* [Slotted Panel](./docs/slotted_panel.md)
* [Perforated Panel](./docs/perforated_panel.md)
* [Microperforated Panel](./docs/microperforated_panel.md)

There is also a [configuration](./docs/configuration.md) screen on which you can change less frequently altered values such as air temperature and pressure.


<!--------------------------------------------------------------------------------------------------------------------->
<a name="online-version"></a>
## Online Version

An online version of this tool is available [here](http://whealy.com/acoustics/PA_Calculator/index.html)



<!--------------------------------------------------------------------------------------------------------------------->
<a name="graph"></a>
## Graph

The graph maintains an aspect ratio of 21:9 as the browser screen resizes down to a minimum width of 1000 pixels.

### Smooth Curve

If desired, the "Smooth curve" checkbox can be switched on.  This will connect each plot point using Bézier curves; however, it should be noted that this feature was added for its aesthetic appeal and does ***not*** imply that the actual absorption between the plot points follows the line drawn on the screen


### Start frequency

The graph always plots an 8 octave range starting at the specified start frequency.  Normally, this should be left set to 62.5 Hz in order to see the standard analysis range (i.e. up to 16 KHz).  However, should you wish to, you can set the start frequency to be as low as 20 Hz, in which case, you will still see an 8 octave range, but the upper limit will now be 5.1 KHz

### Input using sliders

All inputs are made using the range sliders.  The sliders can be moved either by dragging the button with the mouse, or for more precise input, select the slider and use the left/right arrow keys.

I decided to use sliders as the input UI element instead of simple input fields for two reasons:

1. It prevents invalid or out of range values from being entered, thus ensuring that the calculation engine always receives valid input
2. It creates an "animation" effect whereby you can see how the absorption curve changes dynamically as you move a slider



<!--------------------------------------------------------------------------------------------------------------------->
<a name="local-storage"></a>
## Local Storage

This app uses the browser's local storage to remember your absorber device parameters.  If at anytime you wish to clear these cached values, select the "Configuration" tab and press the "Clear Cache" button.  Only the values pertaining to this application are cleared from local storage.

![Configuration](./img/configuration_screen.png)

It is possible that after a new version of this app is released, old values in the local storage cache might cause you to see an empty chart.  If this happens, clear the local storage cache and refresh your browser page.



<!--------------------------------------------------------------------------------------------------------------------->
<a name="local-installation"></a>
## Local Installation

These instructions assume you have already installed Rust and `wasm-pack`, and that Python3 is available to act as a Web server.

1. Clone this repo
2. Change into the repo's top-level directory
3. Compile using `wasm-pack build --release --target web`
4. Start a Python3 Web server using `python3 -m http.server`
5. Visit <http://0.0.0.0:8000>



<!--------------------------------------------------------------------------------------------------------------------->
<a name="background"></a>
## Background

This app is the reimplementation of an [Excel spreadsheet](http://whealy.com/acoustics/Porous.html) I wrote in 2004 and is part of an on-going exercise in learning Rust and cross-compiling it to Web Assembly using [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) and [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/introduction.html).



<!--------------------------------------------------------------------------------------------------------------------->
<a name="to-do"></a>
## To Do

Nichts, nada, nowt...



<!--------------------------------------------------------------------------------------------------------------------->
<a name="caveat"></a>
## Caveat

The author has taken every reasonable step to ensure that the calculations are accurate to the equations and methodology documented in the book "*Acoustic Absorbers and Diffusers.  Theory, Design and Practice*" by Trevor Cox and Peter D'Antonio (First Edition).  The author also recognises that this book is now in its third edition and therefore, certain calculations may have been modified or revised; consequently, the graphs plotted by this tool may vary from those plotted by a tool based on the most recent version of this book.



<!--------------------------------------------------------------------------------------------------------------------->
<a name="known-issues"></a>
## Known Issues

For best results, view this app using Google Chrome or Firefox Quantum browsers from a desktop machine.

This app has not been optimized for display on mobile devices.

### iOS Safari Issues

According to [CanIUse](https://caniuse.com/#feat=wasm), WebAssembly should work on Safari for iOS, but I haven't had any success here yet...

### MacOS Safari Useability Issues

In Apple Safari for MacOS, clicking directly on a range slider does not select or highlight it.  This means that you cannot increment or decrement the slider value using the arrow keys simply by clicking on it first.

Range slider values ***can*** be selected by tabbing whilst holding down the Option Key ⌥.  Unfortunately once selected, the range slider will still not be highlighted... DOH!

### Brave Useability Issues

When this app is viewed using the Brave browser, the spacing of the axis ticks amd labels is not correct



<!--------------------------------------------------------------------------------------------------------------------->
<a name="support"></a>
## Support

Support *can* be provided but I cannot guarantee a prompt response...



<!--------------------------------------------------------------------------------------------------------------------->
<a name="contributing"></a>
## Contributing

Chris Whealy  <chris@whealy.com>



<!--------------------------------------------------------------------------------------------------------------------->
<a name="license"></a>
## License

This project is licensed under the Apache Software License, Version 2.0 except as noted otherwise in the [LICENSE](LICENSE) file.


