pragma circom 2.1.6;

/*
 * The circuit proves the statement "I know the 100th element of the Fibonacci
 * sequence, starting with constants a and b." When `a == 0` and `b == 1`, this is
 * proving knowledge of the 100th (standard) Fibonacci number.
 *
 * Instead of applying the constraint only on the final value, every addition in the
 * sequence is a constraint. Note that because of immutability of signals an array of
 * signals is instead created, with a constraint applied on each element.
 *
 * See immutability of signals [here](https://docs.circom.io/circom-language/signals/).
 */

template fibonacci() {
    /*
     * Given `initial_a` and `initial_b` signals as values used to start the
     * sequence, computes the `n_counts` Fibonacci number and returns it in
     * signal `final_value`.
     */

    /*
    signal input initial_a;
    signal input initial_b;
    signal values[n_counts];
    signal output final_value;

     * Dummy signal with multiplication constraint needed due to error in snarkjs.
     * See [this GitHub issue](https://github.com/iden3/snarkjs/issues/116).
    signal dummy <== initial_a * initial_b;

    values[0] <== initial_a;
    values[1] <== initial_a + initial_b;

    for (var ii = 2; ii < n_counts; ii++) {
        values[ii] <== values[ii - 2] + values[ii - 1];
    }

    final_value <== values[n_counts - 1] + values[n_counts - 2];
     */

    signal input a;
    signal input b;
    signal output c[3];
    signal p;

    p <== a * a;

    c[0] <== p * p;
    c[1] <== a * a + 2;
    c[2] <== b * b + 2;
}

component main = fibonacci();
