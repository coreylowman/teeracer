# teeracer

Path tracing renderer in rust.

Examples with 1000 samples per pixel:

`cargo run --release --example spheres`:

![spheres.png](spheres.png)

`cargo run --release --example mirror-prism`:

![mirror-prism.png](mirror-prism.png)

`cargo run --release --example glass-prism`:

![glass-prism.png](glass-prism.png)

# todos:
- bidirectional
- importance sample lights
- shoot extra ray at light when sampling scatter direction, use that direction if hits light, otherwise random direction
