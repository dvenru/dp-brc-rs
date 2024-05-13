pub mod const_encoding;
pub mod image;

use std::fmt::Error;
use const_encoding::*;

pub struct BarCode(Vec<u8>);

impl BarCode {
    pub fn from_str<T: AsRef<str>>(data: T) -> Result<BarCode, Error> {
        let chars: Vec<u8> = data.as_ref()
            .chars()
            .map(|c| c.to_digit(10).expect("") as u8)
            .collect();

        Ok(BarCode(chars[0..12].to_vec()))
    }

    pub fn get_str(&self) -> String {
        self.0.iter()
            .map(|num| num.to_string())
            .collect::<String>()
    }

    pub fn decode(data: Vec<u8>) -> Result<BarCode, Error> {
        let mut splitted_vec = Vec::<Vec<u8>>::new();

        let mut idx_first = 0;
        for idx_next in vec![3, 7, 35, 5, 35, 7, 3] {
            let split = data[idx_first..(idx_next + idx_first)].to_vec();

            match split.len() {
                3 | 5 => { }
                7 => { splitted_vec.push(split); }
                _ => {
                    splitted_vec.append(
                        &mut split.chunks(7)
                            .map(|chunk| chunk.to_vec())
                            .collect::<Vec<Vec<u8>>>()
                    )
                }
            };
            
            idx_first += idx_next;
        };

        let mut decode_data = vec![BarCode::get_first_char(&splitted_vec)];

        let dec_chars = ENCODINGS.iter()
            .flat_map(|chr| chr.to_vec())
            .collect::<Vec<[u8; 7]>>();

        for char in splitted_vec {
            for (idxc, chr) in dec_chars.iter().enumerate() {
                if chr.to_vec() == char.clone() {
                    decode_data.push((idxc % 10) as u8);
                    break;
                }
            }
        }

        Ok(BarCode(decode_data[0..12].to_vec()))
    }

    fn get_first_char(vec: &Vec<Vec<u8>>) -> u8 {
        let mut decode_char = Vec::<u8>::new();
        let dec_chars = ENCODINGS.iter()
            .flat_map(|chr| chr.to_vec())
            .collect::<Vec<[u8; 7]>>();

        for char in vec[1..6].to_vec() {
            for (idxc, chr) in dec_chars.iter().enumerate() {
                if chr.to_vec() == char {
                    decode_char.push((idxc / 10) as u8);
                    break;
                }
            }
        };
        
        let mut first_char = 0u8;
        for (idx, par) in PARITY.iter().enumerate() {
            if (par.to_vec()) == decode_char {
                first_char = idx as u8;
                break;
            } 
        };

        first_char
    }

    pub fn encode(&self) -> Vec<u8> {
        (&[

            &[1, 0, 1][..],
            &ENCODINGS[0][self.0[1] as usize][..],
            &self.left_payload()[..],
            &[0, 1, 0, 1, 0][..],
            &self.right_payload()[..],
            &ENCODINGS[2][self.get_checksum() as usize][..],
            &[1, 0, 1][..]

        ][..] as &[&[u8]])
            .iter()
            .flat_map(|c| c.iter())
            .cloned()
            .collect()
    }

    fn left_payload(&self) -> Vec<u8> {
        let slices: Vec<[u8; 7]> = self
            .0[2..7]
            .iter()
            .zip(PARITY[self.0[0] as usize].iter())
            .map(|(d, s)| ENCODINGS[*s as usize][*d as usize])
            .collect();

        slices.iter()
            .flat_map(|b| b.into_iter())
            .cloned()
            .collect()
    }

    fn right_payload(&self) -> Vec<u8> {
        let slices: Vec<[u8; 7]> = self
            .0[7..]
            .iter()
            .map(|d| ENCODINGS[2][*d as usize])
            .collect();

        slices.iter()
            .flat_map(|b| b.into_iter())
            .cloned()
            .collect()
    }

    pub fn get_checksum(&self) -> u8 {
        let mut odds = 0;
        let mut evens = 0;

        for (i, d) in self.0.iter().enumerate() {
            match i % 2 {
                1 => odds += *d,
                _ => evens += *d,
            }
        }

        odds *= 3;

        match 10 - ((odds + evens) % 10) {
            10 => 0,
            n => n,
        }
    }
}