### Table of Contents

- [Big O Notation](#big-o-notation)
  - [O(1) Constant Time](#o1-constant-time)
  - [O(n) Linear Time](#on-linear-time)
  - [O(log n) — Logarithmic time](#olog-n--logarithmic-time)


# Big O Notation

**Big-O is a way to measure how fast or slow an algorithm gets as the amount of data grows.**
It doesn’t count exact seconds; it looks at the pattern: as the input grows, does the running time rise slowly, moderately, or very steeply?

> Analogy:
>
> Supermarket checkout. If your cart has more items, the cashier takes longer.
>
> Big-O asks:
>
> _“How quickly will the checkout time increase if the number of items doubles?”_

## O(1) Constant Time

This means: no matter how much data there is, the step takes the same amount of time.

> Analogy:
>
> Numbered drawers. You have a cabinet with 1 million drawers, and you KNOW you need drawer #7.
> Whether the cabinet has 10 drawers or 1 million, you still go straight to drawer 7. The time stays the same.

Example code:

```typescript
// TypeScript

const data: number[] = [1, 2, 3, 4, 5];

function getValueItem(n: number): number {
  return data[n];
}

console.log(getValueItem(2));
// output : 3
console.log(getValueItem(4));
// output : 5
```

## O(n) Linear Time

This means that work time increase in proportion of data.
If there twice as such data, the work will take approximately twice as long.

> Analogy:
>
> A teacher checks homework and wants to find the highscore from his 20 students, so he would also checks all 20 homework books until he find highscore in the class

Example code:

```typescript
// TypeScript

const data: number[] = [1, 2, 6, 4, 5, 9, 0, 3];

function getMaxValue(data: number[]): number {
  if (data.length === 0) throw new Error("empty array");

  let max: number = data[0];

  for (let i = 0; i < data.length; i++) {
    if (data[i] > max) {
      max = data[i];
    }
  }

  return max;
}

console.log(getMaxValue(data));
// output : 9
```

## O(log n) — Logarithmic time

With each step, you cut the possibilities in half.
As a result, the number of steps increases very slowly, even though the data is large.

> Analogy:
>
> Online shopping: find the cheapest items that are enough to qualify for free shipping.

> For example, a store offers free shipping for purchases exceeding Rp150,000. You have a list of items sorted in ascending order.

```typescript
// TypeScript
function firstPriceGte(prices: number[], target: number): number | null {
  let lo = 0,
    hi = prices.length - 1,
    ans = -1;
  while (lo <= hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (prices[mid] >= target) {
      ans = mid;
      hi = mid - 1;
    } else {
      lo = mid + 1;
    }
  }
  return ans === -1 ? null : prices[ans];
}

const priceList = [20_000, 45_000, 60_000, 90_000, 120_000, 160_000, 200_000];
const thresholdFreeDelivery = 150_000;
const i = firstPriceGte(priceList, thresholdFreeDelivery);
console.log(i);
// output : 160000
```

More code examples : [TypeScript](https://github.com/harun-alrosyid/algorithm-n-fundamental/blob/main/big-o-notation/big-o-notation.ts) | [Go](https://github.com/harun-alrosyid/algorithm-n-fundamental/blob/main/big-o-notation/big-o-notation.go) | [Python](https://github.com/harun-alrosyid/algorithm-n-fundamental/blob/main/big-o-notation/big-o-notation.py) | [Rust](https://github.com/harun-alrosyid/algorithm-n-fundamental/blob/main/big-o-notation/big-o-notation.rs)