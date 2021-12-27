use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Segment {
    segments: HashSet<char>,
    digit: Option<u8>,
}

#[derive(Clone, Debug)]
struct SegmentMap {
    a: Option<char>,
    b: Option<char>,
    c: Option<char>,
    d: Option<char>,
    e: Option<char>,
    f: Option<char>,
    g: Option<char>,
}

#[derive(Clone, Debug)]
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
        let set : HashSet<char> = segment_to_convert.chars().collect();
        Segment {segments: set, digit: None}
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
        input_segment.segments.len() as i32
        }

fn find_segment_with_digit (segments : &Vec<Segment>, digit : u8) -> Option<&Segment> {
        segments.iter().find(|x| x.digit == Some(digit))
}

fn get_digits_by_segment_count (input_digits : &mut [Segment], segment_count : i32) -> Vec<&mut Segment> {
    input_digits.iter_mut().filter(|x| (count_valid_segments(x) == segment_count) && (x.digit == None)).collect::<Vec<&mut Segment>>()
}

fn _part1 (digitlines: Vec<DigitLine>) -> i32 {
    fn count_valid_nums (input_segments: &Vec<Segment>) -> i32 {
        fn count_part1_valid_segments (input_segment: &Segment) -> i32 {
            let segments = count_valid_segments(input_segment); 
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
            for segment in &mut *input_digits {
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
            //We should now have the initial segments.  We will then need to find the segment which stands for a, or the top middle, and map it.
            let seg_1 = find_segment_with_digit(&input_digits, 1u8).unwrap().clone();
            let seg_7 = find_segment_with_digit(&input_digits, 7u8).unwrap().clone();
            let seg_true_a = &seg_1.segments.symmetric_difference(&seg_7.segments).collect::<Vec<&char>>()[0];
            input_segment_map.a = Some(**seg_true_a); //Get the first character segment, which stands for segment A.
            {
                let mut six_segs = get_digits_by_segment_count(&mut input_digits[..], 6);
                let mut seg_6 = six_segs.iter_mut().find(|x| x.segments.intersection(&seg_1.segments).collect::<HashSet<&char>>().len() == 1).expect("Could not find segment 6");
                seg_6.digit = Some(6);
            }
            {
                let mut five_segs = get_digits_by_segment_count(&mut input_digits[..], 5);
                let mut seg_3 = five_segs.iter_mut().find(|x| x.segments.is_superset(&seg_1.segments)).expect("Could not find segment 3");
                seg_3.digit = Some(3);
            }
            {
                let new_input_digits = &input_digits.clone();
                let mut five_and_two = get_digits_by_segment_count(&mut input_digits[..], 5);
                {
                    let mut seg_5 = || {
                        let mut seg_5 = five_and_two.iter_mut().find(|x| x.segments.is_subset(&find_segment_with_digit(&new_input_digits, 6).expect("Could not find segment 6").segments)).expect("Could not find segment 5");
                        seg_5.digit = Some(5);
                        let imm_seg_5 = seg_5.clone();
                        imm_seg_5
                    };
                    let seg_5_clone = seg_5();
                    let seg_2 = five_and_two.iter_mut().find(|x| x.segments != seg_5_clone.segments).expect("Could not find segment 2");
                    seg_2.digit = Some(2);
                }
            }
            {
                let new_input_digits = &input_digits.clone();
                let mut nine_and_zero = get_digits_by_segment_count(&mut input_digits[..], 6);
                {
                    let mut seg_9 = || {
                        let mut seg_9 = nine_and_zero.iter_mut().find(|x| x.segments.is_superset(&find_segment_with_digit(new_input_digits, 5).expect("Could not find segment 5").segments)).expect("Could not find segment 9");
                        seg_9.digit = Some(9);
                        let imm_seg_9 = seg_9.clone();
                        imm_seg_9
                    };
                    let seg_9_clone = seg_9();
                    let mut seg_0 = nine_and_zero.iter_mut().find(|x| x.segments != seg_9_clone.segments).expect("Could not find segment 0");
                    seg_0.digit = Some(0);
                }
            }
            for seg in input_digits {
                println!("{:?}",seg);
            }
            println!("Next segment");
        }

        find_all_segments(segment_map, all_digits);
        
        //Find the answer segments
        for segment in &mut *ans_digits {
            let matching_segment = all_digits.iter().find(|x| x.segments == segment.segments).expect("No match found");
            segment.digit = Some(matching_segment.digit.expect("Matching digit did not have a number"));
        }

        let ans_num = ans_digits.iter().fold(0, |val, x|  val * 10 + (x.digit.expect("Did not find a digit in answers") as i32));
        println!("Ans_Nums: {:?} \n Sum: {:?}", ans_digits, ans_num);
        ans_num
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

// 1
// 7
// 4
// 8
// 
// 6 is the only 6 segment which shares NO segments with 1.
// 3 is the only 5 segment which has all of 1's segments.
// 
// 5 is a subset of 6, while
// 2 is not.
// 
// 9 is a superset of 5, while
// 0 is not.