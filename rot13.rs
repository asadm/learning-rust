use std::io::Read;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

impl <R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // iterate over each byte and +rot it
        let read = self.input.read(buf)?;
        for i in 0..read {
            if buf[i].is_ascii_alphabetic() {
                if buf[i] + self.rot > b'z' {
                    buf[i] = buf[i].wrapping_sub(26) + self.rot;
                } else {
                    buf[i] = buf[i].wrapping_add(self.rot);
                }
            }
        }
        Ok(read)
    }
}


fn main() {
    let mut rot =
        RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();
    println!("{}", result);
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn joke() {
//         let mut rot =
//             RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
//         let mut result = String::new();
//         rot.read_to_string(&mut result).unwrap();
//         assert_eq!(&result, "To get to the other side!");
//     }

//     #[test]
//     fn binary() {
//         let input: Vec<u8> = (0..=255u8).collect();
//         let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
//         let mut buf = [0u8; 256];
//         assert_eq!(rot.read(&mut buf).unwrap(), 256);
//         for i in 0..=255 {
//             if input[i] != buf[i] {
//                 assert!(input[i].is_ascii_alphabetic());
//                 assert!(buf[i].is_ascii_alphabetic());
//             }
//         }
//     }
// }