# Chaotic Attractor Simulation

The default implementation uses 2000 particles. This can be changed in src/particles.rs by setting the N_PARTICLES constant. Running the application in release mode significantly helps performance.

<br>

```rust
cargo run --release
```

<br>

## Interactivity

Holding the spacebar adds particles to the screen. The visuals for this are much better when setting N_PARTICLES to 0.0.

<br>

## Demo

Wait for ~15secs, it gets pretty cool :D
![](./lorenz_demo.gif)
