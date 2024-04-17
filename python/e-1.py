def main():
    n = int(input())

    v = list(map(int, input().split()))

    ans = 0
    for i in range(0, n-1):
        ans -= v[i]

    print(ans)


if __name__ == "__main__":
    main()
