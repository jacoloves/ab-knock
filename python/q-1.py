def main():
    k,g,m = map(int, input().split())

    glass = 0
    mag = 0
    
    for _ in range(0, k):
        if glass == g:
            glass = 0
        elif mag == 0:
            mag = m
        else:
            ml = min(g - glass, mag)
            glass += ml
            mag -= ml

    print("{} {}".format(glass, mag))


if __name__ == "__main__":
    main()
