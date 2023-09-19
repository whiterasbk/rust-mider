use crate::midi::constance::byte;
use crate::midi::util::hex;

pub fn vec_put_array(buffer: &mut Vec<byte>, array: &[byte]) {
    buffer.append(Vec::from(array).as_mut());
}

pub fn u32_as_4l_byte_array(a: u32) -> [byte; 4] {
    let mut bytes: [byte; 4] = [0, 0, 0, 0];
    bytes[0] = (a >> 24 & 0xff) as byte;
    bytes[1] = (a >> 16 & 0xff) as byte;
    bytes[2] = (a >> 8 & 0xff) as byte;
    bytes[3] = a as byte;
    return bytes
}

pub fn u32_as_2l_byte_array(a: u32) -> [byte; 2] {
    let mut bytes: [byte; 2] = [0, 0];
    bytes[0] = (a >> 8 & 0xff) as byte;
    bytes[1] = a as byte;
    return bytes
}

pub fn u32_as_vl_vec(a: u32) -> Vec<byte> {
    match a {
        0 => vec![0],

        0 ..= 0x7f => {
            vec![a as byte]
        }

        0x80 ..= 0x3fff => {
            vec![
                (a >> 7 | 0b1000_0000u32) as byte,
                (a & 0b0111_1111u32) as byte
            ]
        }

        0x4000 ..= 0x1fffff => {
            vec![
                (a >> 14 | 0b1000_0000u32) as byte,
                (a >>  7 | 0b1000_0000u32) as byte,
                (a & 0b0111_1111u32) as byte
            ]
        }

        0x200000 ..= 0xfffffff => {
            vec![
                (a >> 21 | 0b1000_0000u32) as byte,
                (a >> 14 | 0b1000_0000u32) as byte,
                (a >>  7 | 0b1000_0000u32) as byte,
                (a & 0b0111_1111u32) as byte
            ]
        }

        _ => panic!("Exception(\"out of stack: $this, only 4 bytes allow\")")
    }
}

pub fn u32_auto_vec(num: u32) -> Vec<byte> {
    let arr = u32_as_4l_byte_array(num);
    let u0 = 0u8;

    if arr[0] == 0 {
        if arr[1] == 0 {
            if arr[2] == 0 {
                if arr[3] == 0 { vec![0] } else { vec![arr[3]] }
            } else {
                vec![arr[2], arr[3]]
            }
        } else {
            vec![arr[1], arr[2], arr[3]]
        }
    } else {
        vec![arr[0], arr[1], arr[2], arr[3]]
    }
}

pub fn bpm2tempo_vec(bpm: u32) -> Vec<byte> { u32_auto_vec(hex::bpm2tempo(bpm)) }
