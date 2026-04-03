use esp_idf_hal::{gpio::*};
use std::thread::sleep;
use std::time::{Duration, Instant};
use crate::utils::{self, DhtData};

static NUMBER_OF_TRY_BEFORE_ERROR: u8 = 10;

pub fn read(sensor: &mut PinDriver<InputOutput>)
    -> Result<DhtData, String>{
  //! Read the value of the sensor
  let mut tries: u8 = 0;

  loop{
    tries = tries + 1 ;
    connect(sensor);
    
    if get_level_until_timeout(sensor, Level::Low, Duration::from_secs(1)).is_ok() {
      if get_level_until_timeout(sensor, Level::High, Duration::from_secs(1)).is_ok() {
        if get_level_until_timeout(sensor, Level::Low, Duration::from_secs(1)).is_ok(){
          match get(sensor){
            Ok(vals) => {
              // log::info!("vals read correctly: {vals:?}");
              return Ok(vals)
            }
            Err(_) => {}
          }
        }
      }
    }
    
    if tries >= NUMBER_OF_TRY_BEFORE_ERROR {
      // log::info!("It tried to read {tries} times but the reading didn't work");
      return Err(format!("It tried to read {tries} times but the reading didn't work"));
    }
  }
}

fn connect(sensor: &mut PinDriver<'_, InputOutput>){
  //!Send the connect sequence to the sensor
  // log::info!("Starting communication");
  
  PinDriver::set_high(sensor).unwrap();
  sleep(Duration::from_millis(100));
  
  PinDriver::set_low(sensor).unwrap();
  
  sleep(Duration::from_millis(30));
  
  PinDriver::set_high(sensor).unwrap();
}

fn get_level_until_timeout(sensor: &mut PinDriver<'_, InputOutput>, level_meter: Level, timeout: Duration)
    -> Result<Duration, String>{
  //!Get a level with a timeout, returns the elapsed time
  let start = Instant::now();
  
  loop{
    if PinDriver::get_level(sensor) == level_meter {
      return Ok(start.elapsed());
    } 
    
    if start.elapsed() >= timeout{
      return Err(format!("Timeout has been exceeded"));
    }
  }
}

fn get(sensor: &mut PinDriver<'_, InputOutput>) -> Result<DhtData, String>{
  //!Return the Data struct of temperature and humidity read by the sensor
  let mut bits: Vec<u8> = Vec::new();

  loop{
    //Wait for timeout between bits is finshed
    if get_level_until_timeout(sensor, Level::High, Duration::from_secs(1)).is_err() {
      break;
    }
    
    //Start reading bit
    match get_level_until_timeout(sensor, Level::Low, Duration::from_secs(1)){
      Ok(elapsed) => {
        if elapsed.as_micros() <= 37{
          bits.push(0);
        } else {
          bits.push(1);
        }
      }
      Err(_) => {
        break;
      }
    };
  }

  match check(bits){
    Ok(bytes) => { Ok(utils::convert_to_data_struct(bytes)) }
    Err(error) => { return Err(error) }
  }
}

fn check(bits: Vec<u8>) -> Result<[u8; 5], String>{
  //!Check if the value read is correct
  const LENGTH: usize = 40;
  
  if bits.len() != LENGTH {
    return Err(format!("There's not {LENGTH} bits"))
  }

  let bytes = utils::bits_to_bytes(bits.clone());

  if utils::checksum(bytes).is_err(){
    // log::info!("checksum didn't pass :( {bytes:?}. here's bits: {bits:?}");
    return Err("checksum didn't pass :(".to_string())
  }

  Ok(bytes)
}