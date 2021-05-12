const data = [
    {value: 1, children: [{
        value: 11
    }]},
    {value: 1, children: [{
        value: 12
    }]},
    {value: 1, children: [{
        value: 12, children: [{
            value: 121,
        }]
    }]},
    {value: 1, children: [{
        value: 13, children: [{
            value: 131,
        }]
    }]},
]

function combine(data) {
    const res = []

    data.forEach(d => {
        const fr = res.filter(r => r.value === d.value)
        if (fr.length === 0) {
            res.push(d)
        } else {
            if (!fr[0].children) {
                fr[0].children = []
            }
            if (!d.children) {
                d.children = []
            }
            const children = [...fr[0].children, ...d.children]
            fr[0].children = combine(children)
        }
    })

    return res
}

console.log(JSON.stringify(combine(data)))
