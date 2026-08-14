---
layout: post
title: "Setting Up the Scene"
date: 2026-08-08 
---

We start by setting up the basics of the scene we're going to be rendering: the geometry of the lunar globe and the scene illumination. The conditions for rendering something resembling a Moon globe are much simpler than what we used to work with in the [3D rasterizer project][rasterizer-project]. 

[Version 0.1 on GitHub][version-0-1]{: .no-github-icon}

## What you will see

Our goal for the time being is to display the sphere illuminated from one side. With a bit of imagination, we can think of it as the Moon lit by the distant Sun. 

<div style="text-align: center;">
<video src="https://github.com/tindandelion/mare-imbrium/releases/download/0.1.2/lunar-globe.webm" alt="Animated lunar globe" autoplay loop muted playsinline
  class="animation"></video>
</div>

## Implementation details 

Compared to what [we've learned to do in the 3D rasterizer][rasterizer-project], the basic scene for the lunar globe is much simpler and does not require all the machinery we implemented back then. 

First, we have a scene illuminated by a single light source: the distant Sun, which can be represented by a [directional light source](https://www.tindandelion.com/rust-3d-rasterizer/2026/05/22/the-cube-gets-light.html#the-light-source-directional-light). 

Second, the Moon's surface is matte, so we don't need a full-blown Phong lighting model for this project. A simple [Lambertian diffuse model](https://www.tindandelion.com/rust-3d-rasterizer/2026/05/29/the-sphere-gets-smooth.html) is enough to render the terminator line.


[version-0-1]: https://github.com/tindandelion/mare-imbrium/tree/0.1.2
[rasterizer-project]: https://www.tindandelion.com/rust-3d-rasterizer/
