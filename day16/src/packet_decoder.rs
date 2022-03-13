use bitvec::prelude::*;
use std::cmp::PartialEq;
use std::fmt;
use std::iter::zip;

pub fn hex_str_to_bitvec(input: &str) -> BitVec<Msb0, u8> {
    let mut buffer: BitVec<Msb0, u8> = BitVec::new();
    input.as_bytes().iter().for_each(|c| {
        if *c <= ('9' as u8) {
            let parsed = c - ('0' as u8);
            buffer.append(&mut BitVec::<Msb0, u8>::from_element(parsed)[4..].to_bitvec());
        } else if *c >= ('A' as u8) && *c <= ('F' as u8) {
            let parsed = c - ('A' as u8) + 10;
            buffer.append(&mut BitVec::<Msb0, u8>::from_element(parsed)[4..].to_bitvec());
        } else {
            panic! {"Invalid Hex character in input string: {:?}", *c as char}
        }
    });
    buffer
}

#[derive(Debug)]
pub struct Packet {
    version: u8,
    type_id: u8,
    packet_type: PacketType,
}

#[derive(Debug, PartialEq)]
enum PacketType {
    OperatorPacket(Vec<Packet>),
    LiteralValue(u64),
}

impl Packet {
    pub fn new(buffer: &BitSlice<Msb0, u8>, mut index: &mut usize) -> Self {
        const VERSION_BITS: usize = 3;
        const TYPE_BITS: usize = 3;
        const LENGTH_TYPE_ID_BIT: usize = 7;
        const CHUNK_SIZE: usize = 5;

        // Find out version
        let start_index = *index;
        let version_start_idx = start_index;
        let version_end_idx = start_index + VERSION_BITS;
        let version = &buffer[version_start_idx..version_end_idx]
            .to_bitvec()
            .load_be::<u8>()
            .clone();
        // Find out type
        let type_id_start_idx = version_end_idx;
        let type_id_end_idx = type_id_start_idx + TYPE_BITS;
        let type_id = &buffer[type_id_start_idx..type_id_end_idx]
            .to_bitvec()
            .load_be::<u8>()
            .clone();
        let data_start_idx = type_id_end_idx;
        println! {
            "DEBUG: Parsing packet at index {:?} (ver={:?}, type={:?}, data_start_idx={:?}) rest={:?}",
            start_index, version, type_id, data_start_idx,
            &buffer[*index..&buffer.last_one().unwrap()+1]
        };

        // All packets with type other than 4 are operator packets
        const TYPE_LITERAL_VALUE: u8 = 4;
        match *type_id {
            TYPE_LITERAL_VALUE => {
                println! {"DEBUG: Collecting data of literal value packet starting at {:?}", data_start_idx};
                let mut val: u64 = 0;
                let mut chunk_idx = data_start_idx;
                // Optionally iterate in chunks until the end of all data because
                // size of literal value is not known beforehand
                for c in
                    buffer[data_start_idx..buffer.last_one().unwrap() + 1].chunks_exact(CHUNK_SIZE)
                {
                    // Bit shifts val left by 4 bits and adds new chunk
                    //      0111 (0x7) * 16   -> 0111 0000 (0x70)
                    // 0111 0000 (0x70) + 0x6 -> 0111 0110 (0x67)
                    val = val * 16 + c[1..CHUNK_SIZE].load_be::<u64>();
                    // Update index to be at end of current chunk
                    chunk_idx += CHUNK_SIZE;
                    // Check first bit if last chunk
                    if c[0] == false {
                        break;
                    }
                }
                // Update index to be at end of this data frame
                *index = chunk_idx;
                Self {
                    version: *version,
                    type_id: *type_id,
                    packet_type: PacketType::LiteralValue(val),
                }
            }
            _ => {
                //   1   6                27 ||   6   4     10 |   2   4           20 || padding
                // 001 110 0 000000000011011 || 110 100  01010 | 010 100  10001 00100 || 0000000
                // VVV TTT I LLLLLLLLLLLLLLL || AAA AAA  AAAAA | BBB BBB  BBBBB BBBBB || padding
                //           len subpackets
                println! {"DEBUG: Operator packet data parsing start at {:?}:\n\t{:?}", *index, buffer};
                // Find length type
                let length_type = buffer[data_start_idx].clone();
                let length_start_idx = data_start_idx + 1;

                const LENGTH_TYPE_NUM_SUBPACKAGES: usize = 11;
                const LENGTH_TYPE_NUM_BITS: usize = 15;

                if length_type {
                    // Number of subpackets is known
                    let subpacket_num_end_idx = length_start_idx + LENGTH_TYPE_NUM_SUBPACKAGES;
                    let subpacket_num_raw =
                        buffer[length_start_idx..subpacket_num_end_idx].to_bitvec();
                    let subpacket_num = subpacket_num_raw.load_be::<u32>().clone();
                    println! {"DEBUG: Parsing operator with num subpackets known: raw={:?} num={:?}",
                        subpacket_num_raw,
                        subpacket_num
                    };
                    // Move index to start of subpackets
                    let subpacket_start_idx = subpacket_num_end_idx;
                    // Try to create the specified number of subpackets
                    let mut subpackets = Vec::new();
                    let mut subpacket_current_idx = subpacket_start_idx;
                    for i in [0..subpacket_num] {
                        let subpacket = Packet::new(buffer, &mut subpacket_current_idx);
                        subpackets.push(subpacket);
                    }
                    // Update index to be at end of subpackages
                    *index = subpacket_current_idx;
                    Self {
                        version: *version,
                        type_id: *type_id,
                        packet_type: PacketType::OperatorPacket(subpackets),
                    }
                } else {
                    // Number of bits is known
                    let length_end_idx = length_start_idx + LENGTH_TYPE_NUM_BITS;
                    let subpacket_num_bits_raw =
                        buffer[length_start_idx..length_end_idx].to_bitvec();
                    let subpacket_num_bits = subpacket_num_bits_raw.load_be::<u32>().clone();
                    // Move index to start of subpackets
                    let subpacket_start_idx = length_end_idx;
                    // Figure out index of last data bit
                    let subpackets_end_idx = subpacket_start_idx + subpacket_num_bits as usize;
                    println! {"DEBUG: Parsing operator with num bits known: num_bits_raw={:?} num_bits={:?} start={:?} end={:?}",
                        subpacket_num_bits_raw,
                        subpacket_num_bits,
                        subpacket_start_idx,
                        subpackets_end_idx,
                    };

                    // Try to create subpackets while in range of subpacket bits
                    let mut subpackets = Vec::new();
                    let mut subpacket_current_idx = subpacket_start_idx;
                    while subpacket_current_idx <= subpackets_end_idx {
                        let subpacket = Packet::new(buffer, &mut subpacket_current_idx);
                        subpackets.push(subpacket);
                    }

                    // Update index to be at end of subpackages
                    *index = subpackets_end_idx;
                    Self {
                        version: *version,
                        type_id: *type_id,
                        packet_type: PacketType::OperatorPacket(subpackets),
                    }
                }
            }
            _ => {
                panic! {"Unknown type in buffer: {:?}", *type_id}
            }
        }
    }

