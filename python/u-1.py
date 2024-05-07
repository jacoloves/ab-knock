def main():
    n, l, r = map(int, input().split())
    a = list(map(int, input().split()))
    for i in range(n):
        if a[i] < l:
            print(l, end=" ")
        elif a[i] > r:
            print(r, end=" ")
        else:
            print(a[i], end=" ")

    print()


if __name__ == "__main__":
    main()
