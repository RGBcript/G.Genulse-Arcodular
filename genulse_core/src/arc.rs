use super::cell::GenulseCell;

pub trait Arc {
    fn process(&mut self, input: &[f32], dt: f32) -> Vec<f32>;
    fn reset(&mut self);
    fn sleep(&mut self);
    fn state(&self) -> &[f32];
}

/// Amygdala Arc
pub struct Amygdala {
    cell: GenulseCell,
    threshold: f32,
}

impl Amygdala {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        Self {
            cell: GenulseCell::new(input_size, output_size, 0.01),
            threshold: 0.5,
        }
    }
}

impl Arc for Amygdala {
    fn process(&mut self, input: &[f32], dt: f32) -> Vec<f32> {
        // CORRECCION AQUI: Agregamos 'mut' para poder modificar la salida
        let mut output = self.cell.process(input, dt);
        
        for val in output.iter_mut() {
            if *val > self.threshold {
                *val = 1.0; 
            } else {
                *val = 0.0; 
            }
        }
        output
    }
    fn reset(&mut self) { self.cell.reset(); }
    fn sleep(&mut self) { self.cell.sleep(); }
    fn state(&self) -> &[f32] { self.cell.state() }
}

/// Hippocampus Arc
pub struct Hippocampus {
    cell: GenulseCell,
}

impl Hippocampus {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        Self { cell: GenulseCell::new(input_size, output_size, 0.5) }
    }
}

impl Arc for Hippocampus {
    fn process(&mut self, input: &[f32], dt: f32) -> Vec<f32> { self.cell.process(input, dt) }
    fn reset(&mut self) { self.cell.reset(); }
    fn sleep(&mut self) { self.cell.sleep(); }
    fn state(&self) -> &[f32] { self.cell.state() }
}

/// VisualCortex Arc
pub struct VisualCortex {
    cell: GenulseCell,
    kernel: Vec<Vec<f32>>,
}

impl VisualCortex {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        let kernel = vec![
            vec![-1.0, -1.0, -1.0],
            vec![-1.0,  8.0, -1.0],
            vec![-1.0, -1.0, -1.0],
        ];
        Self { cell: GenulseCell::new(input_size, output_size, 0.1), kernel }
    }
}

impl Arc for VisualCortex {
    fn process(&mut self, input: &[f32], dt: f32) -> Vec<f32> {
        let convolved = input.to_vec();
        self.cell.process(&convolved, dt)
    }
    fn reset(&mut self) { self.cell.reset(); }
    fn sleep(&mut self) { self.cell.sleep(); }
    fn state(&self) -> &[f32] { self.cell.state() }
}

/// PrefrontalCortex Arc
pub struct PrefrontalCortex {
    cell: GenulseCell,
    amygdala_state: Vec<f32>,
    hippocampus_state: Vec<f32>,
    inhibition_threshold: f32,
}

impl PrefrontalCortex {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        Self {
            cell: GenulseCell::new(input_size, output_size, 0.3),
            amygdala_state: vec![0.0],
            hippocampus_state: vec![0.0],
            inhibition_threshold: 0.8,
        }
    }
    pub fn set_amygdala_state(&mut self, state: Vec<f32>) {
        self.amygdala_state = state;
    }
    pub fn set_hippocampus_state(&mut self, state: Vec<f32>) {
        self.hippocampus_state = state;
    }
}

impl Arc for PrefrontalCortex {
    fn process(&mut self, input: &[f32], dt: f32) -> Vec<f32> {
        let amygdala_panic = self.amygdala_state.iter().any(|&x| x > self.inhibition_threshold);
        if amygdala_panic {
            return vec![0.0; self.cell.state().len()];
        }
        self.cell.process(input, dt)
    }
    fn reset(&mut self) { self.cell.reset(); }
    fn sleep(&mut self) { self.cell.sleep(); }
    fn state(&self) -> &[f32] { self.cell.state() }
}
