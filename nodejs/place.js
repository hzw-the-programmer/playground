const place = {
    label: '0',
    value: 0,
    children: [{
        label: '1',
        value: 1,
        children: [{
            label: '11',
            value: 11,
            children: [{
                label: '111',
                value: 111,
                children: []
            }]
        }],
    }, {
        label: '2',
        value: 2,
        children: [],
    }, {
        label: '3',
        value: 3,
        children: [{
            label: '31',
            value: 31,
            children: [],
        }, {
            label: '32',
            value: 32,
            children: [],
        }],
    }, {
        label: '4',
        value: 4,
        children: [{
            label: '41',
            value: 41,
            children: [],
        }, {
            label: '42',
            value: 42,
            children: [{
                label: '421',
                value: 421,
                children: [],
            }, {
                label: '422',
                value: 422,
                children: [],
            }],
        }, {
            label: '43',
            value: 43,
            children: [],
        }]
    }, {
        label: '5',
        value: 5,
        children: [{
            label: '51',
            value: 51,
            children: [],
        }, {
            label: '52',
            value: 52,
            children: [],
        }]
    }]
}

function findAncestors(place, id, ids) {
    const r = place.children.filter(c => c.value === id)
    if (r.length !== 0) {
        ids.push(place.value)
        return true
    } else {
        for (let i = 0; i < place.children.length; i++) {
            if (findAncestors(place.children[i], id, ids)) {
                ids.unshift(place.value)
                return true
            }
        }
        return false
    }
}

function getFirstPlace(place, ids) {
    ids.push(place.value)
    if (place.children.length !== 0) {
        getFirstPlace(place.children[0], ids)
    }
}

function getPlace(place, ids, i) {
    if (i > ids.length - 1) {
        return null
    }
    
    if (place.value !== ids[i]) {
        return null
    }

    const pl = {
        ...place,
        children: [],
    }
    
    place.children.forEach(p => {
        const tp = getPlace(p, ids, i+1)
        if (tp) {
            pl.children.push(tp)
        }
    })

    return pl
}

let ids = []
findAncestors(place, 32, ids)
console.log(ids)

ids = []
findAncestors(place, 422, ids)
console.log(ids)

ids = []
getFirstPlace(place, ids)
console.log(ids)

console.log(getPlace(place, [0], 0))
console.log(getPlace(place, [0, 3], 0))
console.log(getPlace(place, [0, 3, 31], 0).children[0].children)
