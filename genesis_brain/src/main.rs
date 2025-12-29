use genesis_brain::Brain;
use std::thread;
use std::time::Duration;
use log::info;

fn main() {
    // Inicializar logger para ver los mensajes de log (info!, warn!)
    // Se puede configurar con variable de entorno RUST_LOG=info
    env_logger::init();

    // 1. Crea el Brain
    // Definimos un tamano de red modesto para la simulacion
    let input_size = 64;  
    let output_size = 64;
    let mut brain = Brain::new(input_size, output_size);
    
    info!("Iniciando ciclo de vida de Genesis Brain...");
    
    let mut local_cycle: u64 = 0;

    // Bucle infinito
    loop {
        local_cycle += 1;

        // 2. Simula una entrada visual (ruido o patrón)
        // Generamos un patrón pseudo-aleatorio determinista sin usar crate `rand`
        let mut input = vec![0.0; input_size];
        for i in 0..input_size {
            // Genera algo parecido a ruido estatico o ondas variando por ciclo
            let noise = ((i as f32 * 0.5) + (local_cycle as f32 * 0.1)).sin().abs();
            input[i] = noise;
        }

        // 3. Llama a brain.perceive() y brain.think(0.1)
        brain.perceive(input);
        brain.think(0.1);

        // 4. Imprime el estado actual
        // Usamos \r para sobreescribir la linea si se ejecuta en terminal interactiva, 
        // o println normal para logs. El usuario pidio "Imprime el estado".
        println!("Ciclo {}: {}", local_cycle, brain.status());

        // 5. Cada 100 ciclos, llama a brain.sleep()
        if local_cycle % 100 == 0 {
            brain.sleep();
            println!("--- DORMIDO ---");
        }

        // 6. Usa std::thread::sleep para no quemar la CPU real
        // 50ms = ~20Hz
        thread::sleep(Duration::from_millis(50));
    }
}
