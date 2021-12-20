use std::{fmt, usize};
use std::fmt::{Formatter, Write};
use std::fs::File;
use std::io::Read;

struct Packet {
    version : u8,
    id : u8,
    bytes : String,
}

fn determine_packet_end(bytes: &str) -> usize {
    let version = u8::from_str_radix(&bytes[0..3], 2).unwrap();
    let id = u8::from_str_radix(&bytes[3..6], 2).unwrap();

    let mut packet_len = 6;

    if id == 4 {
        while bytes.chars().nth(packet_len).unwrap() != '0' {
            packet_len += 5;
        }
        packet_len += 5;
        //packet_len = (f32::ceil(packet_len as f32 / 4.0f32) as usize) * 4;
    }
    else {
        if bytes.chars().nth(packet_len).unwrap() == '0' {
            packet_len += 1;
            packet_len += usize::from_str_radix(&bytes[packet_len..packet_len+15], 2).unwrap() + 15;
        }
        else {
            packet_len += 1;
            let sub_packets_count = usize::from_str_radix(&bytes[packet_len..packet_len+11], 2).unwrap();
            packet_len += 11;
            for i in 0..sub_packets_count {
                packet_len += determine_packet_end(&bytes[packet_len..]);
            }
        }
    }
    packet_len
}

impl Packet {
    fn new(bytes: String) -> Self {
        let version = u8::from_str_radix(&bytes[0..3], 2).unwrap();
        let id = u8::from_str_radix(&bytes[3..6], 2).unwrap();

        Packet{ version,
                id,
                bytes : bytes[6..].to_string() }
    }

    fn get_packet_len(&self) -> usize {
        self.bytes.len() + 6
    }

    fn get_literal(&self) -> Option<u64> {
        if self.id == 4 {
            let mut i = 0;
            let mut ret_val = String::new();
            loop {
                let chunk = &self.bytes[i..i+5];
                ret_val.push_str(&chunk[1..5]);
                if chunk.chars().nth(0).unwrap() == '0' {
                    return Some(u64::from_str_radix(&ret_val, 2).unwrap());
                }
                i += 5;
            }
        }
        None
    }

    fn get_subpackets(&self) -> Option<Vec<Packet>> {
        if self.id != 4 {
            let mut ret_val = Vec::<Packet>::new();
            if self.bytes.chars().nth(0).unwrap() == '0' {
                let sub_packets_bits_count = usize::from_str_radix(&self.bytes[1..16], 2).unwrap();
                let mut starting_idx = 16;
                while starting_idx < sub_packets_bits_count+16 {
                    let end_idx = determine_packet_end(&self.bytes[starting_idx..]) + starting_idx;
                    ret_val.push(Packet::new(self.bytes[starting_idx..end_idx].to_string()));
                    starting_idx = end_idx;
                }
            }
            else {
                let sub_packets_num = u64::from_str_radix(&self.bytes[1..12], 2).unwrap();
                let mut starting_idx = 12;
                for i in 0..sub_packets_num {
                    let end_idx = determine_packet_end(&self.bytes[starting_idx..]) + starting_idx;
                    ret_val.push(Packet::new(self.bytes[starting_idx..end_idx].to_string()));
                    starting_idx = end_idx;
                }
            }
            return Some(ret_val);
        }
        None
    }

}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.bytes)
    }
}

fn hex_to_bin_str(c : char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("Bad character")
    }
}

fn read_input_from_file() -> Packet {
    let mut file = File::open("input.txt").unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content);

    let mut byte_str = String::new();
    for ch in content.chars() {
        byte_str.push_str(hex_to_bin_str(ch));
    }

    Packet::new(byte_str)
}

fn get_version_number_sum(packet : &Packet) -> u64 {
    let mut sum : u64 = packet.version as u64;
    if let Some(subpackets) = packet.get_subpackets() {
        for subpacket in subpackets {
            sum += get_version_number_sum(&subpacket);
        }
    }
    sum
}

fn get_expression(packet : &Packet) -> i64 {
    if let Some(subpackets) = packet.get_subpackets() {
        let red_fun = match packet.id {
            0 => std::ops::Add::add,
            1 => std::ops::Mul::mul,
            2 => std::cmp::min,
            3 => std::cmp::max,
            5 => |a,b| if a-b > 0 {1} else {0},
            6 => |a,b| if a-b < 0 {1} else {0},
            7 => |a,b| if a==b {1} else {0},
            _ => panic!("Unrecognized id")
        };
        return subpackets.iter().map(|s| get_expression(s.clone())).reduce(red_fun).unwrap();
    }
    else {
        return packet.get_literal().unwrap() as i64;
    }
}


fn main() {
    let packet = read_input_from_file();
    dbg!(get_version_number_sum(&packet));
    dbg!(get_expression(&packet));
}