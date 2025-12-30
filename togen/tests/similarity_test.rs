use togen::ToGen;

fn hamming_distance_u32(a: u32, b: u32) -> u32 {
    (a ^ b).count_ones()
}

#[test]
fn test_structural_similarity() {
    // Caso 1: Palabras ortográficamente similares (deberían tener Togens cercanos)
    let t1 = ToGen::new("genesis");
    let t2 = ToGen::new("genesys");

    let dist_sem = hamming_distance_u32(t1.get_semantico(), t2.get_semantico());
    let dist_est = hamming_distance_u32(t1.get_estructural(), t2.get_estructural());

    println!("Distance 'genesis' vs 'genesys': Sem={}, Est={}", dist_sem, dist_est);

    // Caso 2: Palabras totalmente diferentes
    let t3 = ToGen::new("skynet");
    let dist_diff_sem = hamming_distance_u32(t1.get_semantico(), t3.get_semantico());
    let dist_diff_est = hamming_distance_u32(t1.get_estructural(), t3.get_estructural());

    println!("Distance 'genesis' vs 'skynet': Sem={}, Est={}", dist_diff_sem, dist_diff_est);

    // Expectativa: La distancia entre similares debe ser menor que entre diferentes
    // Nota: Con el hash actual (rolling), esto probablemente falle. 
    // Este test es para validar la hipótesis del usuario.
    assert!(dist_sem < dist_diff_sem || dist_est < dist_diff_est, "El hash no conserva similitud estructural");
}
