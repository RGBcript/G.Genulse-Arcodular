use wasm_bindgen::prelude::*;
use image::ImageReader as ImageReader2;
use hound::WavReader;
use std::io::Cursor;
use xxhash_rust::xxh3::Xxh3;
use hex;

#[wasm_bindgen]
pub struct ToGen(u128);

#[wasm_bindgen]
impl ToGen {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> Self {
        let header = 0x01u128; // Tipo de dato: Texto
        let meta = Self::calculate_meta(input);
        let semantico = Self::calculate_simhash(input);
        let estructural = Self::calculate_metaphone(input);
        let exacto = Self::calculate_xxh3(input);

        let mut bits: u128 = 0;
        bits |= (header as u128) & 0xFF;
        bits |= ((meta as u128) & 0xFF) << 8;
        bits |= ((semantico as u128) & 0xFFFFFFFF) << 16;
        bits |= ((estructural as u128) & 0xFFFFFFFF) << 48;
        bits |= ((exacto as u128) & 0xFFFFFFFFFFFF) << 80;
        ToGen(bits)
    }

    #[wasm_bindgen]
    pub fn new_image(data: &[u8]) -> Self {
        let header = 0x03u128; // Tipo de dato: Imagen
        let (meta, estructural) = Self::calculate_image_hash_from_bytes(data);
        let semantico = Self::calculate_simhash_from_bytes(data);
        let exacto = Self::calculate_xxh3_from_bytes(data);

        let mut bits: u128 = 0;
        bits |= (header as u128) & 0xFF;
        bits |= ((meta as u128) & 0xFF) << 8;
        bits |= ((semantico as u128) & 0xFFFFFFFF) << 16;
        bits |= ((estructural as u128) & 0xFFFFFFFF) << 48;
        bits |= ((exacto as u128) & 0xFFFFFFFFFFFF) << 80;
        ToGen(bits)
    }

    #[wasm_bindgen]
    pub fn new_audio(data: &[u8]) -> Self {
        let header = 0x04u128; // Tipo de dato: Audio
        let (meta, estructural) = Self::calculate_audio_hash_from_bytes(data);
        let semantico = Self::calculate_simhash_from_bytes(data);
        let exacto = Self::calculate_xxh3_from_bytes(data);

        let mut bits: u128 = 0;
        bits |= (header as u128) & 0xFF;
        bits |= ((meta as u128) & 0xFF) << 8;
        bits |= ((semantico as u128) & 0xFFFFFFFF) << 16;
        bits |= ((estructural as u128) & 0xFFFFFFFF) << 48;
        bits |= ((exacto as u128) & 0xFFFFFFFFFFFF) << 80;
        ToGen(bits)
    }

    #[wasm_bindgen]
    pub fn new_code(input: &str) -> Self {
        let header = 0x02u128; // Tipo de dato: Código
        let meta = Self::calculate_meta(input);
        let semantico = Self::calculate_simhash(input);
        let estructural = Self::calculate_metaphone(input);
        let exacto = Self::calculate_xxh3(input);

        let mut bits: u128 = 0;
        bits |= (header as u128) & 0xFF;
        bits |= ((meta as u128) & 0xFF) << 8;
        bits |= ((semantico as u128) & 0xFFFFFFFF) << 16;
        bits |= ((estructural as u128) & 0xFFFFFFFF) << 48;
        bits |= ((exacto as u128) & 0xFFFFFFFFFFFF) << 80;
        ToGen(bits)
    }

    #[wasm_bindgen]
    pub fn new_action(command: &str) -> Self {
        let header = 0x05u128; // Tipo de dato: Acción
        
        // Determinar Meta (Bits 8-15)
        let meta = if command.contains("Key") {
            0x01u128 // Teclado
        } else if command.contains("Click") || command.contains("Move") {
            0x02u128 // Mouse
        } else {
            0x00u128 // Otros
        };
        
        let semantico = Self::calculate_simhash(command);
        let estructural = Self::calculate_metaphone(command);
        let exacto = Self::calculate_xxh3(command);

        let mut bits: u128 = 0;
        bits |= (header as u128) & 0xFF;
        bits |= ((meta as u128) & 0xFF) << 8;
        bits |= ((semantico as u128) & 0xFFFFFFFF) << 16;
        bits |= ((estructural as u128) & 0xFFFFFFFF) << 48;
        bits |= ((exacto as u128) & 0xFFFFFFFFFFFF) << 80;
        ToGen(bits)
    }

