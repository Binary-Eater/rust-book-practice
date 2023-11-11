fn main() {
    /* Part 1
     *
     * let guess = "42".parse().expect("Not a number!");
     *
     * The above fails to compile since the resultant of parse can be inferred
     * as multiple integer types.
     *
     * let guess: u32 = "42".parse().expect("Not a number!");
     *
     */

    /* Integer literals
     *
     * Can specify type of literal as suffix.
     * Example: 57u8, 10u32
     *
     * Can use _ as a separator for integral constants
     * Example: 1_000 == 1000
     *
     * Defaults to i32
     */

    /* Integer overflows
     *
     * In debug builds, an overflow leads to a panic.
     * In release builds, two's complement wrapping occurs.
     *
     * Depending blindly on two's complement wrapping is considered a bug.
     *
     * There are a couple of API wrappers for explicitly handling overflows.
     *
     * wrapping_* methods for explicitly making use of two's complement wrapping
     * checked_* methods that return None on overflow
     * overflowing_* methods that use wrapping and return a bool indicating wrapping occured
     * saturating_* methods cap values at their minimum/maximum
     *
     */

    /* Floating-point types */
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    /* Character type
     *
     * Uses four bytes to represent a Unicode Scalar Value
     */

    /* Array type */
    let _a1: [i32; 5] = [1, 2, 3, 4, 5];
    let _a2 = [3; 5];

    /* NOTE: unused variables can be kept by explicitly prefixing the names with
     * underscore.
     */
}
