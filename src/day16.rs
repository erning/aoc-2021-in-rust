fn parse_input(input: &str) -> String {
    input
        .trim()
        .chars()
        .map(|c| match c {
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
            _ => unreachable!(),
        })
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[derive(Debug)]
#[allow(dead_code)]
struct Packet {
    version: u8,
    type_id: u8,
    value: u64,
    children: Vec<Packet>,
}

impl Packet {
    fn read(data: &str, p: &mut usize, len: usize) -> u16 {
        let start = *p;
        *p += len;
        u16::from_str_radix(&data[start..*p], 2).unwrap()
    }

    fn read_literal_value(data: &str, p: &mut usize) -> u64 {
        let mut value: u64 = 0;
        loop {
            let g = Self::read(data, p, 5);
            let v = g & 0x0F;
            value = value << 4 | v as u64;
            if g <= 0x0F {
                break;
            }
        }
        value
    }

    fn read_children(data: &str, p: &mut usize) -> Vec<Packet> {
        let mut children: Vec<Packet> = Vec::new();
        let len_id = Self::read(data, p, 1) as u8;
        if len_id == 0 {
            let len = Self::read(data, p, 15) as usize;
            let end = *p + len;
            while *p < end {
                children.push(Self::read_packet(data, p));
            }
        } else {
            let len = Self::read(data, p, 11) as usize;
            for _ in 0..len {
                children.push(Self::read_packet(data, p));
            }
        }
        children
    }

    fn read_packet(data: &str, p: &mut usize) -> Packet {
        let version = Self::read(data, p, 3) as u8;
        let type_id = Self::read(data, p, 3) as u8;
        if type_id == 4 {
            let value = Self::read_literal_value(data, p);
            return Packet {
                version,
                type_id,
                value,
                children: Vec::new(),
            };
        }

        let children = Self::read_children(data, p);
        let subvalues: Vec<u64> = children.iter().map(|p| p.value).collect();
        #[rustfmt::skip]
        let value: u64 = match type_id {
            0 => subvalues.into_iter().sum(),
            1 => subvalues.into_iter().product(),
            2 => subvalues.into_iter().min().unwrap(),
            3 => subvalues.into_iter().max().unwrap(),
            4 => unreachable!(),
            5 => if subvalues[0] > subvalues[1]  { 1 } else { 0 },
            6 => if subvalues[0] < subvalues[1]  { 1 } else { 0 },
            7 => if subvalues[0] == subvalues[1] { 1 } else { 0 },
            _ => unreachable!(),
        };

        Packet {
            version,
            type_id,
            value,
            children,
        }
    }

    fn from_binary_str(data: &str) -> Packet {
        let mut p = 0;
        Self::read_packet(data, &mut p)
    }

    fn from_hex_str(hex: &str) -> Packet {
        let data = parse_input(hex);
        Self::from_binary_str(&data)
    }

    fn sum_of_versions(&self) -> usize {
        let mut sum = self.version as usize;
        for packet in self.children.iter() {
            sum += packet.sum_of_versions();
        }
        sum
    }
}

pub fn part_one(input: &str) -> usize {
    Packet::from_hex_str(input).sum_of_versions()
}

pub fn part_two(input: &str) -> u64 {
    Packet::from_hex_str(input).value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decoder() {
        let packet = Packet::from_hex_str("D2FE28");
        assert_eq!(packet.value, 2021);
        let packet = Packet::from_hex_str("38006F45291200");
        assert_eq!(packet.children.len(), 2);
        let packet = Packet::from_hex_str("EE00D40C823060");
        assert_eq!(packet.children.len(), 3);
    }

    #[test]
    fn example_one() {
        assert_eq!(part_one("8A004A801A8002F478"), 16);
        assert_eq!(part_one("620080001611562C8802118E34"), 12);
        assert_eq!(part_one("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part_one("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn example_two() {
        assert_eq!(part_two("C200B40A82"), 3);
        assert_eq!(part_two("04005AC33890"), 54);
        assert_eq!(part_two("880086C3E88112"), 7);
        assert_eq!(part_two("CE00C43D881120"), 9);
        assert_eq!(part_two("D8005AC2A8F0"), 1);
        assert_eq!(part_two("F600BC2D8F"), 0);
        assert_eq!(part_two("9C005AC2F8F0"), 0);
        assert_eq!(part_two("9C0141080250320F1802104A08"), 1);
    }
}
