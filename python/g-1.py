def main():
    n = int(input())

    v = list(map(int, input().split()))

    v2 = []

    for i in range(0, n-1):
        v2.append(v[i] * v[i+1])

    print(*v2)


if __name__ == "__main__":
    main()
