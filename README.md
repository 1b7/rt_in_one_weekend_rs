# Ray Tracing in One Weekend, in Rust.

A straightforward Rust-based reimplementation of the basic ray tracer shown in the book [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

In addition, this version implements:
- Its own bitmap export functionality (see [bitmap.rs](/src/bitmap.rs)), 
- Parallelised rendering on the CPU (courtesy of [`rayon`](https://crates.io/crates/rayon)),
- Basic triangle rendering.

## Resources Used
- Intitial inspiration and code: Peter Shirley's [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html),
- Ray-Triangle Intersection explanation and formulae: [scratchapixel.com](https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/ray-triangle-intersection-geometric-solution.html)

## Renders
![Random Spheres](random_scene.bmp)

![Cornell Box](cornellesque.bmp)