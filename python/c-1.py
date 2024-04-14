def main():
    n, k = map(int, input().split())

    a = list(map(int, input().split()))

    ans = []

    for i in range(0, n):
        if a[i] % k == 0:
            ans.append(a[i] / k)

    output = " ".join(str(int(num)) for num in ans)
    print(output)


if __name__ == "__main__":
    main()
