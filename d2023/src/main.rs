fn main() -> Result<(), Box<dyn std::error::Error>> {
	let solvers: std::collections::HashMap<u8, daocutil::Solver> = {
		{
			use d2023 as base;
			daocutil::generate_solvers![
				1_u8 day01|  => base::day01,
				2_u8 day02|  => base::day02,
				3_u8 day03|  => base::day03,
			]
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
