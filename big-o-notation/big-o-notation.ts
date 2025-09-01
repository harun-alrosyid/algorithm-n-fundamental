
// Big O(1)
const data = [1, 2, 3, 4, 5];

function getValueItem(n) {
  return data[n];
}

console.log(getValueItem(2));
// output : 3
console.log(getValueItem(4));
// output : 5

const data1 : number[] = [1,2,6,4,5,9,0,3]

// Big O(n)
function getMaxValue(data:number[]):number{	
    if (data.length === 0) throw new Error("empty array");
	let max :number =data[0]
    
    for (let i=0; i < data.length ;i++ ){
    
        if(data[i] > max){
            max =data[i]
        }
    }


    return max
}

console.log(getMaxValue(data1))
// output : 9

// Big O(log n)
function firstPriceGte(prices: number[], target: number): number|null {
  let lo = 0, hi = prices.length - 1, ans = -1;
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

// Big O(n^2)

const tops: string[] = ["t-shirt", "shirt", "sweater"];
const bottoms: string[] = ["jeans", "pants", "shorts"];

function outfitCombo(tops: string[], bottoms: string[]): [string, string][] {
  let combos: [string, string][] = [];
  for (const t of tops) {
    for (const b of bottoms) {
      combos.push([t, b]);
    }
  }
  return combos;
}

const combos = outfitCombo(tops, bottoms);

console.log(combos);
// output : 
// [
//   ["t-shirt", "jeans"], 
//   ["t-shirt", "pants"], 
//   ["t-shirt", "shorts"], 
//   ["shirt", "jeans"], 
//   ["shirt", "pants"], 
//   ["shirt", "shorts"], 
//   ["sweater", "jeans"], 
//   ["sweater", "pants"], 
//   ["sweater", "shorts"]
//] 