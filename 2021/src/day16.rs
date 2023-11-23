use std::error::Error;
use std::fmt;
use std::str::FromStr;

struct Parser<'a> {
    bits: &'a str,
    cursor: usize,
    state: ParserState,
}

#[derive(Debug, PartialEq)]
enum ParserState {
    ParseVersion,
    ParseTypeID,
    ParseLengthTypeID,
    ParseLength,
    ParseLiteralValue,
    ParseSubPacketsByLength(usize),
    ParseSubPacketsByCount(usize),
    CalculateOperationValue,
    Finished,
}

#[derive(Debug, PartialEq)]
enum PacketType {
    Unknown,
    Literal,
    Operator(OperatorType),
}

#[derive(Debug)]
enum LengthType {
    Length = 0,
    Count = 1,
    Unknown = 2,
}

impl From<usize> for LengthType {
    fn from(value: usize) -> LengthType {
        match value {
            0 => LengthType::Length,
            1 => LengthType::Count,
            2 => LengthType::Unknown,
            _ => panic!("Unknown length type value: {}", value),
        }
    }
}

#[derive(Debug, PartialEq)]
enum OperatorType {
    Sum = 0,
    Product = 1,
    Min = 2,
    Max = 3,
    Literal = 4,
    Gt = 5,
    Lt = 6,
    Eq = 7,
}

impl From<usize> for OperatorType {
    fn from(value: usize) -> OperatorType {
        match value {
            0 => OperatorType::Sum,
            1 => OperatorType::Product,
            2 => OperatorType::Min,
            3 => OperatorType::Max,
            4 => OperatorType::Literal,
            5 => OperatorType::Gt,
            6 => OperatorType::Lt,
            7 => OperatorType::Eq,
            _ => panic!("Unknown operator type value: {}", value),
        }
    }
}

#[derive(Debug)]
struct Packet {
    version: usize,
    packet_type: PacketType,
    length_type: LengthType,
    length: Option<usize>,
    packets: Vec<Packet>,
    value: Option<usize>,
}

impl Packet {
    pub fn new() -> Self {
        Packet {
            version: 0,
            packet_type: PacketType::Unknown,
            length_type: LengthType::Unknown,
            length: None,
            packets: vec![],
            value: None,
        }
    }

    pub fn parse(&mut self, bits: &str) -> Result<(), Box<dyn Error>> {
        let mut parser = Parser {
            bits,
            cursor: 0,
            state: ParserState::ParseVersion,
        };

        while parser.state != ParserState::Finished {
            match parser.state {
                // 3 bits (version)
                ParserState::ParseVersion => self.parse_packet_version(&mut parser)?,

                // 3 bits (type id)
                ParserState::ParseTypeID => self.parse_type_id(&mut parser)?,

                // 1 bit (length type id)
                ParserState::ParseLengthTypeID => self.parse_length_type_id(&mut parser)?,

                // 11 or 15 bits, depending on length type id
                ParserState::ParseLength => self.parse_length(&mut parser)?,

                // variable length, chunks of 5, last chunk denoted by leading 0
                ParserState::ParseLiteralValue => self.parse_literal_value(&mut parser)?,

                // sub packets with known length (recurse)
                ParserState::ParseSubPacketsByLength(total_length) => {
                    self.parse_sub_packets_by_length(&mut parser, total_length)?
                }

                // sub packets with known count (recurse)
                ParserState::ParseSubPacketsByCount(total_count) => {
                    self.parse_sub_packets_by_count(&mut parser, total_count)?
                }

                // after parsing, a value can be computed for operator type packets
                ParserState::CalculateOperationValue => {
                    self.calculate_operation_value(&mut parser);
                }

                // "This should never happen." -Solomon Hawk, 12/16/2021
                ParserState::Finished => unreachable!(),
            }
        }

        Ok(())
    }

    pub fn version_sum(&self) -> usize {
        return self.version + self.packets.iter().map(|p| p.version_sum()).sum::<usize>();
    }

    pub fn get_value(&self) -> usize {
        match self.value {
            Some(p) => p,
            None => panic!("Cannot deref a packet without a value"),
        }
    }

    fn parse_packet_version(&mut self, parser: &mut Parser) -> Result<(), Box<dyn Error>> {
        self.version = take_int_from_bits(parser, 3)?;
        parser.state = ParserState::ParseTypeID;
        Ok(())
    }

    fn parse_type_id(&mut self, parser: &mut Parser) -> Result<(), Box<dyn Error>> {
        let operator_type: OperatorType = take_int_from_bits(parser, 3)?.into();

        match operator_type {
            OperatorType::Literal => {
                self.packet_type = PacketType::Literal;
                parser.state = ParserState::ParseLiteralValue;
            }

            op_type => {
                self.packet_type = PacketType::Operator(op_type);
                parser.state = ParserState::ParseLengthTypeID;
            }
        }

        Ok(())
    }

    fn parse_length_type_id(&mut self, parser: &mut Parser) -> Result<(), Box<dyn Error>> {
        self.length_type = take_int_from_bits(parser, 1)?.into();
        parser.state = ParserState::ParseLength;
        Ok(())
    }

