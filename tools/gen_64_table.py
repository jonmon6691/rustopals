a = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
for i in range(255):
    n = a.find(chr(i))
    if n == -1:
        print("None, ", end="")
    else:
        print(f"Some({n}), ", end="")

"""
static B64_DEC: [Option<u8>; 255] = [None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    Some(62), None, None, None, Some(63), Some(52), Some(53), Some(54),
    Some(55), Some(56), Some(57), Some(58), Some(59), Some(60), Some(61), None,
    None, None, None, None, None, None, Some(0), Some(1), Some(2), Some(3),
    Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11),
    Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18),
    Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), None,
    None, None, None, None, None, Some(26), Some(27), Some(28), Some(29),
    Some(30), Some(31), Some(32), Some(33), Some(34), Some(35), Some(36),
    Some(37), Some(38), Some(39), Some(40), Some(41), Some(42), Some(43),
    Some(44), Some(45), Some(46), Some(47), Some(48), Some(49), Some(50),
    Some(51), None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None,
    None];
"""