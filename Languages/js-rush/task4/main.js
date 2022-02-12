function main() {
    console.log(countTrue([true, false, false, true, false]))
    console.log(countTrue([false, false, false, false]))
    console.log(countTrue([]))
}

function countTrue(arr) {
    var result = 0
    arr.forEach( function(element, index) {
        if (element === true)
            result++
    })
    return result
}

main()
