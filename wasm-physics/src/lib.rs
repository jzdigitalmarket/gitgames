use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ParticleSystem {
    count: usize,
    positions_x: Vec<f32>,
    positions_y: Vec<f32>,
    velocities_x: Vec<f32>,
    velocities_y: Vec<f32>,
}

#[wasm_bindgen]
impl ParticleSystem {
    #[wasm_bindgen(constructor)]
    pub fn new(count: usize) -> Self {
        let mut positions_x = Vec::with_capacity(count);
        let mut positions_y = Vec::with_capacity(count);
        let mut velocities_x = Vec::with_capacity(count);
        let mut velocities_y = Vec::with_capacity(count);

        for _ in 0..count {
            positions_x.push(0.0);
            positions_y.push(0.0);
            velocities_x.push(1.0);
            velocities_y.push(1.5);
        }

        ParticleSystem {
            count,
            positions_x,
            positions_y,
            velocities_x,
            velocities_y,
        }
    }

    // Processa os cálculos em paralelo sem garbage collector do JS
    pub fn update(&mut self, dt: f32) {
        for i in 0..self.count {
            self.positions_x[i] += self.velocities_x[i] * dt;
            self.positions_y[i] += self.velocities_y[i] * dt;
        }
    }

    // Retorna o ponteiro de memória diretamente para leitura rápida pelo JS
    pub fn positions_x_ptr(&self) -> *const f32 {
        self.positions_x.as_ptr()
    }
}
