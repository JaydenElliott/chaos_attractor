# Chaotic Attractor Simulation

The default implementation uses 2000 particles. This can be changed in src/particles.rs by setting the N_PARTICLES constnat. Running the application in release mode significantly helps performance.

<br>

```rust
cargo run --release
```

<br>

## Interactivity

Holding the spacebar adds particles to the screen. The visuals for this are much better when setting N_PARTICLES to 0.0.
