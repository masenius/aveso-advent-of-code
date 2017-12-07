use std::u32;

fn row_checksum_v1(row: &[u32]) -> u32 {
    assert!(row.len() > 0);
    let min_max = row.iter()
        .fold([u32::MAX, 0], |mut m_m, &d| {
            if d < m_m[0] {
                m_m[0] = d;
            }
            if d > m_m[1] {
                m_m[1] = d;
            }
            m_m
        });
    min_max[1] - min_max[0]
}

fn row_checksum_v2(row: &[u32]) -> u32 {
    for (i, d) in row.iter().enumerate() {
        for d2 in row.iter().skip(i+1).chain(row.iter().take(i)) {
            if d % d2 == 0 {
                return d / d2;
            }
        }
    }
    0
}

fn spreadsheet_checksum(spreadsheet: &str, checksum_fn: fn(&[u32]) -> u32) -> u32 {
    spreadsheet.lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<u32>>()
        })
        .map(|v| checksum_fn(&v))
        .sum()
}

fn main() {
    let input = include_str!("input");
    println!("Part 2: Spreadsheet checksum: {}", spreadsheet_checksum(input, row_checksum_v1));
    println!("Part 2: Spreadsheet checksum: {}", spreadsheet_checksum(input, row_checksum_v2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_row_checksum_v1() {
        assert_eq!(row_checksum_v1(&[5, 1, 9, 5]), 8);
        assert_eq!(row_checksum_v1(&[7, 5, 3]), 4);
        assert_eq!(row_checksum_v1(&[2, 4, 6, 8]), 6);
        assert_eq!(row_checksum_v1(&[2]), 0);
    }

    #[test]
    fn test_row_checksum_v2() {
        assert_eq!(row_checksum_v2(&[5, 9, 2, 8]), 4);
        assert_eq!(row_checksum_v2(&[9, 4, 7, 3]), 3);
        assert_eq!(row_checksum_v2(&[3, 8, 6, 5]), 2);
    }

    #[test]
    fn test_spreadsheet_checksum() {
        let input = "5 1 9 5
                     7 5 3
                     2 4 6 8";
        assert_eq!(spreadsheet_checksum(input, row_checksum_v1), 18);
    }
}
