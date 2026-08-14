---
layout: post
title: "Getting Ready for Texture Mapping"
date: 2026-08-14 
---

Now we're getting ready to implement texture mapping. To do that, we first need to wrap our heads around different coordinate spaces that we're working in, and update our pipeline to provide additional information to the vertex shader. 

[Version 0.2 on GitHub](https://github.com/tindandelion/mare-imbrium/tree/0.2.0){: .no-github-icon}

## What you will see

Now instead of a grayscale sphere, you will see colors on the surface. Still not a moon, but this render showcases an important milestone: access to the position on the sphere surface in _model coordinate space_ for each pixel. This ability is going to be important to map the lunar texture later. For the time being, however, we just derive the pixel color from the position coordinates:  `Vec3(x, y, z) -> Color(r, g, b)`. 

<div style="text-align: center;">
<video src="https://github.com/tindandelion/mare-imbrium/releases/download/0.2.0/lunar-globe.webm" alt="Animated lunar globe" autoplay loop muted playsinline
   class="animation"></video>
</div>

There are two important things to notice from this synthetic picture: 

* **The colors are smooth.** That means that we interpolate the surface position correctly. 
* **The colors are changing in the animation.** It's not obvious from the picture, but this change of colors occurs because we rotate the sphere around the Y axis. So what we see, essentially, that the sphere's surface is moving right to left before our eyes. 


