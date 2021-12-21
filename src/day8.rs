#[derive(Clone)]
struct Segment {
	a: bool,
	b: bool,
	c: bool,
	d: bool,
	e: bool,
	f: bool,
	g: bool,
	digit: Option<u8>,
}

#[derive(Clone)]
struct SegmentMap {
	a: Option<char>,
	b: Option<char>,
	c: Option<char>,
	d: Option<char>,
	e: Option<char>,
	f: Option<char>,
	g: Option<char>,
}

#[derive(Clone)]
struct DigitLine {
	all_digits: Vec<Segment>,
	answer_digits: Vec<Segment>,
	segment_map: SegmentMap,
}

fn parse (initial_contents : &String) -> Vec<DigitLine> {
	let lines = initial_contents.lines().collect::<Vec<&str>>();

	fn split_line_at_bar (str_to_split: &str) -> (&str, &str) {
		let split_string = str_to_split.split(" | ").collect::<Vec<&str>>();
		(split_string[0], split_string[1]) 
	}

	fn split_segment_group_into_segments (segment_group_to_split : &str) -> Vec<&str> {
		segment_group_to_split.split_whitespace().collect()
	}

	fn convert_segment_strings_into_segments (segment_to_convert : &str) -> Segment {
		let a = segment_to_convert.contains('a');
		let b = segment_to_convert.contains('b');
		let c = segment_to_convert.contains('c');
		let d = segment_to_convert.contains('d');
		let e = segment_to_convert.contains('e');
		let f = segment_to_convert.contains('f');
		let g = segment_to_convert.contains('g');
		Segment {a, b, c, d, e, f, g, digit: None}
	}

	let initial_segment_map = || SegmentMap{a: None, b: None, c: None, d: None, e: None, f: None, g:None};

	lines
		.iter()
		.map(|x| {
			let (all, ans) = split_line_at_bar(x);
			let all_segments = split_segment_group_into_segments(all).iter().map(|x| convert_segment_strings_into_segments(x)).collect::<Vec<Segment>>();
			let ans_segments = split_segment_group_into_segments(ans).iter().map(|x| convert_segment_strings_into_segments(x)).collect::<Vec<Segment>>();
			DigitLine {all_digits: all_segments, answer_digits: ans_segments, segment_map: initial_segment_map()}
		})
		.collect()
}

fn count_valid_segments (input_segment: &Segment) -> i32 {
			input_segment.a as i32 + 
			input_segment.b as i32 +
			input_segment.c as i32 +
			input_segment.d as i32 +
			input_segment.e as i32 +
			input_segment.f as i32 +
			input_segment.g as i32
		}

fn _part1 (digitlines: Vec<DigitLine>) -> i32 {
	fn count_valid_nums (input_segments: &Vec<Segment>) -> i32 {
		fn count_part1_valid_segments (input_segment: &Segment) -> i32 {
			let segments = 
				input_segment.a as i32 + 
				input_segment.b as i32 +
				input_segment.c as i32 +
				input_segment.d as i32 +
				input_segment.e as i32 +
				input_segment.f as i32 +
				input_segment.g as i32;
			if segments == 2 || segments == 3 || segments == 4 || segments == 7 {1} else {0}
		}
		input_segments.iter().map(|x| count_part1_valid_segments(x)).sum()
	}
	digitlines.iter().map(|x| {count_valid_nums(&x.answer_digits)}).sum()
}

fn part2 (digitlines: Vec<DigitLine>) -> i32 {
	fn convert_digitline_to_val (input_digitline : &DigitLine) -> i32 {
		let mut digitline = input_digitline.clone();
		let all_digits = &mut digitline.all_digits;
		let ans_digits = &mut digitline.answer_digits;
		let segment_map = &mut digitline.segment_map;

		fn find_all_segments (input_segment_map : &mut SegmentMap, input_digits : &mut Vec<Segment>) {
			//Convert input digits into the first 4 digits
			for segment in input_digits {
				let seg_count = count_valid_segments(segment);
				if seg_count == 2 {
					segment.digit = Some(1);
				} else if seg_count == 3 {
					segment.digit = Some(7);
				} else if seg_count == 4 {
					segment.digit = Some(4);
				} else if seg_count == 7 {
					segment.digit = Some(8);
				}
			}

		}
		unimplemented!()
	}

	digitlines
		.iter()
		.map(|x| convert_digitline_to_val(x))
		.sum()
}

pub fn run (initial_contents : &String) -> String {
	let initial_ints = parse(initial_contents);
	format!("{}",part2(initial_ints))
}

