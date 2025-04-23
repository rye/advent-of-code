fn main() -> Result<(), Box<dyn std::error::Error>> {
	let solvers: std::collections::HashMap<u8, daocutil::Solver> = {
		{
			use d2021 as base;
			daocutil::generate_solvers![
				1_u8 day01|  => base::day01,
			2_u8 day02|  => base::day02,
			3_u8 day03|  => base::day03,
			4_u8 day04|  => base::day04,
			5_u8 day05|  => base::day05,
			6_u8 day06|  => base::day06,
			7_u8 day07|  => base::day07,
			8_u8 day08|  => base::day08,
			9_u8 day09|  => base::day09,
			10_u8 day10|  => base::day10,
			11_u8 day11|  => base::day11,
			12_u8 day12|  => base::day12,
			14_u8 day14|  => base::day14,
			23_u8 day23|  => base::day23,
			24_u8 day24|  => base::day24,
			25_u8 day25|  => base::day25,]
		}
	};
	let mut args = std::env::args();
	let _ = args.next();
	if let Some(ident) = args.next() {
		if let Some(ident) = daocutil::parse_day_identifier(&ident) {
			if let Some(handler) = solvers.get(&ident) {
				let data: String = match (
					std::fs::File::open(format!("inputs/day{ident:02}")),
					args.next(),
				) {
					(_, Some(filename)) => daocutil::string_from(std::fs::File::open(filename)?)?,
					(Ok(file), _) => daocutil::string_from(file)?,
					(_, None) => daocutil::string_from(std::io::stdin())?,
				};
				handler(&data)?;
			} else {
				println!("Day has no handler: {ident}");
			}
		} else {
			println!("Unknown day identifier: {ident}");
		}
	}
	Ok(())
}
