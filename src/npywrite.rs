extern crate byteorder;
extern crate clap;

use std::io::prelude::*;
use std::fs::File;
use std::io;
use std::io::SeekFrom;

use byteorder::{LittleEndian, WriteBytesExt};
use clap::{Arg, App};


enum DType { U32, I32, U64, I64, F32, F64 }

fn dtype_to_header_str(dt : &DType) -> String {
  match *dt {
    DType::U32 => "<u4".to_string(),
    DType::U64 => "<u8".to_string(),
    DType::I32 => "<i4".to_string(),
    DType::I64 => "<i8".to_string(),
    DType::F32 => "<f4".to_string(),
    DType::F64 => "<f8".to_string()
  }
}

fn write_header(
  fout : &mut File,
  n_rows : u32,
  n_cols : u32,
  data_type : &DType)
  -> Result<(), io::Error> {
  try!(fout.seek(SeekFrom::Start(0)));
  try!(fout.write_u8(147));
  try!(fout.write_all("NUMPY".as_bytes()));
  try!(fout.write_u8(1));
  try!(fout.write_u8(0));
  try!(fout.write_u16::<LittleEndian>(118));
  match n_cols {
    0 => {
      let header_line = format!(
        "{{'descr': '{}', 'fortran_order': False, 'shape': (0,)}}",
        dtype_to_header_str(data_type));
      try!(fout.write_all(header_line.as_bytes()));
    },
    1 => {
      let header_line = format!(
        "{{'descr': '{}', 'fortran_order': False, 'shape': ({},)}}",
        dtype_to_header_str(data_type), n_rows);
      try!(fout.write_all(header_line.as_bytes()));
    },
    _ => {
      let header_line = format!(
        "{{'descr': '{}', 'fortran_order': False, 'shape': ({},{})}}",
        dtype_to_header_str(data_type), n_rows, n_cols);
      try!(fout.write_all(header_line.as_bytes()));
    }
  };
  Ok(())
}

fn convert_line(
  line : &str,
  fout : &mut File,
  dtype : &DType,
  sep : &str)
  -> Result<u32, io::Error> {
  let mut line_byte_out = vec![];
  let mut n_cols : u32 = 0;
  for t in line.split(sep) {
    match *dtype {
      DType::U32 => {
        line_byte_out
          .write_u32::<LittleEndian>(
            t.parse::<u32>().unwrap()).unwrap();
      },
      DType::U64 => {
        line_byte_out
          .write_u64::<LittleEndian>(
            t.parse::<u64>().unwrap()).unwrap();
      },
      DType::I32 => {
        line_byte_out
          .write_i32::<LittleEndian>(
            t.parse::<i32>().unwrap()).unwrap();
      },
      DType::I64 => {
        line_byte_out
          .write_i64::<LittleEndian>(
            t.parse::<i64>().unwrap()).unwrap();
      },
      DType::F32 => {
        line_byte_out
          .write_f32::<LittleEndian>(
            t.parse::<f32>().unwrap()).unwrap();
      },
      DType::F64 => {
        line_byte_out
          .write_f64::<LittleEndian>(
            t.parse::<f64>().unwrap()).unwrap();
      }
    }
    n_cols += 1;
  }
  try!(fout.write_all(&line_byte_out));
  Ok(n_cols)
}

fn convert_data(
  fout : &mut File,
  fin : io::Stdin,
  dtype : &DType,
  sep : &str)
  -> Result<(u32, u32), io::Error> {
  let mut n_cols : u32 = 0;
  let mut n_rows : u32 = 0;
  for line in fin.lock().lines() {
    match line {
      Ok(l) => {
        n_rows += 1;
        n_cols = try!(convert_line(&l, fout, dtype, sep));
      },
      Err(e) => return Err(e)
    }
  };
  Ok((n_rows, n_cols))
}

fn txt_to_npy(
  fin : io::Stdin,
  fout : &mut File,
  data_type : &DType,
  sep : &str)
  -> Result<(), io::Error> {
  for _ in 0..128 { try!(fout.write_all(b" ")); }
  convert_data(fout, fin, data_type, sep)
    .and_then(|(n_rows, n_cols)| write_header(fout, n_rows, n_cols, data_type))
}

fn main() {
  let matches = App::new("npywrite")
    .version("1.0")
    .author("Nicola Montecchio <nicola.montecchio@gmail.com>")
    .about("Read text from STDIN and write a numpy array file.")
    .arg(Arg::with_name("dtype")
         .short("d")
         .long("dtype")
         .value_name("type")
         .help("data type, one of: u32, i32, u64, i64, f32, f64 (default: f32)")
         .takes_value(true))
    .arg(Arg::with_name("separator")
         .short("s")
         .long("separator")
         .value_name("S")
         .help("character used to separate fields (default: space)")
         .takes_value(true))
    .arg(Arg::with_name("output")
         .short("o")
         .long("output")
         .value_name("FILE")
         .help("output file name (default: data.npy)")
         .takes_value(true))
    .get_matches();
  let stdin = io::stdin();
  let out_fpath = matches.value_of("output").unwrap_or("data.npy");
  let sep = matches.value_of("separator").unwrap_or(" ");
  let dtype = match matches.value_of("dtype").unwrap_or("f32") {
    "u32" => DType::U32,
    "u64" => DType::U64,
    "i32" => DType::I32,
    "i64" => DType::I64,
    "f32" => DType::F32,
    "f64" => DType::F64,
    _ => DType::F32
  };
  match File::create(out_fpath).and_then(
    |mut fout| txt_to_npy(stdin, &mut fout, &dtype, &sep)) {
      Ok(_) => {},
      Err(_) => { println!("an error occurred!")}
    }
}
