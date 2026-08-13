---
layout: post
title: "Setting Up the Scene"
date: 2026-08-08 
---

We start by setting up the basics of the scene we're going to be rendering: the geometry of the lunar globe and the scene illumination. Compared to the [3D rasterizer project][rasterizer-project], the conditions for rendering something resembiling a moon globe are much simpler than what we used to work with. 

[Version 0.1 on GitHub][version-0-1]{: .no-github-icon}

## What you will see

Our goal for the time being is to display the sphere illuminated from one side. With a bit of imagination, we can think of it as the Moon being lit by the distant Sun. 

<div style="text-align: center;">
<video src="https://github.com/tindandelion/mare-imbrium/releases/download/0.1.2/lunar-globe.webm" alt="Animated lunar globe" autoplay loop muted playsinline
  width="800" class="animation"></video>
</div>

## Implementation details 

* The Sun is a [directional light source](https://www.tindandelion.com/rust-3d-rasterizer/2026/05/22/the-cube-gets-light.html#the-light-source-directional-light); 
* Moon surface is matte. We don't need a full-blown Phong lighting model for this project. A simple [Lambertian diffuse model](https://www.tindandelion.com/rust-3d-rasterizer/2026/05/29/the-sphere-gets-smooth.html) is enough. 

[version-0-1]: https://github.com/tindandelion/mare-imbrium/tree/0.1.2
[rasterizer-project]: https://www.tindandelion.com/rust-3d-rasterizer/
