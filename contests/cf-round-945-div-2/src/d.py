import sys

def query(l, x):
    print(f"? {l} {x}")
    sys.stdout.flush()
    ret = int(input())
    assert ret >= 0
    return ret

def answer(m):
    print(f"! {m}")
    sys.stdout.flush()
    ret = int(input())
    assert ret == 1

def solve_case():
    n, k = map(int, input().split())
    max_val = 0
    for i in range(n, 0, -1):
        r = query(1, i * n)
        if r <= n:
            assert r == n
            max_val = i
            break
    assert max_val > 0
    for i in range(n // k, 0, -1):
        m = i * max_val
        p = 0
        for j in range(1, k+1):
            if p >= n:
                p = 0
                break
            p = query(p + 1, m)
        if p == n:
            answer(m)
            return
    answer(-1)

def solve():
    t = int(input())
    for _ in range(t):
        solve_case()

solve()
