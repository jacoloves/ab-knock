def main():
    n = int(input())
    a = list(map(int, input().split()))

    first_max = max(a)
    second_max = -1

    for e in a:
        if e == first_max:
            continue
        second_max = max(second_max, e)

    print(second_max)


if __name__ == "__main__":
    main()
