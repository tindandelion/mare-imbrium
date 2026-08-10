---
layout: home
title: Welcome
permalink: /
list_title: Project diary
---

Hello and welcome to my new learning project: **Mare Imbrium**. In this project, I'm going to explore new topics in computer graphics programming while building a program that renders lunar landscapes using real-world data from NASA.


## Motivations

Logically, this is a continuation of my previous project, [3D Rasterizer in Rust](https://www.tindandelion.com/rust-3d-rasterizer/). There we built a software 3D rasterizer from scratch. The purpose of that project was to learn the basics of computer graphics: geometry, lighting models, and shading techniques. To showcase the capabilities of the rasterizer, we built a series of programs that visualized different geometric shapes. The project culminated in rendering a tumbling torus, illuminated by several light sources from different angles.

By the end of the project I got a bit bored looking at the torus all the time. A torus is a good shape for learning different 3D graphics techniques, but frankly it started to feel artificial. While still working on that project, I researched new ideas that would let me continue my studies, but would also bring something more interesting and engaging.

Ever since I learned how to [render a sphere under a single light](https://www.tindandelion.com/rust-3d-rasterizer/2026/05/29/the-sphere-gets-smooth.html), one idea kept popping up in my mind. That scene looked like a planet in space! So I decided to take that idea and run with it. A quick web search led me to NASA's website and their [CGI Moon Kit](https://svs.gsfc.nasa.gov/4720) dataset. This dataset is a collection of high-resolution lunar maps designed specifically for use in 3D rendering software. And so, the idea of my next project started to take shape.

In this project, I would like to use my newly acquired skills and NASA's data to render realistic-looking lunar landscapes. Specifically, I have a few ideas for visualizations in mind:

* The first logical step is to render the lunar globe, illuminated by the sun from the side. Using the Moon Kit data, we can render the colors, but in addition we can make use of the elevation maps to bring out more details on the surface: shadows in craters and mountains, as they would be seen through a telescope.

* Next, we can pick a specific site on the lunar surface and display it with finer details. It would be interesting to visualize terrain details and shadows in the animation, as the sun moves across the lunar sky and changes the illumination of the scenery.

* A flight simulation over the lunar surface. Picking a specific trajectory, for example along the equator, we could simulate a view from a spaceship flying over the surface at a low altitude, so we could see all the details of the terrain at its finest resolution.

## Current progress

Our first goal is to render the Moon as a globe.

<div style="text-align: center;">
<video src="https://github.com/tindandelion/mare-imbrium/releases/latest/download/lunar-globe.webm" alt="Lunar globe" autoplay loop muted playsinline
  width="800" class="animation"></video>
</div>

