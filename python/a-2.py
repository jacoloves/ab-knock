def main():
    n = int(input())
    d = list(map(int, input().split()))

    cnt = 0
    for i, j in enumerate(d):
        m_str = str(i + 1)
        for x in range(j):
            d_str = str(x + 1)
            if len(set(m_str + d_str)) == 1:
                cnt += 1

    print(cnt)


if __name__ == "__main__":
    main()
