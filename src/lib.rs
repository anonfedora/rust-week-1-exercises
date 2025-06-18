// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    let version_hex = &raw_tx_hex[0..8];
    let mut bytes = [0u8; 4];

    match hex::decode_to_slice(version_hex, &mut bytes) {
        Ok(_) => {
            let version = u32::from_le_bytes(bytes);
            Ok(version)
        }
        Err(_) => Err("Hex decode error".to_string()),
    }
}
