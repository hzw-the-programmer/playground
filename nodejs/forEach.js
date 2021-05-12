const a = [0, 1, 2, 3, 4, 5, 6]

a.forEach(i => {
    console.log(i)
    if (i === 3) return
    console.log(i)
})
