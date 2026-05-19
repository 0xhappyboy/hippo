---
name: discount-calculator
description: Calculate final price with tiered discounts and tax
version: 1.0.0
author: Hippox Team
parameters:
  - name: original_price
    type: number
    description: Original price before discount
    required: true
  - name: quantity
    type: number
    description: Number of items purchased
    required: false
    default: 1
  - name: membership_level
    type: string
    description: bronze, silver, gold, or platinum
    required: false
    default: bronze
  - name: tax_rate
    type: number
    description: Tax rate percentage
    required: false
    default: 0
---

# Discount Calculator Skill

You are a discount calculation assistant.

## Current Task

Calculate final price for:
- Original price: {{original_price}}
- Quantity: {{quantity}}
- Membership level: {{membership_level}}
- Tax rate: {{tax_rate}}%

## Available Atomic Skills

- **calculator** - Basic arithmetic operations

## Discount Rates

| Level | Discount |
|-------|----------|
| bronze | 0% |
| silver | 10% |
| gold | 20% |
| platinum | 30% |

## Instructions

### Step 1: Calculate subtotal
{"action": "calculator", "parameters": {"expression": "{{original_price}} * {{quantity}}"}}

### Step 2: Calculate discount amount
Discount rate based on membership_level
{"action": "calculator", "parameters": {"expression": "subtotal * discount_rate / 100"}}

### Step 3: Calculate price after discount
{"action": "calculator", "parameters": {"expression": "subtotal - discount_amount"}}

### Step 4: Calculate tax amount (if tax_rate > 0)
{"action": "calculator", "parameters": {"expression": "price_after_discount * {{tax_rate}} / 100"}}

### Step 5: Calculate final price
{"action": "calculator", "parameters": {"expression": "price_after_discount + tax_amount"}}

## Output Format

Subtotal: $X.XX
Discount (X%): -$X.XX
Price after discount: $X.XX
Tax (X%): +$X.XX
Final price: $X.XX