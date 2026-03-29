pub fn bits_to_bytes(bits: Vec<u8>) -> [u8; 5] {
  //! Converts bits to bytes (MSB)
  let mut bytes = [0u8; 5];

  bits.iter()
  .enumerate()
  .for_each(|(i, x)| {
    let byte_index = i / 8;
    let bit_position = 7 - (i % 8); //Flip to MSB

    if byte_index < bytes.len() {
      bytes[byte_index] |= x << bit_position;
    }
  });

  bytes
}

pub fn checksum(bytes: [u8; 5]) -> Result<(), ()> {
  //! Is checksum passed
  let mut total: u16 = 0;

  bytes.iter().for_each(|x| {total = total + *x as u16;});

  //Remove the checksum value from checksum total
  total = total - (bytes[4] as u16);

  if total == (bytes[4] as u16){
    Ok(())
  }else{
    Err(())
  }
}

pub fn convert_to_decimal(bytes: [u8; 5]) -> [f32; 2]{
  //! convert bytes to float
  [
    bytes[0] as f32 + bytes[1] as f32 / 10.0,
    bytes[2] as f32 + bytes[3] as f32 / 10.0
  ]
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_bit_convert() {
    let bits = [0,0,0,0,1,1,1,1,
                1,0,0,0,1,1,1,1,
                1,1,1,1,0,0,0,0,
                0,0,0,0,0,0,0,0,
                1,1,1,1,1,1,1,1];

    assert_eq!(bits_to_bytes(bits.to_vec()), [15, 143, 240, 0, 255]);
  }
  
  #[test]
  fn test_checksum() {
    let bytes = [1,2,3,4,10];
    
    assert_eq!(checksum(bytes), Ok(()));
  }

  #[test]
  fn test_convert_to_decimal() {
    let bytes = [1,2,3,4,10];
    assert_eq!(convert_to_decimal(bytes), [1.2, 3.4]);
  }
}
