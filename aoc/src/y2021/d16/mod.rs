pub fn main(input: &str) -> String {
    let bits = input_to_bits(input);
    let (_, root) = Packet::from_bits(&bits);
    let a = root.version_sum();
    let b = root.eval();
    format!("{} {}", a, b)
}

fn input_to_bits(input: &str) -> Vec<u8> {
    input
        .chars()
        .flat_map(|c| {
            let s = c.to_string();
            let mut x = u8::from_str_radix(&s, 16).unwrap();
            let mut bits = vec![0; 4];
            for b in bits.iter_mut().rev() {
                *b = x & 1;
                x >>= 1;
            }
            bits
        })
        .collect()
}

fn bits_to_num(bits: &[u8]) -> usize {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(i, &b)| (b as usize) << i)
        .sum()
}

#[derive(Clone, Debug)]
struct Packet {
    version: usize,
    data:    usize,
    op:      Op,
    packets: Packets,
}

#[derive(Clone, Debug, PartialEq)]
enum Op {
    Sum,
    Prod,
    Min,
    Literal,
    Max,
    Gt,
    Lt,
    Eq,
}

type Packets = Vec<Box<Packet>>;

impl Packet {
    fn new(version: usize, op: usize) -> Self {
        let op = match op {
            0 => Op::Sum,
            1 => Op::Prod,
            2 => Op::Min,
            4 => Op::Literal,
            3 => Op::Max,
            5 => Op::Gt,
            6 => Op::Lt,
            7 => Op::Eq,
            _ => panic!("unknown op {}", op),
        };
        Self {
            version,
            data: 0,
            op,
            packets: vec![],
        }
    }

    fn from_bits(bits: &[u8]) -> (usize, Self) {
        let (version, bits) = (bits_to_num(&bits[..3]), &bits[3..]);
        let (p_type, bits) = (bits_to_num(&bits[..3]), &bits[3..]);
        let mut packet = Self::new(version, p_type);
        let num_bits_read = if packet.op == Op::Literal {
            packet.parse_literal_data(bits)
        } else {
            packet.parse_subpackets(bits)
        };
        (6 + num_bits_read, packet)
    }

    fn parse_literal_data(&mut self, bits: &[u8]) -> usize {
        let mut n = 0;
        let mut data = vec![];
        for chunk in bits.chunks(5) {
            data.extend(&chunk[1..]);
            n += 5;
            if chunk[0] == 0 {
                break;
            }
        }
        self.data = bits_to_num(&data);
        n
    }

    fn parse_subpackets(&mut self, bits: &[u8]) -> usize {
        let (len_type, bits) = (bits[0], &bits[1..]);
        let header_len = match len_type {
            0 => 15,
            1 => 11,
            _ => panic!("unknown length type {}", len_type),
        };
        let limit = bits_to_num(&bits[..header_len]);
        let bits = &bits[header_len..];
        let mut pos = 0;
        while (len_type == 0 && pos < limit)
            || (len_type == 1 && self.packets.len() < limit)
        {
            let (n, packet) = Self::from_bits(&bits[pos..]);
            self.packets.push(Box::new(packet));
            pos += n;
        }
        1 + header_len + pos
    }

    fn version_sum(&self) -> usize {
        let subpacket_sum: usize =
            self.packets.iter().map(|p| p.version_sum()).sum();
        self.version + subpacket_sum
    }

    fn eval(&self) -> usize {
        let packets = self.packets.iter().map(|p| p.eval());
        let without_first = packets.clone().skip(1);
        let n_sub_1 = self.packets.len().saturating_sub(1);
        let without_last = packets.clone().take(n_sub_1);
        match self.op {
            Op::Sum => packets.sum(),
            Op::Prod => packets.product(),
            Op::Min => packets.min().unwrap(),
            Op::Literal => self.data,
            Op::Max => packets.max().unwrap(),
            Op::Gt => without_last.gt(without_first) as usize,
            Op::Lt => without_last.lt(without_first) as usize,
            Op::Eq => without_last.eq(without_first) as usize,
        }
    }
}
