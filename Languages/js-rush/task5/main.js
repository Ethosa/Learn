function main() {
    console.log(triangle(1))
    console.log(triangle(6))
    console.log(triangle(215))
}

function triangle(num) {
    var result = 1
    for (var i = 2; i <= num; ++i)
        result += i
    return result
}

main()