    pub fn from_hex_str(packet_string: &str) -> Self {
        Packet::new(&mut hex_str_to_bitvec(packet_string), &mut 0)
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        let mut equal = self.version == other.version && self.type_id == other.type_id;
        if equal {
            match &self.packet_type {
                PacketType::LiteralValue(val) => match other.packet_type {
                    PacketType::LiteralValue(other_val) => equal &= *val == other_val,
                    _ => {
                        equal = false;
                    }
                },
                PacketType::OperatorPacket(subpackets) => {
                    match &other.packet_type {
                        PacketType::OperatorPacket(other_subpackets) => {
                            // Recursively iterate all subpackets and check for equality
                            equal = subpackets.len() == other_subpackets.len();
                            if equal {
                                for (sp, o_sp) in zip(subpackets, other_subpackets) {
                                    equal &= sp == o_sp;
                                    if !equal {
                                        break;
                                    }
                                }
                            }
                        }
                        _ => {
                            equal = false;
                        }
                    }
                }
                _ => {
                    panic! {"Could not compare packets because packet type {:?} is not implemented yet", self.packet_type}
                }
            }
        }
        equal
    }
}

#[cfg(test)]
mod tests {
    use crate::packet_decoder::{hex_str_to_bitvec, Packet, PacketType};
    use bitvec::prelude::*;
    use std::fs;

