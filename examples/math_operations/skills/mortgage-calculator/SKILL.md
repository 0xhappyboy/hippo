---
name: mortgage-calculator
description: Calculate monthly mortgage payment with detailed breakdown
version: 1.0.0
author: Hippox Team
parameters:
  - name: principal
    type: number
    description: Loan amount in USD
    required: true
  - name: annual_rate
    type: number
    description: Annual interest rate (e.g., 4.5 for 4.5%)
    required: true
  - name: years
    type: number
    description: Loan term in years
    required: true
  - name: show_breakdown
    type: boolean
    description: Show total payment and total interest breakdown
    required: false
    default: true
---

# Mortgage Calculator Skill

You are a mortgage calculation assistant.

## Current Task

Calculate monthly mortgage payment with:

- Principal: {{principal}}
- Annual interest rate: {{annual_rate}}%
- Loan term: {{years}} years

## Available Atomic Skills

- **calculator** - Basic arithmetic operations
- **math_power** - Power operations (base^exponent)

## Instructions

Complete ALL steps below:

### Step 1: Calculate monthly interest rate

Call `calculator` with expression: "{{annual_rate}} / 100 / 12"
Save result as `monthly_rate`

### Step 2: Calculate total number of payments

Call `calculator` with expression: "{{years}} \* 12"
Save result as `total_payments`

### Step 3: Calculate (1 + monthly_rate)

Call `calculator` with expression: "1 + monthly_rate"
Save result as `one_plus_rate`

### Step 4: Calculate (1 + monthly_rate)^total_payments

Call `math_power` with:

- base: the result from Step 3 (e.g., "1.00375")
- exponent: the result from Step 2 (e.g., "360")
  Save result as `power_result`

### Step 5: Calculate monthly payment

Use formula: principal _ monthly_rate _ power_result / (power_result - 1)
Call `calculator` with expression: "{{principal}} _ monthly_rate _ power_result / (power_result - 1)"
Save result as `monthly_payment`

### Step 6: If show_breakdown is true

- Calculate total payment: monthly_payment \* total_payments
- Calculate total interest: total_payment - principal

## Output Format

Monthly Payment: $X.XX
Total Payment: $X.XX (if show_breakdown)
Total Interest: $X.XX (if show_breakdown)
