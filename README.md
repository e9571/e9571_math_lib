# e9571_math_lib Usage Examples

This document demonstrates the usage of the `e9571_math_lib` module in a Rust program, designed for casino-related mathematical operations such as calculating percentage changes, distances, and validating thresholds.

## Source Code Example

Below is a Rust program showcasing various mathematical functions from the `e9571_math_lib` module. The code performs calculations relevant to casino scenarios, such as comparing betting amounts or validating changes.

```rust
use e9571_math_lib::e9571_math_lib::*;

fn main() {
    // Example 1: Count_Add
    println!("=== Count_Add ===");
    println!("Percentage change (Real=110, Close=100): {}", count_add("110", "100"));
    println!("Percentage change (Close=0): {}", count_add("110", "0"));

    // Example 2: Count_Amo_Add
    println!("\n=== Count_Amo_Add ===");
    println!("Percentage change (Real=110, Close=100): {}", count_amo_add("110", "100"));
    println!("Percentage change (Close=0): {}", count_amo_add("110", "0"));

    // Example 3: Count_Add_float
    println!("\n=== Count_Add_float ===");
    println!("Percentage change (Real=110, Close=100): {}", count_add_float(110.0, 100.0));
    println!("Percentage change (Close=0): {}", count_add_float(110.0, 0.0));

    // Example 4: IsWithinFivePercent
    println!("\n=== IsWithinFivePercent ===");
    println!("Within 5% (100, 104): {}", is_within_five_percent(100.0, 104.0));
    println!("Within 5% (100, 106): {}", is_within_five_percent(100.0, 106.0));

    // Example 5: CalculateDistance
    println!("\n=== CalculateDistance ===");
    println!("Distance (100, 110): {}", calculate_distance(100.0, 110.0));
    println!("Distance (-10, 10): {}", calculate_distance(-10.0, 10.0));
}
```

## Explanation of Functions

The `e9571_math_lib` module provides mathematical utility functions for casino applications, such as calculating percentage changes, validating thresholds, and computing distances between values.

1. **`count_add`**:
   - Calculates the percentage change between two values provided as strings (`real` and `close`).
   - **Use Case**: Comparing actual and expected betting amounts in string format.

2. **`count_amo_add`**:
   - Similar to `count_add`, calculates percentage change for string inputs, possibly with different handling (e.g., for specific amount formats).
   - **Use Case**: Validating changes in aggregated bet amounts.

3. **`count_add_float`**:
   - Calculates the percentage change between two floating-point values (`real` and `close`).
   - **Use Case**: Precise calculations for financial data like payouts or balances.

4. **`is_within_five_percent`**:
   - Checks if two floating-point values are within 5% of each other.
   - **Use Case**: Validating if a bet or payout is within an acceptable margin.

5. **`calculate_distance`**:
   - Computes the absolute distance between two floating-point values.
   - **Use Case**: Measuring differences in betting amounts or game scores.

## Casino Scenario Usage

These functions are ideal for casino applications, such as:
- **Percentage Change**: Calculating differences in betting amounts or payouts (`count_add`, `count_amo_add`, `count_add_float`).
- **Threshold Validation**: Ensuring payouts or bets are within acceptable limits (`is_within_five_percent`).
- **Distance Calculation**: Measuring differences in scores, balances, or odds (`calculate_distance`).

## Example Output

The output depends on the implementation of `e9571_math_lib`. An example output might look like:

```
=== Count_Add ===
Percentage change (Real=110, Close=100): 10.0
Percentage change (Close=0): inf

=== Count_Amo_Add ===
Percentage change (Real=110, Close=100): 10.0
Percentage change (Close=0): inf

=== Count_Add_float ===
Percentage change (Real=110, Close=100): 10.0
Percentage change (Close=0): inf

=== IsWithinFivePercent ===
Within 5% (100, 104): true
Within 5% (100, 106): false

=== CalculateDistance ===
Distance (100, 110): 10.0
Distance (-10, 10): 20.0
```

## Notes
- **Module Dependency**: The `e9571_math_lib` module is assumed to be available and correctly implemented.
- **Error Handling**: Ensure proper handling for edge cases (e.g., division by zero in percentage calculations).
- **Casino Context**: The functions are suitable for processing betting amounts, validating payouts, or comparing game metrics.
- **GitHub Rendering**: This Markdown uses Rust syntax highlighting, clear headings, and structured explanations for optimal display.
- **Precision**: Floating-point calculations (`count_add_float`, `is_within_five_percent`, `calculate_distance`) use `f64` for accuracy.