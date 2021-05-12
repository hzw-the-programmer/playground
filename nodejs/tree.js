const data = [
    {
        id: 1,
        name: '1',
        child: [
            {
                id: 11,
                name: '11',
            }
        ]
    },
    {
        id: 2,
        name: '2',
        child: [
            {
                id: 22,
                name: '22',
                child: [
                    {
                        id: 222,
                        name: '222',
                        child: [
                            {
                                id: 2222,
                                name: '2222',
                            }
                        ]
                    }
                ]
            }
        ]
    },
]

function convert(data) {
    const res = []
    if (data) {
        data.forEach(d => {
            res.push({
                title: d.name,
                key: d.id,
                children: convert(d.child)
            })
        })
    }
    return res
}

console.log(JSON.stringify(convert(data)))
