use std::time::Duration;

#[derive(Debug, Clone)]
pub struct GenulseCell {
    pub w_slow: Vec<i8>,
    pub a_fast: Vec<f32>,
    pub state: Vec<f32>,
    pub tau: f32,
    pub last_update: Duration,
}

impl GenulseCell {
    pub fn new(input_size: usize, output_size: usize, tau: f32) -> Self {
        Self {
            w_slow: vec![1; input_size * output_size], 
            a_fast: vec![0.0; input_size * output_size],
            state: vec![0.0; output_size],
            tau,
            last_update: Duration::from_secs(0),
        }
    }

    pub fn process(&mut self, input: &[f32], dt: f32) -> Vec<f32> {
        self.update_fast_weights(input, dt);
        let output = self.compute_output(input, dt);
        
        // --- ESTABILIZACIÓN (Leaky Integrator) ---
        // La neurona pierde el 10% de su energía en cada paso (freno natural)
        self.state = output.iter().map(|&x| x * 0.9).collect();
        
        output
    }

    fn update_fast_weights(&mut self, input: &[f32], dt: f32) {
        for (i, &input_val) in input.iter().enumerate() {
            for (j, state_val) in self.state.iter().enumerate() {
                let index = i * self.state.len() + j;
                if index < self.a_fast.len() {
                    // Hebbian learning con limite: si ya es muy alto, no aprende mas
                    if self.a_fast[index] < 10.0 { 
                        self.a_fast[index] += 0.05 * input_val * state_val * dt / self.tau;
                    }
                }
            }
        }
    }

    fn compute_output(&mut self, input: &[f32], _dt: f32) -> Vec<f32> {
        let mut output = vec![0.0; self.state.len()];
        for (j, output_val) in output.iter_mut().enumerate() {
            for (i, &input_val) in input.iter().enumerate() {
                let index = i * self.state.len() + j;
                if index < self.w_slow.len() {
                    let w_slow_val = self.w_slow[index] as f32;
                    let combined_weight = w_slow_val + self.a_fast[index];
                    *output_val += input_val * combined_weight;
                }
            }
        }
        
        // --- ESTABILIZACIÓN (Clamping) ---
        // ReLU + Limite maximo de 50.0 para evitar infinito
        for val in output.iter_mut() { 
            *val = val.max(0.0).min(50.0); 
        }
        
        // No guardamos state aqui, se guarda en process() con decay
        output
    }

    pub fn reset(&mut self) { self.state.fill(0.0); }

    pub fn sleep(&mut self) {
        let alpha = 0.1;
        for i in 0..self.w_slow.len() {
            // Protección contra NaN/Inf durante el sueño
            let current_fast = if self.a_fast[i].is_finite() { self.a_fast[i] } else { 0.0 };
            
            let consolidated = self.w_slow[i] as f32 + alpha * current_fast;
            
            // BitNet Quantization (-1, 0, 1)
            self.w_slow[i] = if consolidated > 0.5 { 1 } else if consolidated < -0.5 { -1 } else { 0 };
            self.a_fast[i] = 0.0;
        }
        // Limpieza profunda del estado liquido
        self.state.fill(0.0);
    }
    
    pub fn state(&self) -> &[f32] { &self.state }
}
