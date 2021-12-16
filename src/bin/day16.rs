use std::fs;

// An abstraction to allow us to keep track what we have parsed.
// Since the parsing can be recursive, it would be handy to abstract out this type.
struct ConsumableString {
    // This keeps track of where we've consumed
    idx: usize,
    // This is the wrapped string
    inner_str: String,
}

impl ConsumableString {
    // Reads n characters and updates the idx.
    fn consume(&mut self, n: usize) -> &str {
        let s = &self.inner_str[self.idx..self.idx + n];
        self.idx += n;
        s.clone()
    }

    // Parses the header.
    fn parse_header(&mut self) -> (i64, i32) {
        let packet_version = i64::from_str_radix(self.consume(3), 2).unwrap();
        let type_id = i32::from_str_radix(self.consume(3), 2).unwrap();
        return (packet_version, type_id);
    }

    // Returns the value of the evaluated packet, along with how large the packet was.
    // The latter value will help with recursive parsing as we know how much data to skip.
    fn parse(&mut self) -> (i64, usize) {
        // parse header
        let (packet_version, type_id) = self.parse_header();
        // let mut total_version = packet_version;
        if type_id == 4 {
            // literal
            let mut s = String::new();
            loop {
                let block = self.consume(5);
                s.push_str(&block[1..]);
                if block.chars().next().unwrap() == '0' {
                    break;
                }
            };
            (i64::from_str_radix(&s, 2).unwrap(), self.idx)
                // (packet_version, self.idx) // Use this for part 1
        } else {
            // Operator
            let length_type_id = self.consume(1);
            let sub_packets = match length_type_id {
                "0" => {
                    let i = to_int(self.consume(15)) as usize;
                    let mut packets: Vec<i64> = Vec::new();
                    let mut left = i;
                    while left > 0 {
                        let (x, tc) = ConsumableString {
                            idx: 0,
                            inner_str: String::from(&self.inner_str[self.idx..]),
                        }.parse();
                        left -= tc;
                        self.idx += tc;
                        packets.push(x)
                    }
                    packets
                }
                "1" => {
                    let num_sub_packets = to_int(self.consume(11));
                    let mut packets: Vec<i64> = Vec::new();
                    for _ in 0..num_sub_packets {
                        let (x, c) = ConsumableString {
                            idx: 0,
                            inner_str: String::from(&self.inner_str[self.idx.clone()..]),
                        }.parse();
                        self.idx += c;
                        packets.push(x);
                    }
                    packets
                }
                _ => unimplemented!("Not possible")
            };
            // (sub_packets.iter().sum::<i64>() + packet_version, self.idx) // Use this for part 1

            match type_id {
                0 => (sub_packets.iter().sum(), self.idx),
                1 => (sub_packets.iter().product(), self.idx),
                2 => (*sub_packets.iter().min().unwrap(), self.idx),
                3 => (*sub_packets.iter().max().unwrap(), self.idx),
                5 => {
                    let val = if sub_packets.first().unwrap() > sub_packets.last().unwrap() { 1 } else { 0 };
                    (val, self.idx)
                }
                6 => {
                    let val = if sub_packets.first().unwrap() < sub_packets.last().unwrap() { 1 } else { 0 };
                    (val, self.idx)
                }
                7 => {
                    let val = if sub_packets.first().unwrap() == sub_packets.last().unwrap() { 1 } else { 0 };
                    (val, self.idx)
                }
                _ => unimplemented!("Not possible")
            }
        }
    }
}


fn main() {
    let s = fs::read_to_string("./src/input16.txt").unwrap();
    let s: &str = s.trim();
    let binary_string = to_binary_string(s);
    let out = ConsumableString {
        idx: 0,
        inner_str: String::from(binary_string),
    }.parse();
    dbg!(out);
}

fn to_int(i: &str) -> i32 {
    i32::from_str_radix(i, 2).unwrap()
}

fn to_binary_string(s: &str) -> String {
    let mut binary_string = String::new();
    for x in s.chars() {
        match x {
            '0' => binary_string.push_str("0000"),
            '1' => binary_string.push_str("0001"),
            '2' => binary_string.push_str("0010"),
            '3' => binary_string.push_str("0011"),
            '4' => binary_string.push_str("0100"),
            '5' => binary_string.push_str("0101"),
            '6' => binary_string.push_str("0110"),
            '7' => binary_string.push_str("0111"),
            '8' => binary_string.push_str("1000"),
            '9' => binary_string.push_str("1001"),
            'A' => binary_string.push_str("1010"),
            'B' => binary_string.push_str("1011"),
            'C' => binary_string.push_str("1100"),
            'D' => binary_string.push_str("1101"),
            'E' => binary_string.push_str("1110"),
            'F' => binary_string.push_str("1111"),
            _ => panic!("unparsable.")
        }
    }
    binary_string
}
