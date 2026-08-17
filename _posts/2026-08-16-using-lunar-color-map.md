---
layout: post
title: "Rendering the Moon in Colors"
date: 2026-08-16
---

In the [last session][last-post], we improved our rendering pipeline so that we can now access the surface position in model space from the pixel shader. The next step is to use that data to pick pixel colors from the [lunar color map][cgi-moon-kit]. Finally, we'll be able to see the Moon in all its beauty!

[Version 0.3 on GitHub][version-0-3]{: .no-github-icon}

## What you will see

And there you go! Now we have a rotating Moon. Of course, it's not exactly how you'd see it from Earth — we don't see it rotate, and the Sun's position isn't realistic — but as a showcase, it looks quite impressive.

<div style="text-align: center;">
<video src="https://github.com/tindandelion/mare-imbrium/releases/download/0.3.3/lunar-globe.webm" alt="Animated lunar globe" autoplay loop muted playsinline
   class="animation"></video>
</div>

## How we got here

So now, in the pixel shader, we have a position on the surface in model coordinate space. What's left is to convert those coordinates from a Cartesian $(x, y, z)$ triple into geographic $(lat, lon)$ and pick the corresponding _texel_ (a fancy name for _texture pixel_)from the lunar color map. Conceptually, this is a pretty straightforward task. In practice, though, there are some hidden challenges to address, so let's do it step by step.

To check the intermediate steps and make sure we're on the right path, we've created a much simpler texture to use as a guinea pig before applying the real one:

<div style="padding: 1em; margin: 1em; background-color: grey;">
    <img src="https://github.com/tindandelion/mare-imbrium/raw/364e36b471e4c8e0fb6895b7cccba6df4b21d01e/assets/test-texture.png" alt="My test texture" width="50%" style="image-rendering: pixelated;">
</div>

With this texture, it's much easier to see that the mapping works correctly:

* the red cross clearly marks the center of the texture;
* the vertical stripes at `x == 0` and `x == width` show the texture boundaries when applied to the sphere.

## The mathematics

First, let's move away from "lat/lon" notation toward more abstract values; it's easier to reason about the math if we don't have to keep track of "north/south" and "east/west". Mathematically, we're dealing with a [spherical coordinate system][spherical-coordinates] on a unit sphere, and we're interested in two angles:

* the _polar angle_ $\theta$ with respect to the Y-axis (the "up" direction in our coordinate system), with domain $\theta \in [0, \pi]$;
* the _azimuthal angle_ $\varphi$ in the XZ-plane, with domain $\varphi \in [0, 2\pi)$.

We also assume that our texture maps $\varphi$ horizontally from left to right and $\theta$ vertically from top to bottom, with the origin at the top-left corner.

The formulas for converting from Cartesian to spherical coordinates are [well known][cartesian-to-spherical], although there are a few practical subtleties. In particular, the derivation of $\varphi$ involves $\arctan(\frac{y}{x})$ with different conditions depending on the quadrant. The good news is that we don't need to implement this piecewise definition from scratch. The conversion is so common that most programming environments implement [`atan2(y, x)`][wikipedia-atan2] function, and Rust is [no exception][rust-doc-atan2] in that regard. We'll have to wrap our heads around its results, though.

## Trying things out

Enough theory — let's implement the conversion formulas from the Wikipedia article and see what we get. In our coordinate system, where Y is the polar axis, we have:

$$
\begin{aligned}
\theta &= \arccos(y) \\
\varphi &= \mathrm{atan2}(z, x) + \pi
\end{aligned}\tag{1}
$$
{: #formula-1}

Notice that `atan2` returns a value in $[-\pi, \pi]$, so we add $\pi$ to shift it into $[0, 2\pi]$.

Plugging these formulas into the code, we get the following picture when we look at the sphere from the north pole:

<div class="still-compare">
    <figure style="max-width: 90%">
        <img src="{{site.baseurl}}/assets/images/2026-08-16-using-lunar-color-map/first-try.webp" alt="Our first try at applying the color map" />
        <figcaption>Our first try at applying the color map</figcaption>
    </figure>
</div>

Surprisingly, it almost works on the first try! We're nearly there, except that our texture is oriented rather awkwardly, as if we were looking at the Moon from the left. That's not right: we'd like to see the Moon's face.

One way to correct that is to use a world-space transform. We could leave the texture mapping as it is and simply rotate the sphere by $\frac{\pi}{2}$ around the Y-axis to see the Moon's face, but it's inconvenient to remember to do that every time.

We can do a bit better: let's make the Moon's face visible without any extra transforms on the underlying mesh. We get the same effect by rotating the model-space coordinates by $-\frac{\pi}{2}$ (notice the negative angle!) and then converting them to spherical coordinates. Let's try it out:

<div class="still-compare">
    <figure style="max-width: 90%">
        <img src="{{site.baseurl}}/assets/images/2026-08-16-using-lunar-color-map/with-rotation-applied.webp" alt="Apply rotation transform" />
        <figcaption>Color map after applying rotation</figcaption>
    </figure>
</div>

That's it! Now the Moon should be facing us by default.

There's one thing we can do to simplify the calculations, though. A rotation by $-\frac{\pi}{2}$ can be done without matrix multiplication, because a right-angle rotation matrix reduces to rearranging vector components:

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

Combining that with [formula (1)](#formula-1), we arrive at the final formulas for converting model coordinates into spherical texture coordinates without any intermediate calculations:

$$
\boxed{\begin{aligned}
\theta &= \arccos(y) \\
\varphi &= \mathrm{atan2}(x, -z) + \pi
\end{aligned}}
$$

## Using the real color map

After all that effort, let's replace our test texture with the actual lunar color map from NASA's CGI Moon Kit and marvel at the result. Isn't it beautiful?

<div class="still-compare">
    <figure style="max-width: 90%">
        <img src="{{site.baseurl}}/assets/images/2026-08-16-using-lunar-color-map/moon-color-map.webp" alt="Moon globe with real color map" />
        <figcaption>Moon globe with real color map</figcaption>
    </figure>
</div>

[last-post]: {{site.baseurl}}/{% post_url 2026-08-14-getting-ready-for-texture-mapping %}
[version-0-3]: https://github.com/tindandelion/mare-imbrium/tree/0.3.4
[cgi-moon-kit]: https://svs.gsfc.nasa.gov/4720
[spherical-coordinates]: https://en.wikipedia.org/wiki/Spherical_coordinate_system
[cartesian-to-spherical]: https://en.wikipedia.org/wiki/Spherical_coordinate_system#Cartesian_coordinates
[wikipedia-atan2]: https://en.wikipedia.org/wiki/Atan2
[rust-doc-atan2]: https://doc.rust-lang.org/std/primitive.f32.html#method.atan2