    fn parse_length(&mut self, parser: &mut Parser) -> Result<(), Box<dyn Error>> {
        match self.length_type {
            // next 15 bits are a number that represents the total length in bits contained by this packet
            LengthType::Length => {
                parser.state =
                    ParserState::ParseSubPacketsByLength(take_int_from_bits(parser, 15)?);
            }

            // next 11 bits are a number that represents the number of sub-packets immediately contained by this packet
            LengthType::Count => {
                parser.state = ParserState::ParseSubPacketsByCount(take_int_from_bits(parser, 11)?);
            }

            _ => unreachable!(),
        }

        Ok(())
    }

    fn parse_literal_value(&mut self, parser: &mut Parser) -> Result<(), Box<dyn Error>> {
        let mut slice;
        let mut bit_string = String::new();

        loop {
            slice = slice_bits(parser.bits, &mut parser.cursor, 5);
            bit_string.push_str(&slice.chars().skip(1).collect::<String>());

            if slice.starts_with('0') {
                break;
            }
        }

        self.length = Some(parser.cursor);
        self.value = Some(bin_to_int(&bit_string)?);

        parser.state = ParserState::Finished;

        Ok(())
    }
    fn parse_sub_packets_by_length(
        &mut self,
        parser: &mut Parser,
        length: usize,
    ) -> Result<(), Box<dyn Error>> {
        let mut parsed_length = 0;

        while parsed_length < length {
            let parsed_packet: Packet = parser.bits[parser.cursor..].parse()?;
            let packet_len = &parsed_packet.length.expect("Packet must have a length");

            parsed_length += packet_len;
            parser.cursor += packet_len;

            self.packets.push(parsed_packet);
        }

        self.length = Some(parser.cursor);

        parser.state = ParserState::CalculateOperationValue;

        Ok(())
    }

    fn parse_sub_packets_by_count(
        &mut self,
        parser: &mut Parser,
        count: usize,
    ) -> Result<(), Box<dyn Error>> {
        let mut parsed_count = 0;

        while parsed_count < count {
            let parsed_packet: Packet = parser.bits[parser.cursor..].parse()?;
            parsed_count += 1;
            parser.cursor += &parsed_packet.length.expect("Packet must have a length");

            self.packets.push(parsed_packet);
        }

        self.length = Some(parser.cursor);

        parser.state = ParserState::CalculateOperationValue;

        Ok(())
    }
    fn calculate_operation_value(&mut self, parser: &mut Parser) {
        match self.packet_type {
            PacketType::Operator(OperatorType::Sum) => {
                self.value = Some(self.packets.iter().map(|p| p.get_value()).sum());
            }

            PacketType::Operator(OperatorType::Product) => {
                self.value = Some(self.packets.iter().map(|p| p.get_value()).product());
            }

            PacketType::Operator(OperatorType::Min) => {
                self.value = self.packets.iter().map(|p| p.get_value()).min()
            }

            PacketType::Operator(OperatorType::Max) => {
                self.value = self.packets.iter().map(|p| p.get_value()).max()
            }

            PacketType::Operator(OperatorType::Gt) => {
                self.value =
                    Some((self.packets[0].get_value() > self.packets[1].get_value()).into())
            }

            PacketType::Operator(OperatorType::Lt) => {
                self.value =
                    Some((self.packets[0].get_value() < self.packets[1].get_value()).into())
            }

            PacketType::Operator(OperatorType::Eq) => {
                self.value =
                    Some((self.packets[0].get_value() == self.packets[1].get_value()).into())
            }

            _ => unreachable!(),
        }

        parser.state = ParserState::Finished;
    }
}

#[derive(Debug)]
pub struct PacketParseError;

impl Error for PacketParseError {}

impl fmt::Display for PacketParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse packet")
    }
}

impl FromStr for Packet {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Packet, Self::Err> {
        let mut packet = Packet::new();

        if is_bin_str(s) {
            packet.parse(s)?
        } else {
            packet.parse(
                &s.chars()
                    .map(|c| hex_to_bin(c.to_string()))
                    .collect::<Result<Vec<String>, Box<dyn Error>>>()?
                    .join(""),
            )?
        }

        Ok(packet)
    }
}

fn is_bin_str(s: &str) -> bool {
    s.chars().all(|c| c == '0' || c == '1')
}

fn hex_to_bin(s: String) -> Result<String, Box<dyn Error>> {
    Ok(format!("{:0>4b}", usize::from_str_radix(&s, 16)?))
}

fn take_int_from_bits(parser: &mut Parser, amount: usize) -> Result<usize, Box<dyn Error>> {
    bin_to_int(slice_bits(parser.bits, &mut parser.cursor, amount))
}

fn bin_to_int(s: &str) -> Result<usize, Box<dyn Error>> {
    let bit_string: String = s
        .chars()
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join("");

    Ok(usize::from_str_radix(&bit_string, 2)?)
}

