def main():
    n, x = map(int, input().split())
    a = list(map(int, input().split()))

    cnt = 0
    for e in a:
        if e <= x:
            cnt += e

    print(cnt)


if __name__ == "__main__":
    main()
