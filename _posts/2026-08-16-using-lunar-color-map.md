---
layout: post
title: "Rendering the Moon in Colors"
date: 2026-08-16
---

In the [last session][last-post] we improved our rendering pipeline such that now we can access the model surface position in the pixel shader. The next step is to use that data to pick pixel colors from the [lunar color map](https://svs.gsfc.nasa.gov/4720). Finally, we'll be able to see the Moon in all its beauty! 

## What you will see

And there you go! Now we have a rotating Moon. Of course it's not how you'd see it from the Earth — we don't see the rotation, and the Sun position is off — but as a showcase it looks quite impressive. 

<div style="text-align: center;">
<video src="https://github.com/tindandelion/mare-imbrium/releases/download/0.3.3/lunar-globe.webm" alt="Animated lunar globe" autoplay loop muted playsinline
   class="animation"></video>
</div>

[last-post]: {{site.baseurl}}/{% post_url 2026-08-14-getting-ready-for-texture-mapping %}