    fn calculate_image_hash_from_bytes(data: &[u8]) -> (u8, u32) {
        let img = ImageReader2::new(Cursor::new(data))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        let img_luma = img.to_luma8();
        
        // Calcular brillo promedio (Meta)
        let mut total_luma: u32 = 0;
        let pixel_count = img_luma.width() as usize * img_luma.height() as usize;
        for pixel in img_luma.pixels() {
            total_luma += pixel[0] as u32;
        }
        let avg_luma = (total_luma / pixel_count as u32) as u8;
        
        // Calcular Average Hash (Estructural)
        let img_small = img.resize(8, 4, image::imageops::FilterType::Nearest).to_luma8();
        let mut hash_bits = 0u32;
        let mut total = 0u32;
        let mut count = 0usize;
        
        for pixel in img_small.pixels() {
            total += pixel[0] as u32;
            count += 1;
        }
        let avg = total / count as u32;
        
        let mut bit_index = 0;
        for pixel in img_small.pixels() {
            if pixel[0] as u32 > avg && bit_index < 32 {
                hash_bits |= 1 << (31 - bit_index);
            }
            bit_index += 1;
        }
        
        (avg_luma, hash_bits)
    }

    fn calculate_audio_hash_from_bytes(data: &[u8]) -> (u8, u32) {
        let cursor = Cursor::new(data);
        let reader = WavReader::new(cursor).unwrap();
        let samples: Vec<i32> = reader.into_samples::<i16>().map(|s| s.unwrap() as i32).collect();
        
        // Calcular volumen máximo (Meta)
        let max_volume = samples.iter().map(|&s| s.abs()).max().unwrap_or(0);
        let meta = (max_volume.min(255)) as u8;
        
        // Calcular Zero Crossing Rate (Estructural)
        let mut zero_crossings = 0u32;
        for i in 1..samples.len() {
            if (samples[i-1] < 0 && samples[i] >= 0) || (samples[i-1] >= 0 && samples[i] < 0) {
                zero_crossings += 1;
            }
        }
        
        (meta, zero_crossings)
    }

    fn calculate_meta(input: &str) -> u128 {
        let length = input.len() as u128;
        if length == 0 {
            0
        } else {
            (length.next_power_of_two().trailing_zeros() as u128) << 8
        }
    }

