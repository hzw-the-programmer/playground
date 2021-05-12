async function a1() {
    p1 = new Promise((resolve, reject) => {
        setTimeout(() => {
            resolve(1)
        }, 2000);
    })
    console.log('await p1 begin')
    r1 = await p1;
    console.log('await p1 end')
    return 2;
}

console.log('a1 begin')
a1().then(v => {console.log(v)});
console.log('a1 end')