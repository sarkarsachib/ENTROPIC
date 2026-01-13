//! Noise functions for procedural generation
//!
//! This module provides convenient wrappers around the `noise` crate
//! for terrain and biome generation.

use noise::{NoiseFn, Perlin, Simplex};
use std::sync::Arc;

/// Noise generator using Perlin noise
pub struct PerlinNoise {
    perlin: Arc<Perlin>,
}

impl PerlinNoise {
    /// Create a new Perlin noise generator with default seed
    pub fn new() -> Self {
        Self {
            perlin: Arc::new(Perlin::new(0)),
        }
    }

    /// Create a new Perlin noise generator with a specific seed
    pub fn with_seed(seed: u32) -> Self {
        Self {
            perlin: Arc::new(Perlin::new(seed)),
        }
    }

    /// Get 2D noise value at given coordinates
    pub fn get(&self, x: f64, y: f64) -> f64 {
        self.perlin.get([x, y])
    }

    /// Get 3D noise value at given coordinates
    pub fn get3(&self, x: f64, y: f64, z: f64) -> f64 {
        self.perlin.get([x, y, z])
    }

    /// Generate multi-octave noise (fractal Brownian motion)
    pub fn fbm(&self, x: f64, y: f64, octaves: u32, persistence: f64, lacunarity: f64) -> f64 {
        let mut total = 0.0;
        let mut frequency = 1.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;

        for _ in 0..octaves {
            total += self.perlin.get([x * frequency, y * frequency]) * amplitude;
            max_value += amplitude;
            amplitude *= persistence;
            frequency *= lacunarity;
        }

        total / max_value
    }
}

/// Noise generator using Simplex noise
pub struct SimplexNoise {
    simplex: Arc<Simplex>,
}

impl SimplexNoise {
    /// Create a new Simplex noise generator with default seed
    pub fn new() -> Self {
        Self {
            simplex: Arc::new(Simplex::new(0)),
        }
    }

    /// Create a new Simplex noise generator with a specific seed
    pub fn with_seed(seed: u32) -> Self {
        Self {
            simplex: Arc::new(Simplex::new(seed)),
        }
    }

    /// Get 2D noise value at given coordinates
    pub fn get(&self, x: f64, y: f64) -> f64 {
        self.simplex.get([x, y])
    }

    /// Get 3D noise value at given coordinates
    pub fn get3(&self, x: f64, y: f64, z: f64) -> f64 {
        self.simplex.get([x, y, z])
    }

    /// Generate multi-octave noise (fractal Brownian motion)
    pub fn fbm(&self, x: f64, y: f64, octaves: u32, persistence: f64, lacunarity: f64) -> f64 {
        let mut total = 0.0;
        let mut frequency = 1.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;

        for _ in 0..octaves {
            total += self.simplex.get([x * frequency, y * frequency]) * amplitude;
            max_value += amplitude;
            amplitude *= persistence;
            frequency *= lacunarity;
        }

        total / max_value
    }
}

impl Default for PerlinNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SimplexNoise {
    fn default() -> Self {
        Self::new()
    }
}
