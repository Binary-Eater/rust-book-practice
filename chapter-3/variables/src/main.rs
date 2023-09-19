fn main() {
    /* Part 1
     *
     * let x = 5;
     * println!("The value of x is: {x}");
     * x = 6; - Cannot update immutable variable x
     * println!("The value of x is: {x}");
     */

    /* Part 2 */
    println!("----- BEGIN PART 2 -----");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("------ END PART 2 ------");

    /* Part 3
     *
     * const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
     */

    /* Part 4 */
    println!("----- BEGIN PART 4 -----");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    println!("------ END PART 4 ------");

    /* Part 5
     *
     * Shadowing enables changing both value and type information of the newly
     * declared variable.
     *
     * let spaces = "   ";
     * let spaces = spaces.len();
     *
     * The above compiles since the last instance of the spaces variable is of
     * integer type and has a value of the length of the previous spaces string
     * variable instance.
     *
     * let mut spaces = "   ";
     * spaces = spaces.len();
     *
     * The above fails to compile since the second line tries to update a
     * mutable string variable with an integer value.
     */
}
