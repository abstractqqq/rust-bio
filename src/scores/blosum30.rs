use super::lookup;
use std::sync::LazyLock;

// taken from https://github.com/seqan/seqan/blob/master/include%2Fseqan%2Fscore%2Fscore_matrix_data.h
static MAT: LazyLock<[i32; 27 * 27]> = LazyLock::new(|| {
    [
        4, 0, -3, 0, 0, -2, 0, -2, 0, -1, 0, -1, 1, 0, 0, -1, 1, -1, 1, 1, 0, 1, -5, -4, 0, 0, -7,
        0, 5, -2, 5, 0, -3, 0, -2, -2, -2, 0, -1, -2, 4, -1, -2, -1, -2, 0, 0, -1, -2, -5, -3, 0,
        -1, -7, -3, -2, 17, -3, 1, -3, -4, -5, -2, -1, -3, 0, -2, -1, -2, -3, -2, -2, -2, -2, -2,
        -2, -2, -6, 0, -2, -7, 0, 5, -3, 9, 1, -5, -1, -2, -4, -3, 0, -1, -3, 1, -1, -1, -1, -1, 0,
        -1, -1, -2, -4, -1, 0, -1, -7, 0, 0, 1, 1, 6, -4, -2, 0, -3, -2, 2, -1, -1, -1, -1, 1, 2,
        -1, 0, -2, -1, -3, -1, -2, 5, -1, -7, -2, -3, -3, -5, -4, 10, -3, -3, 0, 1, -1, 2, -2, -1,
        -1, -4, -3, -1, -1, -2, -1, 1, 1, 3, -4, -1, -7, 0, 0, -4, -1, -2, -3, 8, -3, -1, -2, -1,
        -2, -2, 0, -1, -1, -2, -2, 0, -2, -1, -3, 1, -3, -2, -1, -7, -2, -2, -5, -2, 0, -3, -3, 14,
        -2, -2, -2, -1, 2, -1, -1, 1, 0, -1, -1, -2, -1, -3, -5, 0, 0, -1, -7, 0, -2, -2, -4, -3,
        0, -1, -2, 6, 4, -2, 2, 1, 0, 0, -3, -2, -3, -1, 0, 0, 4, -3, -1, -3, 0, -7, -1, -2, -1,
        -3, -2, 1, -2, -2, 4, 4, -2, 3, 2, -1, 0, -3, -2, -3, -2, 0, 0, 3, -3, 1, -2, 0, -7, 0, 0,
        -3, 0, 2, -1, -1, -2, -2, -2, 4, -2, 2, 0, 0, 1, 0, 1, 0, -1, 0, -2, -2, -1, 1, 0, -7, -1,
        -1, 0, -1, -1, 2, -2, -1, 2, 3, -2, 4, 2, -2, 0, -3, -2, -2, -2, 0, 0, 1, -2, 3, -1, 0, -7,
        1, -2, -2, -3, -1, -2, -2, 2, 1, 2, 2, 2, 6, 0, 0, -4, -1, 0, -2, 0, 0, 0, -3, -1, -1, 0,
        -7, 0, 4, -1, 1, -1, -1, 0, -1, 0, -1, 0, -2, 0, 8, 0, -3, -1, -2, 0, 1, 0, -2, -7, -4, -1,
        0, -7, 0, -1, -2, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, -1, -1, 0, -1, 0, 0, -1, 0, -2, -1,
        0, -1, -7, -1, -2, -3, -1, 1, -4, -1, 1, -3, -3, 1, -3, -4, -3, -1, 11, 0, -1, -1, 0, -1,
        -4, -3, -2, 0, -1, -7, 1, -1, -2, -1, 2, -3, -2, 0, -2, -2, 0, -2, -1, -1, 0, 0, 8, 3, -1,
        0, 0, -3, -1, -1, 4, 0, -7, -1, -2, -2, -1, -1, -1, -2, -1, -3, -3, 1, -2, 0, -2, -1, -1,
        3, 8, -1, -3, -1, -1, 0, 0, 0, -1, -7, 1, 0, -2, 0, 0, -1, 0, -1, -1, -2, 0, -2, -2, 0, 0,
        -1, -1, -1, 4, 2, 0, -1, -3, -2, -1, 0, -7, 1, 0, -2, -1, -2, -2, -2, -2, 0, 0, -1, 0, 0,
        1, 0, 0, 0, -3, 2, 5, 0, 1, -5, -1, -1, 0, -7, 0, -1, -2, -1, -1, -1, -1, -1, 0, 0, 0, 0,
        0, 0, -1, -1, 0, -1, 0, 0, -1, 0, -2, -1, 0, -1, -7, 1, -2, -2, -2, -3, 1, -3, -3, 4, 3,
        -2, 1, 0, -2, 0, -4, -3, -1, -1, 1, 0, 5, -3, 1, -3, 0, -7, -5, -5, -2, -4, -1, 1, 1, -5,
        -3, -3, -2, -2, -3, -7, -2, -3, -1, 0, -3, -5, -2, -3, 20, 5, -1, -2, -7, -4, -3, -6, -1,
        -2, 3, -3, 0, -1, 1, -1, 3, -1, -4, -1, -2, -1, 0, -2, -1, -1, 1, 5, 9, -2, -1, -7, 0, 0,
        0, 0, 5, -4, -2, 0, -3, -2, 1, -1, -1, -1, 0, 0, 4, 0, -1, -1, 0, -3, -1, -2, 4, 0, -7, 0,
        -1, -2, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, -1, -1, 0, -1, 0, 0, -1, 0, -2, -1, 0, -1,
        -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7,
        -7, -7, -7, -7, 1,
    ]
});

/// Return the BLOSUM30 substitution matrix score of [a, b]
///
/// # Example
///
/// ```
/// use bio::scores::blosum30;
/// assert_eq!(blosum30(b'H', b'A'), -2);
/// ```
pub fn blosum30(a: u8, b: u8) -> i32 {
    let a = lookup(a);
    let b = lookup(b);

    MAT[27 * a + b] // a = column index, b = row index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blosum30() {
        let score1 = blosum30(b'H', b'H');
        assert_eq!(score1, 14);
        let score2 = blosum30(b'O', b'*');
        assert_eq!(score2, -7);
        let score3 = blosum30(b'A', b'*');
        assert_eq!(score3, -7);
        let score4 = blosum30(b'*', b'*');
        assert_eq!(score4, 1);
        let score5 = blosum30(b'X', b'X');
        assert_eq!(score5, -1);
        let score6 = blosum30(b'X', b'Z');
        assert_eq!(score6, 0);
    }
}
