extern crate advent_of_code_2020;

use self::advent_of_code_2020::*;

fn main() {
    let input = get_input("day_5.txt");

    let mut seats: Vec<(u32, u32)> = Vec::with_capacity(input.matches('\n').count());
    for line in input.trim().split('\n') {
        seats.push(get_seat(line));
    }

    let mut seat_ids: Vec<u32> = seats.iter()
        .map(|seat| { seat.0 * 8 + seat.1 })
        .collect();

    seat_ids.sort();

    println!("{}", seat_ids.last().unwrap());

    let gaps: Vec<(&u32, &u32)> = seat_ids.iter()
        .zip(seat_ids.iter().skip(1))
        .filter(|pair| { pair.1 - pair.0 != 1 })
        .collect();

    println!("{:?}", gaps);
}

fn btou32(bits: Vec<bool>) -> u32 {
    let mut max = 2u32.pow(bits.len() as u32);
    let mut index = 0;

    for bit in bits {
        max = max / 2;
        if bit {
            index += max
        }
    }

    index
}

fn get_seat(directions: &str) -> (u32, u32) {
    (
        btou32(directions[..7].chars().map(|c| c == 'B').collect()),
        btou32(directions[7..].chars().map(|c| c == 'R').collect())
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btou32() {
        let steps = vec![false, false, false];
        assert_eq!(0, btou32(steps));

        let steps = vec![true, false, true];
        assert_eq!(5, btou32(steps));

        let steps = vec![true, true, true];
        assert_eq!(7, btou32(steps));
    }

    #[test]
    fn test_get_seat() {
        let directions = "BFFFBBFRRR";
        assert_eq!((70, 7), get_seat(directions));

        let directions = "FFFBBBFRRR";
        assert_eq!((14, 7), get_seat(directions));

        let directions = "BBFFBBFRLL";
        assert_eq!((102, 4), get_seat(directions));
    }
}
