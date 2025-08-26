
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
	let max :number =data[1]
    
    for (let i=0; i < data.length ;i++ ){
    
        if(data[i] > max){
            max =data[i]
        }
    }


    return max
}

console.log(getMaxValue(data1))
// output : 9
