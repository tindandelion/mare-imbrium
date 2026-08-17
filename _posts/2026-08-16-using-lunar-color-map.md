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

The formulas for converting from Cartesian to spherical coordinates are [well known][cartesian-to-spherical], although there's a few practical subtleties to be aware about. In particular, notice that the derivation of $\theta$ involves $\arctan(\frac{y}{x})$ with different conditions. The good news is that we don't need to implement it from scratch. That function is so common in trigonometric operations, that most programming environments implement a function [`atan2(y, x)`][wikipedia-atan2], and Rust is [no exception][rust-doc-atan2]. We'll have to wrap our head around its results, though. 

## Trying things out 

Enough theory, let's get started. Let's just go ahead and implement the conversion formulas from the Wikipedia article, and observe the result. In our coordinate system where Y is a polar axis, we'll get: 

$$
\begin{aligned}
\theta &= \arccos(y) \\
\varphi &= \mathrm{atan2}(z, x) + \pi
\end{aligned}\tag{1}
$$

Notice that `atan2` gives us the result in the range $[-\pi, \pi]$, so we need to add $\pi$ to shift it to $[0, 2\pi]$. 

Plugging these formulas into the code, we get the following picture when we look at the sphere from the north pole:

<div class="still-compare">
    <figure style="max-width: 90%">
        <img src="{{site.baseurl}}/assets/images/2026-08-16-using-lunar-color-map/first-try.webp" alt="Our first try at applying color map" />
        <figcaption>Our first try at applying color map</figcaption>
    </figure>
</div>

Surprisingly, it almost works from the first shot! We're almost there, except that our texture is oriented rather awkwardly, as if we look at the Moon from the left side. In fact, we could leave it as it is, and correct the resulting image by rotating the sphere by $\frac{\pi}{2}$ around Y axis, to see the Moon's face, but it's really not convenient to remember to do that all the time. 

We can do a bit better: let's require that we should see the Moon's face without any additional transforms of the undelying mesh. We can achieve the same effect if we apply rotation **by $-\frac{\pi}{2}$** to model space coordinates before we convert them to spherical. Let's try it out: 

<div class="still-compare">
    <figure style="max-width: 90%">
        <img src="{{site.baseurl}}/assets/images/2026-08-16-using-lunar-color-map/with-rotation-applied.webp" alt="Apply rotation transform" />
        <figcaption>Color map after applying rotation</figcaption>
    </figure>
</div>

That's it! Now the Moon should be facing us by default. 

There's one thing we can do to simplify calculations, though. Remember that rotating by $-\frac{\pi}{2}$ can be done without involving matrix multiplication, because the rotation matrix for right angles is reduced to rearranging vector components: 

$$
\mathbf{R}_y(-\frac{\pi}{2}) \cdot \mathbf{x} =
\begin{pmatrix}
\cos(-\frac{\pi}{2}) & 0 & \sin(-\frac{\pi}{2}) \\
0 & 1 & 0 \\
-\sin(-\frac{\pi}{2}) & 0 & \cos(-\frac{\pi}{2})
\end{pmatrix} \cdot \mathbf{x} = 
\begin{pmatrix}
0 & 0 & -1 \\
0 & 1 & 0 \\
1 & 0 & 0
\end{pmatrix} \begin{bmatrix} x \\ y \\ z \end{bmatrix} = 
\begin{bmatrix} -z \\ y \\ x \end{bmatrix}
$$

Combining that with formula (1), we arrive at the final formula to convert model coordinates into spherical texture coordinates: 

$$
\boxed{\begin{aligned}
\theta &= \arccos(y) \\
\varphi &= \mathrm{atan2}(x, -z) + \pi
\end{aligned}}
$$

## Using the real color map 

After all that effort, let's replace our text texture with the actual Moon color map from the NASA's Moon kit and marvel at the result. Isn't it beautiful? 

<div class="still-compare">
    <figure style="max-width: 90%">
        <img src="{{site.baseurl}}/assets/images/2026-08-16-using-lunar-color-map/moon-color-map.webp" alt="Moon globe with real color map" />
        <figcaption>Moon globe with real color map</figcaption>
    </figure>
</div>














[last-post]: {{site.baseurl}}/{% post_url 2026-08-14-getting-ready-for-texture-mapping %}
[version-0-3]: https://github.com/tindandelion/mare-imbrium/tree/0.3.4
[spherical-coordinates]: https://en.wikipedia.org/wiki/Spherical_coordinate_system
[cartesian-to-spherical]: https://en.wikipedia.org/wiki/Spherical_coordinate_system#Cartesian_coordinates