    const packet_literal_2021_hex: &str = "D2FE28";
    const packet_literal_2021_bin: &str = "110100101111111000101000";
    const packet_operator_0_hex: &str = "38006F45291200";
    const packet_operator_0_bin: &str = "00111000000000000110111101000101001010010001001000000000";
    const packet_operator_1_hex: &str = "EE00D40C823060";
    const packet_operator_1_bin: &str = "11101110000000001101010000001100100000100011000001100000";

    #[test]
    #[should_panic]
    fn test_parse_invalid_input_to_bitvec() {
        // Non-hex character
        let non_hex = "ZZ";
        let non_bitvec: BitVec<Msb0, u8> = BitVec::new();
        assert_eq!(hex_str_to_bitvec(non_hex), non_bitvec);
    }

    #[test]
    fn test_parse_valid_input_to_bitvec() {
        // Simple
        let simple_hex = "6A6A";
        let simple_bitvec: BitVec = bitvec![0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0];
        assert_eq!(hex_str_to_bitvec(simple_hex), simple_bitvec);

        // Zero
        let zero_hex = "0";
        let zero_bitvec: BitVec = bitvec![0, 0, 0, 0];
        assert_eq!(hex_str_to_bitvec(zero_hex), zero_bitvec);

        // Literal
        let packet_literal_2021_bitvec: BitVec =
            bitvec![1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0];
        assert_eq!(
            hex_str_to_bitvec(packet_literal_2021_hex),
            packet_literal_2021_bitvec
        );

        // Operator 0
        let packet_operator_0_bitvec: BitVec = bitvec![
            0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0,
            1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ];
        assert_eq!(
            hex_str_to_bitvec(packet_operator_0_hex),
            packet_operator_0_bitvec
        );

        // Operator 1
        let packet_operator_1_bitvec: BitVec = bitvec![
            1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0
        ];
        assert_eq!(
            hex_str_to_bitvec(packet_operator_1_hex),
            packet_operator_1_bitvec
        );
    }

    #[test]
    fn test_packet_equality_literal_values() {
        let packet_literal_2021_struct: Packet = Packet {
            version: 6, // 110
            type_id: 4, // 100
            // [1]0111 + [1]1110 + [0]0101 -> 011111100101  First bit controls if end
            packet_type: PacketType::LiteralValue(2021),
        };
        let packet_literal_6666_struct: Packet = Packet {
            version: 6, // 110
            type_id: 4, // 100
            // [1]0111 + [1]1110 + [0]0101 -> 011111100101  First bit controls if end
            packet_type: PacketType::LiteralValue(6666),
        };
        assert_eq!(
            packet_literal_2021_struct == packet_literal_2021_struct,
            true
        );
        assert_eq!(
            packet_literal_6666_struct == packet_literal_6666_struct,
            true
        );
        assert_eq!(
            packet_literal_2021_struct == packet_literal_6666_struct,
            false
        );
    }

    #[test]
    fn test_packet_equality_operator_and_literal_value() {
        let packet_literal_2021_struct: Packet = Packet {
            version: 6, // 110
            type_id: 4, // 100
            // [1]0111 + [1]1110 + [0]0101 -> 011111100101  First bit controls if end
            packet_type: PacketType::LiteralValue(2021),
        };
        let packet_operator_0_struct: Packet = Packet {
            version: 1, // 001
            type_id: 6, // 110
            packet_type: PacketType::OperatorPacket(vec![
                Packet {
                    version: 6,
                    type_id: 4,
                    packet_type: PacketType::LiteralValue(10),
                },
                Packet {
                    version: 6,
                    type_id: 4,
                    packet_type: PacketType::LiteralValue(20),
                },
            ]),
        };
        assert_eq!(
            packet_literal_2021_struct == packet_operator_0_struct,
            false
        );
    }

