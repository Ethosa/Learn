function main() {
    console.log(legendre(5, 100))
    console.log(legendre(2, 128))
    console.log(legendre(3, 50))
}

function legendre(p, n) {
    var sum = Math.floor(n/p)
        pow = p*p
    while (n >= pow){
        sum += Math.floor(n/pow)
        pow *= p
    }
    return sum
}

main()
