---
layout: post
title: "Rendering the Moon in Colors"
date: 2026-08-16
---

In the [last session][last-post] we improved our rendering pipeline such that now we can access the model surface position in the pixel shader. The next step is to use that data to pick pixel colors from the [lunar color map](https://svs.gsfc.nasa.gov/4720). Finally, we'll be able to see the Moon in all its beauty! 

[Version 0.3 on GitHub][version-0-3]{: .no-github-icon}

## What you will see

And there you go! Now we have a rotating Moon. Of course it's not how you'd see it from the Earth — we don't see the rotation, and the Sun position is off — but as a showcase it looks quite impressive. 

<div style="text-align: center;">
<video src="https://github.com/tindandelion/mare-imbrium/releases/download/0.3.3/lunar-globe.webm" alt="Animated lunar globe" autoplay loop muted playsinline
   class="animation"></video>
</div>

## How we got here 

So now in the pixel shader we have a position at the surface in model coordinate space. What's left is to convert those coordinates from Cartesian $(x, y, z)$ triple into the geographic $(lat, lon)$, and pick the corresponding texel from the lunar color map. Conceptually, this is a pretty straighforward task. However, in practice there are some hodden challenges to address, so let's do it step by step. 

To check the intermediate steps and convince myself that we're on the right path, I've created a much simpler texture that I'm going to be using as a gunea pig before I apply the real texture: 

<div style="padding: 1em; margin: 1em; background-color: grey;">
    <img src="https://github.com/tindandelion/mare-imbrium/raw/364e36b471e4c8e0fb6895b7cccba6df4b21d01e/assets/test-texture.png" alt="My test texture" width="50%" style="image-rendering: pixelated;">
</div>

Using this texture for experiments, it's much easier to see that the mapping works correctly: 

* The red cross clearly marks the center of the texture; 
* The vertical stripes at `x=0` and `x == width` clearly show the texture boundaries, when applied to the shpere. 

## The mathematics 

First, let's move away from lat/lon notation towards more abstract values; it's going to be easier to reason about the math if we don't have to struggle through "north/south" and "east/west". Mathematically, we're dealing with [spherical coordinate system][spherical-coordinates] on a unit sphere, and we are interested in two angles: 

* the _polar angle_ $\theta$ with respect to Y-axis ("up" direction in our coordinate system), with domain $\theta \in [0, \pi]$;
* the _azimuth angle_ $\varphi$ in the XZ-plane, with domain $\varphi \in [0, 2 \pi)$. 

We also assume that our texture maps $\theta$ horizontally left-to-right, and $\varphi$ vertically top-to-bottom, with the origin at the top-left corner. 





[last-post]: {{site.baseurl}}/{% post_url 2026-08-14-getting-ready-for-texture-mapping %}
[version-0-3]: https://github.com/tindandelion/mare-imbrium/tree/0.3.4
[spherical-coordinates]: https://en.wikipedia.org/wiki/Spherical_coordinate_system