    #[test]
    fn test_packet_equality_operator_and_operator_simple() {
        let packet_operator_0_struct: Packet = Packet {
            version: 1, // 001
            type_id: 6, // 110
            packet_type: PacketType::OperatorPacket(vec![
                Packet {
                    version: 6,
                    type_id: 4,
                    packet_type: PacketType::LiteralValue(10),
                },
                Packet {
                    version: 6,
                    type_id: 4,
                    packet_type: PacketType::LiteralValue(20),
                },
            ]),
        };
        let packet_operator_other_struct: Packet = Packet {
            version: 1, // 111
            type_id: 6, // 011
            packet_type: PacketType::OperatorPacket(vec![
                Packet {
                    version: 6,
                    type_id: 4,
                    packet_type: PacketType::LiteralValue(60),
                },
                Packet {
                    version: 6,
                    type_id: 4,
                    packet_type: PacketType::LiteralValue(80),
                },
            ]),
        };
        let packet_operator_other_longer_struct: Packet = Packet {
            version: 1, // 111
            type_id: 6, // 011
            packet_type: PacketType::OperatorPacket(vec![
                Packet {
                    version: 6,
                    type_id: 4,
                    packet_type: PacketType::LiteralValue(60),
                },
                Packet {
                    version: 6,
                    type_id: 4,
                    packet_type: PacketType::LiteralValue(80),
                },
                Packet {
                    version: 6,
                    type_id: 4,
                    packet_type: PacketType::LiteralValue(90),
                },
            ]),
        };
        assert_eq!(packet_operator_0_struct == packet_operator_0_struct, true);
        assert_eq!(
            packet_operator_other_struct == packet_operator_other_struct,
            true
        );
        assert_eq!(
            packet_operator_other_longer_struct == packet_operator_other_longer_struct,
            true
        );
        assert_eq!(
            packet_operator_0_struct == packet_operator_other_struct,
            false
        );
        assert_eq!(
            packet_operator_0_struct == packet_operator_other_longer_struct,
            false
        );
        assert_eq!(
            packet_operator_other_struct == packet_operator_other_longer_struct,
            false
        );
    }

    #[test]
    fn test_packet_equality_operator_and_operator_complex() {
        let packet_operator_0_struct: Packet = Packet {
            version: 1,
            type_id: 6,
            packet_type: PacketType::OperatorPacket(vec![
                Packet {
                    version: 6,
                    type_id: 4,
                    packet_type: PacketType::LiteralValue(10),
                },
                Packet {
                    version: 6,
                    type_id: 4,
                    packet_type: PacketType::LiteralValue(20),
                },
            ]),
        };
        let packet_operator_nested_struct: Packet = Packet {
            version: 4,
            type_id: 5,
            packet_type: PacketType::OperatorPacket(vec![Packet {
                version: 1,
                type_id: 5,
                packet_type: PacketType::OperatorPacket(vec![Packet {
                    version: 1,
                    type_id: 5,
                    packet_type: PacketType::OperatorPacket(vec![Packet {
                        version: 6,
                        type_id: 4,
                        packet_type: PacketType::LiteralValue(20),
                    }]),
                }]),
            }]),
        };
        let packet_operator_double_nested_0_struct: Packet = Packet {
            version: 3,
            type_id: 5,
            packet_type: PacketType::OperatorPacket(vec![
                Packet {
                    version: 1,
                    type_id: 5,
                    packet_type: PacketType::OperatorPacket(vec![
                        Packet {
                            version: 6,
                            type_id: 4,
                            packet_type: PacketType::LiteralValue(80),
                        },
                        Packet {
                            version: 6,
                            type_id: 4,
                            packet_type: PacketType::LiteralValue(90),
                        },
                    ]),
                },
                Packet {
                    version: 1,
                    type_id: 5,
                    packet_type: PacketType::OperatorPacket(vec![
                        Packet {
                            version: 6,
                            type_id: 4,
                            packet_type: PacketType::LiteralValue(90),
                        },
                        Packet {
                            version: 6,
                            type_id: 4,
                            packet_type: PacketType::LiteralValue(60),
                        },
                    ]),
                },
            ]),
        };
        let packet_operator_double_nested_1_struct: Packet = Packet {
            version: 3,
            type_id: 5,
            packet_type: PacketType::OperatorPacket(vec![
                Packet {
                    version: 1,
                    type_id: 5,
                    packet_type: PacketType::OperatorPacket(vec![
                        Packet {
                            version: 6,
                            type_id: 4,
                            packet_type: PacketType::LiteralValue(1),
                        },
                        Packet {
                            version: 6,
                            type_id: 4,
                            packet_type: PacketType::LiteralValue(2),
                        },
                    ]),
                },
                Packet {
                    version: 1,
                    type_id: 5,
                    packet_type: PacketType::OperatorPacket(vec![
                        Packet {
                            version: 6,
                            type_id: 4,
                            packet_type: PacketType::LiteralValue(3),
                        },
                        Packet {
                            version: 6,
                            type_id: 4,
                            packet_type: PacketType::LiteralValue(4),
                        },
                    ]),
                },
            ]),
        };
        // Self equalities
        assert_eq!(packet_operator_0_struct == packet_operator_0_struct, true);
        assert_eq!(
            packet_operator_nested_struct == packet_operator_nested_struct,
            true
        );
        assert_eq!(
            packet_operator_double_nested_0_struct == packet_operator_double_nested_0_struct,
            true
        );
        assert_eq!(
            packet_operator_double_nested_1_struct == packet_operator_double_nested_1_struct,
            true
        );
        // Inequality with others
        assert_eq!(
            packet_operator_0_struct == packet_operator_nested_struct,
            false
        );
        assert_eq!(
            packet_operator_0_struct == packet_operator_double_nested_0_struct,
            false
        );
        assert_eq!(
            packet_operator_0_struct == packet_operator_double_nested_1_struct,
            false
        );
        assert_eq!(
            packet_operator_nested_struct == packet_operator_double_nested_0_struct,
            false
        );
        assert_eq!(
            packet_operator_nested_struct == packet_operator_double_nested_1_struct,
            false
        );
        assert_eq!(
            packet_operator_double_nested_0_struct == packet_operator_double_nested_1_struct,
            false
        );
    }