/**
 * Returns a string slice of a packet binary string starting at `cursor` and
 * ending at `cursor + amount`. Also increments `cursor` by `amount`.
 */
fn slice_bits<'a>(s: &'a str, cursor: &mut usize, amount: usize) -> &'a str {
    *cursor += amount;
    &s[*cursor - amount..*cursor]
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Result<Packet, Box<dyn Error>> {
    input.parse()
}

#[aoc(day16, part1)]
fn part1(packet: &Packet) -> usize {
    packet.version_sum()
}

#[aoc(day16, part2)]
fn part2(packet: &Packet) -> usize {
    packet.get_value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal() {
        let input = "D2FE28";
        let packet: Packet = input.parse().unwrap();

        assert_eq!(packet.version, 6);
        assert_eq!(packet.packet_type, PacketType::Literal);
        assert_eq!(packet.value, Some(2021));

        assert_eq!(packet.version_sum(), 6);
    }

    #[test]
    fn operator() {
        let input = "38006F45291200";
        let packet: Packet = input.parse().unwrap();

        assert_eq!(packet.version, 1);
        assert_eq!(packet.packet_type, PacketType::Operator(OperatorType::Lt));

        assert_eq!(packet.version_sum(), 9);
    }

    #[test]
    fn example1() {
        // 8A004A801A8002F478 represents an operator packet (version 4) which
        // contains an operator packet (version 1) which contains an operator
        // packet (version 5) which contains a literal value (version 6); this
        // packet has a version sum of 16.
        let input = "8A004A801A8002F478";
        let packet: Packet = input.parse().unwrap();

        assert_eq!(packet.version, 4);

        assert_eq!(packet.version_sum(), 16);
    }

    #[test]
    fn example2() {
        // 620080001611562C8802118E34 represents an operator packet (version 3)
        // which contains two sub-packets; each sub-packet is an operator packet
        // that contains two literal values. This packet has a version sum of 12.
        let input = "620080001611562C8802118E34";
        let packet: Packet = input.parse().unwrap();

        assert_eq!(packet.version, 3);

        assert_eq!(packet.version_sum(), 12);
    }

    #[test]
    fn example3() {
        // C0015000016115A2E0802F182340 has the same structure as the previous
        // example, but the outermost packet uses a different length type ID.
        // This packet has a version sum of 23.
        let input = "C0015000016115A2E0802F182340";
        let packet: Packet = input.parse().unwrap();

        assert_eq!(packet.version, 6);

        assert_eq!(packet.version_sum(), 23);
    }

    #[test]
    fn example4() {
        // A0016C880162017C3686B18A3D4780 is an operator packet that contains an
        // operator packet that contains an operator packet that contains five
        // literal values; it has a version sum of 31.
        let input = "A0016C880162017C3686B18A3D4780";
        let packet: Packet = input.parse().unwrap();

        assert_eq!(packet.version, 5);

        assert_eq!(packet.version_sum(), 31);
    }

    #[test]
    fn sum_op() {
        // C200B40A82 finds the sum of 1 and 2, resulting in the value 3
        let input = "C200B40A82";
        let packet: Packet = input.parse().unwrap();

        assert_eq!(packet.value, Some(3));
    }

    #[test]
    fn product_op() {
        // 04005AC33890 finds the product of 6 and 9, resulting in the value 54
        let input = "04005AC33890";
        let packet: Packet = input.parse().unwrap();

        assert_eq!(packet.value, Some(54));
    }

    #[test]
    fn min_op() {
        // 880086C3E88112 finds the minimum of 7, 8, and 9, resulting in the value 7
        let input = "880086C3E88112";
        let packet: Packet = input.parse().unwrap();

        assert_eq!(packet.value, Some(7));
    }

    #[test]
    fn max_op() {
        // CE00C43D881120 finds the maximum of 7, 8, and 9, resulting in the value 9
        let input = "CE00C43D881120";
        let packet: Packet = input.parse().unwrap();

        assert_eq!(packet.value, Some(9));
    }

    #[test]
    fn gt_op() {
        // D8005AC2A8F0 produces 1, because 5 is less than 15
        let input = "D8005AC2A8F0";
        let packet: Packet = input.parse().unwrap();

        assert_eq!(packet.value, Some(1));
    }

    #[test]
    fn lt_op() {
        // F600BC2D8F produces 0, because 5 is not greater than 15
        let input = "F600BC2D8F";
        let packet: Packet = input.parse().unwrap();

        assert_eq!(packet.value, Some(0));
    }

    #[test]
    fn eq_op() {
        // 9C005AC2F8F0 produces 0, because 5 is not equal to 15.
        let input = "9C005AC2F8F0";
        let packet: Packet = input.parse().unwrap();

        assert_eq!(packet.value, Some(0));
    }

    #[test]
    fn eq_nested_op() {
        // 9C0141080250320F1802104A08 produces 1, because 1 + 3 = 2 * 2
        let input = "9C0141080250320F1802104A08";
        let packet: Packet = input.parse().unwrap();

        assert_eq!(packet.value, Some(1));
    }
}
