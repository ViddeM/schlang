fn main() {
    let pepe = 45

    // Should debug print 20
    if pepe < 20 {
        dbg(10)
    } else if pepe >= 20 {
        dbg(20)
    }

    // Should debug print 40
    if 99 > 100 {
        dbg(30)
    } else {
        dbg(40) 
    }
}
