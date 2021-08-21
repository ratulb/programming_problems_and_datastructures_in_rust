///Leetcode 68. Text Justification

pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    if words.is_empty() || max_width == 0 {
        return vec![];
    }
    let mut result = vec![];
    let mut i = 0;
    let n = words.len();
    while i < n {
        let mut j = i + 1;
        let mut line_length = words[i].len();
        while j < n && (line_length + words[j].len() + (j - i - 1)) < max_width as usize {
            line_length += words[j].len();
            j += 1;
        }
        let diff = (max_width - line_length as i32) as usize;
        let num_words = j - i;
        if num_words == 1 || j >= n {
            left_justify(&words, diff, i, j, &mut result);
        } else {
            middle_justify(&words, diff, i, j, &mut result);
        }
        i = j;
    }
    result
}

fn left_justify(words: &Vec<String>, diff: usize, i: usize, j: usize, result: &mut Vec<String>) {
    let spaces_on_right = diff - (j - i - 1);
    result.push(words[i].to_owned());
    for k in i + 1..j {
        result.push("_".to_string());
        result.push(words[k].to_owned());
    }
    result.push((0..spaces_on_right).map(|_| '_').collect::<String>());
}

fn middle_justify(words: &Vec<String>, diff: usize, i: usize, j: usize, result: &mut Vec<String>) {
    let spaces_needed = j - i - 1;
    let spaces = diff / spaces_needed;
    let extra_spaces = diff % spaces_needed;
    result.push(words[i].to_owned());
    let mut extra_spaces = extra_spaces as i32;
    for k in i + 1..j {
        let spaces_to_apply = spaces + if extra_spaces > 0 { 1 } else { 0 };
        extra_spaces -= 1;
        result.push("_".to_string().repeat(spaces_to_apply));
        result.push(words[k].to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::full_justify;
    #[test]
    fn full_justify_test1() {
        let words = vec![
            "This".to_string(),
            "is".to_string(),
            "an".to_string(),
            "example".to_string(),
            "of".to_string(),
            "text".to_string(),
            "justification.".to_string(),
        ];
        let max_width = 15;
        let result = full_justify(words, max_width);
        println!("{:?}", result);
    }
}
