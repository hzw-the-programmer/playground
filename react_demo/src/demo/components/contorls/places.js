export const places = [
    {
        id: 1,
        name: 'ShenZhen',
        pid: 0,
        level: 1,
        children: [
            {
                id: 3,
                name: 'FuTian',
                pid: 1,
                level: 2,
                children: [
                    {
                        id: 6,
                        name: 'XiaSha',
                        pid: 3,
                        level: 3
                    }
                ]
            },
            {
                id: 4,
                name: 'NanShan',
                pid: 1,
                level: 2
            },
            {
                id: 5,
                name: 'LuoHu',
                pid: 1,
                level: 2
            },
            {
                id: 7,
                name: 'BaoAn',
                pid: 1,
                level: 2
            }
        ]
    },
    {
        id: 2,
        name: 'HongKong',
        pid: 0,
        level: 1
    }
]
