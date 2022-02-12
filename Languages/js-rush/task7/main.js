function main() {
    console.log(isAutomorphic(5))
    console.log(isAutomorphic(8))
    console.log(isAutomorphic(76))
}

function isAutomorphic(num) {
    var pow = (num*num).toString()
    return pow.endsWith(num.toString())
}

main()
