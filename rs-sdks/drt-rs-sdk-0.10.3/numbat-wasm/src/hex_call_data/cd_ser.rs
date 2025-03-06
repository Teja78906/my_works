use alloc::vec::Vec;

use super::SEPARATOR;

fn half_byte_to_hex_digit(num: u8) -> u8 {
	if num < 10 {
		b'0' + num
	} else {
		b'a' + num - 0xau8
	}
}

fn byte_to_hex(byte: u8) -> (u8, u8) {
	let digit1 = half_byte_to_hex_digit(byte >> 4);
	let digit2 = half_byte_to_hex_digit(byte & 0x0f);
	(digit1, digit2)
}

/// Serializes to Numbat's smart contract call format.
///
/// This format consists of the function name, followed by '@', follwed by hex-encoded argument bytes separated by '@' characters.
/// Example: "funcName@00000@aaaa@1234@@".
/// Arguments can be empty, in which case no hex digits are emitted.
/// Argument hex encodings will always have an even number of digits.
///
/// HexCallDataSerializer owns its output.
///
/// Converting from whatever type the argument to bytes is not in scope. Use the `serializer` module for that.
///
pub struct HexCallDataSerializer(Vec<u8>);

impl HexCallDataSerializer {
	pub fn new(func_name: &[u8]) -> Self {
		let mut data = Vec::with_capacity(func_name.len());
		data.extend_from_slice(func_name);
		HexCallDataSerializer(data)
	}

	pub fn as_slice(&self) -> &[u8] {
		self.0.as_slice()
	}

	fn push_byte(&mut self, byte: u8) {
		let (digit1, digit2) = byte_to_hex(byte);
		self.0.push(digit1);
		self.0.push(digit2);
	}

	pub fn push_argument_bytes(&mut self, bytes: &[u8]) {
		self.0.reserve(1 + bytes.len() * 2);
		self.0.push(SEPARATOR);
		for byte in bytes.iter() {
			self.push_byte(*byte);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_push_bytes_1() {
		let mut cd = HexCallDataSerializer::new(&*b"func");
		let arg_bytes: &[u8] = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
		cd.push_argument_bytes(arg_bytes);
		assert_eq!(cd.as_slice(), &b"func@0123456789abcdef"[..]);
	}

	#[test]
	fn test_push_bytes_2() {
		let mut cd = HexCallDataSerializer::new(&*b"func");
		let arg_bytes: &[u8] = &[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
		cd.push_argument_bytes(arg_bytes);
		cd.push_argument_bytes(arg_bytes);
		assert_eq!(
			cd.as_slice(),
			&b"func@0123456789abcdef@0123456789abcdef"[..]
		);
	}

	#[test]
	fn test_push_empty_1() {
		let mut cd = HexCallDataSerializer::new(&*b"func");
		cd.push_argument_bytes(&[][..]);
		assert_eq!(cd.as_slice(), &b"func@"[..]);
	}

	#[test]
	fn test_push_empty_2() {
		let mut cd = HexCallDataSerializer::new(&*b"func");
		cd.push_argument_bytes(&[][..]);
		cd.push_argument_bytes(&[][..]);
		assert_eq!(cd.as_slice(), &b"func@@"[..]);
	}

	#[test]
	fn test_push_empty_3() {
		let mut cd = HexCallDataSerializer::new(&*b"");
		cd.push_argument_bytes(&[][..]);
		cd.push_argument_bytes(&[][..]);
		cd.push_argument_bytes(&[][..]);
		assert_eq!(cd.as_slice(), &b"@@@"[..]);
	}

	#[test]
	fn test_push_some_empty_1() {
		let mut cd = HexCallDataSerializer::new(&*b"func");
		let arg_bytes: &[u8] = &[0xff, 0xff];
		cd.push_argument_bytes(arg_bytes);
		cd.push_argument_bytes(&[][..]);
		assert_eq!(cd.as_slice(), &b"func@ffff@"[..]);
	}

	#[test]
	fn test_push_some_empty_2() {
		let mut cd = HexCallDataSerializer::new(&*b"func");
		let arg_bytes: &[u8] = &[0xff, 0xff];
		cd.push_argument_bytes(&[][..]);
		cd.push_argument_bytes(&[][..]);
		cd.push_argument_bytes(arg_bytes);
		cd.push_argument_bytes(&[][..]);
		cd.push_argument_bytes(&[][..]);
		assert_eq!(cd.as_slice(), &b"func@@@ffff@@"[..]);
	}
}
