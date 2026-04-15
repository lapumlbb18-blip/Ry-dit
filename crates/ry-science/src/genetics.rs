// crates/ry-science/src/genetics.rs
// Sistema de Genética y Mutación para Ry-Dit
//
// Conecta con Radiación (ry-physics) para mutaciones inducidas.
// Permite evolución procedural de criaturas y texturas.

use serde_json::{json, Value};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Nucleobase {
    A, T, C, G
}

impl Nucleobase {
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..4) {
            0 => Nucleobase::A,
            1 => Nucleobase::T,
            2 => Nucleobase::C,
            _ => Nucleobase::G,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Nucleobase::A => 'A',
            Nucleobase::T => 'T',
            Nucleobase::C => 'C',
            Nucleobase::G => 'G',
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'A' => Some(Nucleobase::A),
            'T' => Some(Nucleobase::T),
            'C' => Some(Nucleobase::C),
            'G' => Some(Nucleobase::G),
            _ => None,
        }
    }
}

pub struct DNA {
    pub sequence: Vec<Nucleobase>,
}

impl DNA {
    pub fn new(length: usize) -> Self {
        let mut sequence = Vec::with_capacity(length);
        for _ in 0..length {
            sequence.push(Nucleobase::random());
        }
        DNA { sequence }
    }

    pub fn from_string(s: &str) -> Self {
        let sequence = s.chars().filter_map(Nucleobase::from_char).collect();
        DNA { sequence }
    }

    pub fn to_string(&self) -> String {
        self.sequence.iter().map(|b| b.to_char()).collect()
    }

    /// Mutación puntual basada en probabilidad
    /// radiation_factor: aumenta la probabilidad de mutación (0.0 a 1.0+)
    pub fn mutate(&mut self, mutation_rate: f64, radiation_factor: f64) -> usize {
        let mut rng = rand::thread_rng();
        let mut count = 0;
        let effective_rate = (mutation_rate * (1.0 + radiation_factor)).clamp(0.0, 1.0);

        for base in self.sequence.iter_mut() {
            if rng.gen_bool(effective_rate) {
                *base = Nucleobase::random();
                count += 1;
            }
        }
        count
    }

    /// Crossover (recombinación) de dos ADN
    pub fn crossover(parent_a: &DNA, parent_b: &DNA) -> DNA {
        let mut rng = rand::thread_rng();
        let len = parent_a.sequence.len().min(parent_b.sequence.len());
        let mid = rng.gen_range(0..len);
        
        let mut child_seq = Vec::with_capacity(len);
        for i in 0..mid {
            child_seq.push(parent_a.sequence[i]);
        }
        for i in mid..len {
            child_seq.push(parent_b.sequence[i]);
        }
        
        DNA { sequence: child_seq }
    }

    /// Mapear ADN a atributos numéricos (fenotipo)
    /// Divide el ADN en "genes" de longitud N y calcula valores
    pub fn express(&self, gene_length: usize) -> Vec<f64> {
        let mut phenotype = Vec::new();
        for chunk in self.sequence.chunks(gene_length) {
            let mut val = 0.0;
            let mut weight = 1.0;
            for base in chunk {
                let base_val = match base {
                    Nucleobase::A => 0.0,
                    Nucleobase::T => 0.33,
                    Nucleobase::C => 0.66,
                    Nucleobase::G => 1.0,
                };
                val += base_val * weight;
                weight *= 0.5;
            }
            phenotype.push(val);
        }
        phenotype
    }
}

// ============================================================================
// MODULE EXPORTS
// ============================================================================

pub fn dna_new(params: Value) -> Value {
    let len = params.as_u64().unwrap_or(32) as usize;
    let dna = DNA::new(len);
    json!({
        "sequence": dna.to_string(),
        "length": len
    })
}

pub fn dna_mutate(params: Value) -> Value {
    let obj = params.as_object();
    let seq = obj.and_then(|o| o.get("sequence")).and_then(|v| v.as_str()).unwrap_or("ATCG");
    let rate = obj.and_then(|o| o.get("rate")).and_then(|v| v.as_f64()).unwrap_or(0.01);
    let radiation = obj.and_then(|o| o.get("radiation")).and_then(|v| v.as_f64()).unwrap_or(0.0);

    let mut dna = DNA::from_string(seq);
    let mutations = dna.mutate(rate, radiation);

    json!({
        "sequence": dna.to_string(),
        "mutations": mutations
    })
}

pub fn dna_express(params: Value) -> Value {
    let obj = params.as_object();
    let seq = obj.and_then(|o| o.get("sequence")).and_then(|v| v.as_str()).unwrap_or("ATCG");
    let gene_len = obj.and_then(|o| o.get("gene_length")).and_then(|v| v.as_u64()).unwrap_or(4) as usize;

    let dna = DNA::from_string(seq);
    let phenotype = dna.express(gene_len);

    json!({
        "phenotype": phenotype
    })
}
