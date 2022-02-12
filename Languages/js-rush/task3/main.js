function main() {
    console.log(addition(0))
    console.log(addition(9))
    console.log(addition(-3))
}

function addition(num) {
    if (num >= 0)
        return ++num
    return --num
}

main()
