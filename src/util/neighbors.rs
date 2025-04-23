const OFFSETS_INCLUDING_DIAGS: [(i32, i32); 8] = [
	(-1, 1),
	(0, 1),
	(1, 1),
	(-1, 0),
	(1, 0),
	(-1, -1),
	(0, -1),
	(1, -1),
];

const OFFSETS_EXCLUDING_DIAGS: [(i32, i32); 4] = [(0, 1), (-1, 0), (1, 0), (0, -1)];

pub fn get(pos: &(i32, i32)) -> impl Iterator<Item = (i32, i32)> + '_ {
	OFFSETS_INCLUDING_DIAGS
		.into_iter()
		.map(move |offset| (pos.0 + offset.0, pos.1 + offset.1))
}

pub fn get_no_diags(pos: &(i32, i32)) -> impl Iterator<Item = (i32, i32)> + '_ {
	OFFSETS_EXCLUDING_DIAGS
		.into_iter()
		.map(move |offset| (pos.0 + offset.0, pos.1 + offset.1))
}
