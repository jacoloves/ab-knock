def main():
    mm, dd = map(int, input().split())
    y, m, d = map(int, input().split())

    d += 1
    if d > dd:
        d = 1
        m += 1
        if m > mm:
            m = 1
            y += 1
    
    print("{} {} {}".format(y, m, d))


if __name__ == "__main__":
    main()