    fn calculate_simhash(input: &str) -> u128 {
        let mut hash = 0u32;
        for byte in input.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
        }
        hash as u128
    }

    fn calculate_simhash_from_bytes(data: &[u8]) -> u128 {
        let mut hash = 0u32;
        for byte in data {
            hash = hash.wrapping_mul(31).wrapping_add(*byte as u32);
        }
        hash as u128
    }

    fn calculate_metaphone(input: &str) -> u128 {
        let mut hash = 0u32;
        for byte in input.bytes() {
            hash = hash.wrapping_mul(131).wrapping_add(byte as u32);
        }
        hash as u128
    }

    fn calculate_xxh3(input: &str) -> u128 {
        let mut hasher = Xxh3::new();
        hasher.update(input.as_bytes());
        let hash = hasher.digest();
        (hash >> 16) as u128 // Tomamos los 48 bits superiores
    }

    fn calculate_xxh3_from_bytes(data: &[u8]) -> u128 {
        let mut hasher = Xxh3::new();
        hasher.update(data);
        let hash = hasher.digest();
        (hash >> 16) as u128 // Tomamos los 48 bits superiores
    }

    #[wasm_bindgen]
    pub fn to_hex(&self) -> String {
        hex::encode(self.0.to_be_bytes())
    }

    #[wasm_bindgen]
    pub fn get_header(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    #[wasm_bindgen]
    pub fn get_meta(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    #[wasm_bindgen]
    pub fn get_semantico(&self) -> u32 {
        ((self.0 >> 16) & 0xFFFFFFFF) as u32
    }

    #[wasm_bindgen]
    pub fn get_estructural(&self) -> u32 {
        ((self.0 >> 48) & 0xFFFFFFFF) as u32
    }

    #[wasm_bindgen]
    pub fn get_exacto(&self) -> u64 {
        (self.0 >> 80) as u64
    }
}

#[wasm_bindgen]
pub fn togen_from_string(input: &str) -> String {
    let togen = ToGen::new(input);
    togen.to_hex()
}

#[wasm_bindgen]
pub fn togen_from_bytes_image(data: &[u8]) -> String {
    let togen = ToGen::new_image(data);
    togen.to_hex()
}

#[wasm_bindgen]
pub fn togen_from_bytes_audio(data: &[u8]) -> String {
    let togen = ToGen::new_audio(data);
    togen.to_hex()
}

#[wasm_bindgen]
pub fn togen_from_code(input: &str) -> String {
    let togen = ToGen::new_code(input);
    togen.to_hex()
}

#[wasm_bindgen]
pub fn togen_from_action(command: &str) -> String {
    let togen = ToGen::new_action(command);
    togen.to_hex()
}

#[wasm_bindgen]
pub fn togen_image_stream(data: &[u8], cols: usize, rows: usize) -> js_sys::Array {
    let togens = new_image_stream(data, (cols, rows));
    let hexes: Vec<JsValue> = togens.iter().map(|t| JsValue::from_str(&t.to_hex())).collect();
    js_sys::Array::from_iter(hexes.into_iter())
}

// Helper functions for patch processing
fn calculate_image_hash_from_patch(img: &image::DynamicImage) -> (u8, u32) {
    let img_luma = img.to_luma8();

    // Calculate average brightness (Meta)
    let mut total_luma: u32 = 0;
    let pixel_count = img_luma.width() as usize * img_luma.height() as usize;
    for pixel in img_luma.pixels() {
        total_luma += pixel[0] as u32;
    }
    let avg_luma = (total_luma / pixel_count as u32) as u8;

    // Calculate Average Hash (Estructural)
    let img_small = img.resize(8, 4, image::imageops::FilterType::Nearest).to_luma8();
    let mut hash_bits = 0u32;
    let mut total = 0u32;
    let mut count = 0usize;

    for pixel in img_small.pixels() {
        total += pixel[0] as u32;
        count += 1;
    }
    let avg = total / count as u32;

    let mut bit_index = 0;
    for pixel in img_small.pixels() {
        if pixel[0] as u32 > avg && bit_index < 32 {
            hash_bits |= 1 << (31 - bit_index);
        }
        bit_index += 1;
    }

    (avg_luma, hash_bits)
}

fn calculate_simhash_from_patch(img: &image::DynamicImage) -> u128 {
    // Convert image to bytes for hashing
    let mut bytes = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).unwrap();
    ToGen::calculate_simhash(&String::from_utf8_lossy(&bytes))
}

fn calculate_xxh3_from_patch(img: &image::DynamicImage) -> u128 {
    // Convert image to bytes for hashing
    let mut bytes = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).unwrap();
    ToGen::calculate_xxh3(&String::from_utf8_lossy(&bytes))
}

// Multitogen Stream for Images
pub fn new_image_stream(data: &[u8], grid_size: (usize, usize)) -> Vec<ToGen> {


    // Load image from bytes
    let img = ImageReader2::new(Cursor::new(data))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let (cols, rows) = grid_size;
    let patch_width = img.width() as usize / cols;
    let patch_height = img.height() as usize / rows;

    let mut togens = Vec::new();

    for y in 0..rows {
        for x in 0..cols {
            // Calculate patch boundaries
            let left = x * patch_width;
            let top = y * patch_height;
            let right = (x + 1) * patch_width;
            let bottom = (y + 1) * patch_height;

            // Crop patch
            let patch = img.crop_imm(left as u32, top as u32, (right - left) as u32, (bottom - top) as u32);

            // Calculate TOGEN components for this patch
            let (meta, estructural) = calculate_image_hash_from_patch(&patch);
            let semantico = calculate_simhash_from_patch(&patch);
            let exacto = calculate_xxh3_from_patch(&patch);

            // Create TOGEN for this patch
            let mut bits: u128 = 0;
            bits |= 0x03u128; // Header: Image
            bits |= ((meta as u128) & 0xFF) << 8;
            bits |= ((semantico as u128) & 0xFFFFFFFF) << 16;
            bits |= ((estructural as u128) & 0xFFFFFFFF) << 48;
            bits |= ((exacto as u128) & 0xFFFFFFFFFFFF) << 80;

            togens.push(ToGen(bits));
        }
    }

    togens
}