    #[test]
    fn test_create_literal_packet() {
        let packet_literal_2021_struct: Packet = Packet {
            version: 6, // 110
            type_id: 4, // 100
            // [1]0111 + [1]1110 + [0]0101 -> 011111100101  First bit controls if end
            packet_type: PacketType::LiteralValue(2021),
        };
        let packet_literal_2021_bitvec = hex_str_to_bitvec(packet_literal_2021_hex);
        assert_eq!(
            Packet::new(&packet_literal_2021_bitvec, &mut 0),
            packet_literal_2021_struct
        );
        assert_eq!(
            Packet::from_hex_str(packet_literal_2021_hex),
            packet_literal_2021_struct
        );
    }

    #[test]
    fn test_create_operator_packet_with_two_literal_value_packets() {
        let packet_operator_0_struct: Packet = Packet {
            version: 1, // 001
            type_id: 6, // 110
            packet_type: PacketType::OperatorPacket(vec![
                Packet {
                    version: 6, // 110
                    type_id: 4, // 100
                    packet_type: PacketType::LiteralValue(10),
                },
                Packet {
                    version: 2, // 110
                    type_id: 4, // 100
                    packet_type: PacketType::LiteralValue(20),
                },
            ]),
        };
        assert_eq!(
            Packet::from_hex_str(packet_operator_0_hex),
            packet_operator_0_struct
        );
    }

    // #[test]
    // fn test_create_operator_packet_with_multiple_literal_value_packets() {
    //     let packet_operator_1_struct: Packet = Packet {
    //         version: 7, // 111
    //         type_id: 3, // 011
    //         packet_type: PacketType::OperatorPacket(vec![
    //             Packet {
    //                 version: 2, // 010
    //                 type_id: 4, // 100
    //                 packet_type: PacketType::LiteralValue(1),
    //             },
    //             Packet {
    //                 version: 4, // 100
    //                 type_id: 4, // 100
    //                 packet_type: PacketType::LiteralValue(2),
    //             },
    //             Packet {
    //                 version: 1, // 001
    //                 type_id: 4, // 100
    //                 packet_type: PacketType::LiteralValue(3),
    //             },
    //         ]),
    //     };
    //     assert_eq!(
    //         Packet::from_hex_str(packet_operator_1_hex),
    //         packet_operator_1_struct
    //     );
    // }
}
