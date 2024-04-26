def main():
    n, q = map(int, input().split())

    tv = list(map(int, input().split()))

    mp = {}

    for t in tv:
        if t in mp:
            mp[t] += 1
        else:
            mp[t] = 1

    cnt = 0
    for e in mp.values():
        if e % 2 != 0:
            cnt += 1
    
    print(n - cnt)


if __name__ == "__main__":
    main()
