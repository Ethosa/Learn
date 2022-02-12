function main() {
    console.log(isRepdigit(66))
    console.log(isRepdigit(0))
    console.log(isRepdigit(-11))
}

function isRepdigit(num) {
    if (num < 0)
        return false
    var str = num.toString()
    Array(str).forEach( function(elem, i) {
        if (elem != str[0])
            return false
    })
    return true
}

main()
