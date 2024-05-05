def main():
    n,s,m,l = map(int, input().split())

    ans = 10**9

    for i in range(n+1):
        for j in range(n+1):
            for k in range(n+1):
                tmp = 6*i + 8*j + 12*k
                if tmp >= n:
                    ans = min(ans, s*i + m*j + l*k)

    print(ans)


if __name__ == "__main__":
    main()
