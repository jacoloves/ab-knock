def sq(x):
    return x * x


def main():
    n = int(input())

    x = []
    y = []
    for _ in range(n):
        v1, v2 = map(int, input().split())

        x.append(v1)
        y.append(v2)

    for i in range(0, n):
        m = 0
        id = -1
        for j in range(0, n):
            if i == j:
                continue
            d = sq(x[i] - x[j]) + sq(y[i] - y[j])
            if d > m:
                m = d
                id = j

        print(id + 1)


if __name__ == "__main__":
    main()
