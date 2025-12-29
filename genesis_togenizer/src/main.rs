use clap::Parser;
use hex;
use xxhash_rust::xxh3::Xxh3;
use image::ImageReader;
use hound::WavReader;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Texto a convertir en Togen
    input: String,
}

struct ToGen(u128);

impl ToGen {
    fn new(input: &str) -> Self {
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

    fn new_image(path: &str) -> Self {
        let header = 0x03u128; // Tipo de dato: Imagen
        let (meta, estructural) = Self::calculate_image_hash(path);
        let semantico = Self::calculate_simhash(path);
        let exacto = Self::calculate_xxh3(path);

        let mut bits: u128 = 0;
        bits |= (header as u128) & 0xFF;
        bits |= ((meta as u128) & 0xFF) << 8;
        bits |= ((semantico as u128) & 0xFFFFFFFF) << 16;
        bits |= ((estructural as u128) & 0xFFFFFFFF) << 48;
        bits |= ((exacto as u128) & 0xFFFFFFFFFFFF) << 80;
        ToGen(bits)
    }

    fn new_audio(path: &str) -> Self {
        let header = 0x04u128; // Tipo de dato: Audio
        let (meta, estructural) = Self::calculate_audio_hash(path);
        let semantico = Self::calculate_simhash(path);
        let exacto = Self::calculate_xxh3(path);

        let mut bits: u128 = 0;
        bits |= (header as u128) & 0xFF;
        bits |= ((meta as u128) & 0xFF) << 8;
        bits |= ((semantico as u128) & 0xFFFFFFFF) << 16;
        bits |= ((estructural as u128) & 0xFFFFFFFF) << 48;
        bits |= ((exacto as u128) & 0xFFFFFFFFFFFF) << 80;
        ToGen(bits)
    }

    fn new_code(path: &str) -> Self {
        let header = 0x02u128; // Tipo de dato: Código
        let meta = Self::calculate_meta(path);
        let semantico = Self::calculate_simhash(path);
        let estructural = Self::calculate_metaphone(path);
        let exacto = Self::calculate_xxh3(path);

        let mut bits: u128 = 0;
        bits |= (header as u128) & 0xFF;
        bits |= ((meta as u128) & 0xFF) << 8;
        bits |= ((semantico as u128) & 0xFFFFFFFF) << 16;
        bits |= ((estructural as u128) & 0xFFFFFFFF) << 48;
        bits |= ((exacto as u128) & 0xFFFFFFFFFFFF) << 80;
        ToGen(bits)
    }

    fn new_action(command: &str) -> Self {
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

    fn calculate_image_hash(path: &str) -> (u8, u32) {
        let img = ImageReader::open(path).unwrap().decode().unwrap();
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

    fn calculate_audio_hash(path: &str) -> (u8, u32) {
        let reader = WavReader::open(path).unwrap();
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

    fn to_hex(&self) -> String {
        hex::encode(self.0.to_be_bytes())
    }

    fn print_parts(&self, input: &str) {
        let header = (self.0 & 0xFF) as u8;
        let meta = ((self.0 >> 8) & 0xFF) as u8;
        let semantico = ((self.0 >> 16) & 0xFFFFFFFF) as u32;
        let estructural = ((self.0 >> 48) & 0xFFFFFFFF) as u32;
        let exacto = (self.0 >> 80) as u64;

        println!("Input: \"{}\"", input);
        println!("TOGEN: {}", self.to_hex());
        println!("[Header: 0x{:02X}] [Meta: 0x{:02X}] [Semántico: 0x{:08X}] [Estructural: 0x{:08X}] [Exacto: 0x{:012X}]",
                 header, meta, semantico, estructural, exacto);
    }
}

fn main() {
    let args = Args::parse();
    
    let togen = if args.input.starts_with("ACT:") {
        // Procesar como acción
        let command = &args.input[4..]; // Eliminar prefijo "ACT:"
        ToGen::new_action(command)
    } else {
        // Procesar como archivo o texto
        let path = Path::new(&args.input);
        if path.exists() {
            if let Some(ext) = path.extension() {
                match ext.to_str().unwrap() {
                    "jpg" | "jpeg" | "png" => ToGen::new_image(&args.input),
                    "wav" => ToGen::new_audio(&args.input),
                    "rs" | "py" | "js" | "json" | "cpp" | "java" | "go" | "ts" | "html" | "css" => ToGen::new_code(&args.input),
                    _ => ToGen::new(&args.input),
                }
            } else {
                ToGen::new(&args.input)
            }
        } else {
            ToGen::new(&args.input)
        }
    };
    
    togen.print_parts(&args.input);
}
