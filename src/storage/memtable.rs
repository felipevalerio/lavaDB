pub struct MemTableData {
	pub key: Vec<u8>,
	pub value: Option<Vec<u8>>,
	pub timestamp: u128,
	pub deleted: bool,
}



pub struct MemTable {
	data: Vec<MemTableData>,
	size: usize,
}