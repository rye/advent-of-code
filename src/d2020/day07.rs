use regex::Regex;

pub fn process_color(color: &str) -> String {
	color.replace(' ', "_")
}

pub fn process_content_spec(content_spec: &str) -> (usize, String) {
	let content_inner_spec = Regex::new("(\\d+) (.+) bags?").unwrap();

	let captures = content_inner_spec.captures(content_spec).unwrap();

	let number = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
	let color = process_color(captures.get(2).unwrap().as_str());

	(number, color)
}

pub fn process_contents(contents: &str) -> Vec<(usize, String)> {
	match contents {
		"no other bags" => Vec::new(),
		_ => contents.split(", ").map(process_content_spec).collect(),
	}
}

pub fn duplicate_color(contents: &(usize, String)) -> Vec<String> {
	std::iter::repeat_n(contents.1.clone(), contents.0).collect()
}

pub fn ruleify(container: &str, contents: &[(usize, String)]) -> String {
	let colors: Vec<String> = contents.iter().flat_map(duplicate_color).collect();

	format!("in({}, [{}]).", container, colors.join(", "))
}

pub fn generate_output(data: &str) -> Result<(), Box<dyn std::error::Error>> {
	let bag_re = Regex::new("(.+) bags contain (.*).").unwrap();

	for line in data.lines() {
		let caps = bag_re.captures(line).unwrap();

		let color = process_color(caps.get(1).unwrap().as_str());
		let content = process_contents(caps.get(2).unwrap().as_str());

		println!("{}", ruleify(&color, &content));
	}

	println!(
		"
contains(X,Y) :- in(X,Z), member(Y,Z).

:- table contents/2.
contents(X,Y) :- contains(X,Y).
contents(X,Y) :- contents(X,Z), contents(Z,Y).

expand([],[]).
expand([BAG|BAG_LIST],EXPANSION) :-
    expand(BAG_LIST,LIST_EXPANSION),
    in(BAG,CONTENTS),
    append(CONTENTS,LIST_EXPANSION,EXPANSION).

expand_contents(X,[],[]) :- expand(X,[]).
expand_contents(X,Y,TRACE) :-
    expand(X,Z),
    expand_contents(Z,Y,TRACE1),
    append(Z,TRACE1,TRACE).

size(X,Z) :-
    expand_contents(X,_,TRACE),
    length(TRACE,Z)."
	);

	Ok(())
}
