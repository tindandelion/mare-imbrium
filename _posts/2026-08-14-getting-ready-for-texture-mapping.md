---
layout: post
title: "Getting Ready for Texture Mapping"
date: 2026-08-14
---

Now we're getting ready to make use of the lunar color maps provided by [CGI Moon Kit][cgi-moon-kit]. But we're not there yet; we first need to wrap our heads around the different coordinate spaces we're working in, and update our pipeline to provide additional data to the pixel shader: the surface position in the _model coordinate space_.

[Version 0.2 on GitHub][version-0-2]{: .no-github-icon}

## What you will see

Instead of a dull grayscale sphere, you will see a smoothly colored surface. Still not the Moon per se, but this render showcases an important milestone: in the pixel shader, we now have access to the interpolated surface position in **model coordinate space**.

This ability is crucial for using the lunar color map later on. But that's coming up later: for the time being, we just derive the pixel color from the position coordinates: `Vec3(x, y, z) -> Color(r, g, b)`. That gives us visual confirmation that the changes we've made work correctly.

<div style="text-align: center;">
<video src="https://github.com/tindandelion/mare-imbrium/releases/download/0.2.1/lunar-globe.webm" alt="Animated lunar globe" autoplay loop muted playsinline
   class="animation"></video>
</div>

There are two important things to notice in this admittedly synthetic picture:

* **The color transitions are smooth.** That indicates that we interpolate the surface position correctly.
* **The colors change in the animation.** The colors in this animation change because we rotate the sphere around the Y-axis. So what we see, essentially, is that the sphere's surface is moving from right to left before our eyes. To give us a visual clue about the rotation, there's a bright blue line that goes through the `x == 0.0` plane.

## What we focus on

Let's first get clear about the problem we're trying to solve.

On the one hand, we have [a pipeline to render a shaded sphere][prev-post], where we interpolate the surface coordinates and normals to calculate the illumination of each rendered pixel.

On the other hand, [CGI Moon Kit][cgi-moon-kit] provides us with lunar color maps in various resolutions as [lat/lon projections][equirectangular]:

<a href="https://svs.gsfc.nasa.gov/vis/a000000/a004700/a004720/lroc_color_2k.jpg">
   <img src="https://svs.gsfc.nasa.gov/vis/a000000/a004700/a004720/lroc_color_2k.jpg" alt="Lunar color map" style="width: 90%; padding-top: 1em;">
</a>

Until now, we just used the same base color for each pixel and simply shaded it according to the lighting model formulas. But having the color map at hand, we can now **choose a different base color** for every pixel we render! To make it work, however, the shader has to know the position on the sphere's surface when it renders the pixel.

To be more precise, we need to know the geographic coordinates $(lat, lon)$ to pick the color from the map, but the Cartesian coordinates work just as well: we can convert between these two coordinate systems using well-known formulas.

In a nutshell, the algorithm for gluing the color map to the sphere looks like this:

1. For each screen pixel, determine its coordinates $P_m = (x, y, z)$ in _model coordinate space_;
2. Convert those coordinates from Cartesian to geographic $(lat, lon)$ and pick the corresponding pixel from the color map;
3. Use that pixel's color as the base color and apply shading based on the surface normal in _world coordinate space_.

All that can be done in the pixel shader, as long as we have the following input data for each pixel:

* Position coordinates in model coordinate space $P_m$ — to pick the color from the color map;
* Normal coordinates in world coordinate space $N_w$ — to calculate shading.

For now, our focus is on providing the values of $P_m$ to the pixel shader. We'll deal with the color map in the follow-up sessions.

[prev-post]: {{site.baseurl}}/{% post_url 2026-08-08-starting-up %}
[version-0-2]: https://github.com/tindandelion/mare-imbrium/tree/0.2.1
[cgi-moon-kit]: https://svs.gsfc.nasa.gov/4720
[equirectangular]: https://en.wikipedia.org/wiki/Equirectangular_projection
