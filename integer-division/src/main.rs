// This function performs integer division (a ÷ b) without using the division operator
// It uses repeated subtraction with binary search to optimize performance
pub fn divide(mut a: i32, mut b: i32) -> i32 {
    // Define constants for the minimum and maximum values of 32-bit integers
    const INT_MIN: i32 = i32::MIN; // -2^31
    const INT_MAX: i32 = i32::MAX; // 2^31 - 1

    // Special case 1: If dividing by 1, return the number itself
    // Example: 10 ÷ 1 = 10
    if b == 1 {
        return a;
    }

    // Special case 2: Handle potential overflow
    // When dividing INT_MIN by -1, result would exceed INT_MAX
    // Example: -2^31 ÷ (-1) would exceed 2^31 - 1
    if a == INT_MIN && b == -1 {
        return INT_MAX;
    }

    // Determine if the result should be positive or negative
    // Result is positive if both numbers have same sign (positive÷positive or negative÷negative)
    let sign: bool = (a > 0 && b > 0) || (a < 0 && b < 0);

    // Convert both numbers to negative to handle all cases uniformly
    // This prevents overflow issues when dealing with INT_MIN
    a = if a > 0 { -a } else { a };
    b = if b > 0 { -b } else { b };

    // Initialize the result counter
    let mut ans = 0;

    // While dividend (a) is less than or equal to divisor (b)
    while a <= b {
        // Start with the divisor as our working value
        let mut x: i32 = b;
        // Counter for how many times we've doubled our number
        let mut cnt: i32 = 1;

        // Try to double the number as many times as possible without overflow
        // x >= (INT_MAX >> 1) checks if doubling x would cause overflow
        // a <= (x << 1) checks if doubled value is still less than dividend
        while x >= (INT_MAX >> 1) && a <= (x << 1) {
            x <<= 1; // Double x (multiply by 2)
            cnt <<= 1; // Double count (multiply by 2)
        }

        // Add the count to our result
        ans += cnt;
        // Subtract the largest valid multiple from dividend
        a -= x;
    }

    // Return positive or negative result based on input signs
    if sign { ans } else { -ans }
}

fn main() {
    let dividend = 10;
    let divisor = 3;

    let result = divide(dividend, divisor);
    println!("{} ÷ {} = {}", dividend, divisor, result);
}
