#[derive(Debug, Clone, Copy)]
pub enum ShiftMode {
    Carry,
    Number(u8),
    Repeat,
}

/// Checks if the amount of bits of a number is odd or even.
/// Returns true if its even.
pub fn bit_oddity(n: u8) -> bool {
    let mut n = n;
    let mut amount_of_ones = 0;

    while n > 0 {
        if n & 1 == 1 {
            amount_of_ones = amount_of_ones + 1;
        }

        n = n >> 1;
    }

    amount_of_ones % 2 == 0
}
