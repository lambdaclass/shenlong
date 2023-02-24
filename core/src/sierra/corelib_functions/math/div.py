# def invmod(a, b):
#     if a == 0:
#         return 0
#     elif b % a == 0:
#         return 1
#     else:
#         return b - invmod(b % a, a) * b // a


# print(
#     (6 * invmod(3, 3618502788666131213697322783095070105623107215331596699973092056135872020481))
#     % 3618502788666131213697322783095070105623107215331596699973092056135872020481
# )
# print(
#     6
#     * pow(6, -1, 3618502788666131213697322783095070105623107215331596699973092056135872020481)
#     % 3618502788666131213697322783095070105623107215331596699973092056135872020481
# )
prime = 3618502788666131213697322783095070105623107215331596699973092056135872020481


def modular_inverse(a, m):
    """
    Calcule l'inverse modulaire de a modulo m en utilisant des soustractions répétées.
    Retourne l'inverse modulaire si il existe, sinon retourne None.
    """

    # Initialisation
    x = 0
    y = 1
    r = m
    s = a
    # Boucle de soustractions répétées
    while s != 0:
        print("start loop")
        q = r // s
        print("q = ", q)
        print("s = ", s)
        print("q * s", q * s)
        print("r, s = s, r - q * s")
        r, s = s, r - q * s
        print("r = ", r)
        print("s = ", s)
        print("r = ", r)
        x, y = y, x - q * y
        # print(q % prime)
    print("end loop")
    # Si r = pgcd(a, m) = 1, alors l'inverse modulaire est x
    print("inverse", x)
    return x

inv = modular_inverse(2, prime)

print("6 * inverse = ", 6 * inv)

print("6 * inverse mod prime =", 6 * inv % prime)

6 * 2**-1
