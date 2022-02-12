function main() {
    console.log(weeklySalary([8, 8, 8, 8, 8, 0, 0]))
    console.log(weeklySalary([10, 10, 10, 0, 8, 0, 0]))
    console.log(weeklySalary([0, 0, 0, 0, 0, 12, 0]))
}

function weeklySalary(arr) {
    let result = 0
    arr.forEach(function(elem, i) {
        let overtime = elem - 8 > 0? elem - 8: 0
        if (i < 5)
            result += 10*(elem-overtime) + 15*overtime
        else
            result += 20*(elem-overtime) + 30*overtime
    })
    return result
}

main()
