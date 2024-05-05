def main():
    n, l = map(int, input().split())
    a = list(map(int, input().split()))

    cnt = 0
    for i in range(0, n):
        if a[i] >= l:
            cnt += 1

    print(cnt)


if __name__ == "__main__":
    main()
