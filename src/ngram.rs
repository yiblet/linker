use std::collections::VecDeque;

fn addr_of(s: &str) -> usize {
    s.as_ptr() as usize
}

pub fn positioned<'a, I: Iterator<Item = &'a str>>(
    input: &'a str,
    iter: I,
) -> impl Iterator<Item = (usize, &'a str)> {
    iter.map(move |sub| (addr_of(sub) - addr_of(input), sub))
}

pub fn ngram<T: Copy, I: Iterator<Item = T>>(
    mut iter: I,
    n: usize,
) -> impl Iterator<Item = Vec<T>> {
    if n == 0 {
        panic!("invalid length: {n}")
    }
    let mut buf = VecDeque::with_capacity(n + 1);
    while buf.len() < (n - 1) {
        match iter.next() {
            None => break,
            Some(v) => buf.push_back(v),
        }
    }

    iter.map(move |v| {
        buf.push_back(v);
        while buf.len() > n {
            buf.pop_front();
        }

        let res: Vec<_> = buf.iter().cloned().collect();
        res
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ngram_empty() {
        let input: Vec<i32> = vec![];
        let output: Vec<Vec<i32>> = vec![];
        assert_eq!(ngram(input.into_iter(), 2).collect::<Vec<_>>(), output);
    }

    #[test]
    fn test_ngram_single() {
        let input = vec![1];
        let output: Vec<Vec<i32>> = vec![];
        assert_eq!(ngram(input.into_iter(), 2).collect::<Vec<_>>(), output);
    }

    #[test]
    fn test_ngram_double() {
        let input = vec![1, 2];
        let output = vec![vec![1, 2]];
        assert_eq!(ngram(input.into_iter(), 2).collect::<Vec<_>>(), output);
    }

    #[test]
    fn test_ngram_multiple() {
        let input = vec![1, 2, 3, 4, 5];
        let output = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        assert_eq!(ngram(input.into_iter(), 2).collect::<Vec<_>>(), output);
    }

    #[test]
    fn test_ngram_trigrams() {
        let input = vec![1, 2, 3, 4, 5];
        let output = vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]];
        assert_eq!(ngram(input.into_iter(), 3).collect::<Vec<_>>(), output);
    }

    #[test]
    #[should_panic(expected = "invalid length: 0")]
    fn test_ngram_zero() {
        let input = vec![1, 2, 3, 4, 5];
        let _ = ngram(input.into_iter(), 0).collect::<Vec<_>>();
    }

    // use split whitespace as the postioned function
    fn positioned_split_whitespace(input: &str) -> impl Iterator<Item = (usize, &str)> {
        positioned(input, input.split_whitespace())
    }

    #[test]
    fn test_positioned_split_whitespace() {
        let text = "Hello, world! This is a test.";
        let result: Vec<_> = positioned_split_whitespace(text).collect();
        assert_eq!(
            result,
            vec![
                (0, "Hello,"),
                (7, "world!"),
                (14, "This"),
                (19, "is"),
                (22, "a"),
                (24, "test.")
            ]
        );
    }

    #[test]
    fn test_positioned_split_whitespace_with_leading_whitespace() {
        let text = "   Hello, world! This is a test.  ";
        let result: Vec<_> = positioned_split_whitespace(text).collect();
        assert_eq!(
            result,
            vec![
                (3, "Hello,"),
                (10, "world!"),
                (17, "This"),
                (22, "is"),
                (25, "a"),
                (27, "test.")
            ]
        );
    }

    #[test]
    fn test_positioned_split_whitespace_with_multiple_whitespace() {
        let text = "Hello,   world!  This   is a   test.  ";
        let result: Vec<_> = positioned_split_whitespace(text).collect();
        assert_eq!(
            result,
            vec![
                (0, "Hello,"),
                (9, "world!"),
                (17, "This"),
                (24, "is"),
                (27, "a"),
                (31, "test.")
            ]
        );
    }

    #[test]
    fn test_positioned_split_whitespace_empty_string() {
        let text = "";
        let result: Vec<_> = positioned_split_whitespace(text).collect();
        assert_eq!(result, Vec::<(usize, &str)>::new());
    }
}
