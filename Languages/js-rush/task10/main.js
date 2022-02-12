function main() {
    console.log(isPrimPythTriple([4, 5, 3]))
    console.log(isPrimPythTriple([7, 12, 13]))
    console.log(isPrimPythTriple([39, 15, 36]))
    console.log(isPrimPythTriple([77, 36, 85]))
}

function copr(a, b) {
    for (var i = 2; i <= 10; ++i) {
        if (a % i == 0 && b % i == 0)
            return false
    }
    return true
}

function isPrimPythTriple(arr) {
    arr = arr.sort()
    let a = arr[0]
        b = arr[1]
        c = arr[2]
        corprime = copr(a, b) && copr(b, c) && copr(a, c)
    if (corprime && Math.hypot(a, b) == c)
        return true
    return false
}

main()
