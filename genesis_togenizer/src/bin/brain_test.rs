use genulse_core::arc::{Arc, VisualCortex, Amygdala};

fn main() {
    println!("🧠 INICIANDO GENESIS SYSTEM v1.1 (Integration Test)...");

    println!("👁️  Generando Estímulo Visual Simulado (4x4 patches)...");
    // Simulamos patrón visual (borde vertical)
    let visual_input: Vec<f32> = vec![
        1.0, 0.0, 0.0, 1.0,
        1.0, 0.0, 0.0, 1.0,
        1.0, 0.0, 0.0, 1.0,
        1.0, 0.0, 0.0, 1.0,
    ];

    // Instanciar Arcos
    let mut visual_cortex = VisualCortex::new(16, 16);
    let mut amygdala = Amygdala::new(16, 16);

    let dt = 0.1; // 100ms por paso
    
    println!("⚡ Ejecutando Ciclo Cognitivo...");
    for t in 0..5 {
        // A. Procesar Visión (Convolución 3x3 interna)
        let v_output = visual_cortex.process(&visual_input, dt);
        
        // B. Chequear Amenaza
        let a_output = amygdala.process(&visual_input, dt);
        
        // C. Ver si hay pánico (> 0.9)
        let panic = a_output.iter().any(|&x| x > 0.9);
        
        // Imprimimos la activación de la neurona central (index 5)
        println!("   [T={}] Visual Activity (Neuron 5): {:.4} | Panic: {}", 
            t, v_output[5], panic);
            
        // D. Consolidación (Sleep) al final
        if t == 4 {
            println!("💤 Durmiendo (Consolidando memoria)...");
            visual_cortex.sleep();
        }
    }
    
    println!("✅ Test Finalizado. El cerebro responde.");
}
