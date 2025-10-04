use serde::{Serialize, Deserialize};
use ndarray::Array3;
use std::fs::File;
use std::io::Write;
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct BitMatrix {
    data: Vec<Vec<Vec<i8>>>,
}

pub fn save_bitmatrix_text(bitmatrix: &Array3<i8>, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut content = String::new();
    content.push_str("G = array([\n");
    for (i, row) in bitmatrix.outer_iter().enumerate() {
        content.push_str("           [");
        for (j, col) in row.outer_iter().enumerate() {
            content.push('[');
            for (k, &val) in col.iter().enumerate() {
                content.push_str(&format!("{}", val));
                if k < col.len() - 1 {
                    content.push_str(", ");
                }
            }
            content.push(']');
            if j < row.len() - 1 {
                content.push_str(", ");
            }
        }
        content.push(']');
        if i < bitmatrix.dim().0 - 1 {
            content.push_str(",\n");
        } else {
            content.push('\n');
        }
    }
    content.push_str("          ], dtype=int8)\n");

    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn save_bitmatrix_json(bitmatrix: &Array3<i8>, filename: &str) -> Result<(), Box<dyn Error>> {
    let data: Vec<Vec<Vec<i8>>> = bitmatrix.outer_iter().map(|row| {
        row.outer_iter().map(|col| col.to_vec()).collect()
    }).collect();
    let bm = BitMatrix { data };
    let json = serde_json::to_string_pretty(&bm)?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}