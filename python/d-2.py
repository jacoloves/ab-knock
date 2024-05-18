def main():
    n = int(input())
    calc_max = 919

    for i in range(n, calc_max + 1):
        mbase = i
        ans = i
        base = mbase % 10
        mbase //= 10
        m1 = mbase % 10
        mbase //= 10
        m2 = mbase % 10
        if base == m1 * m2:
            print(ans)
            return


if __name__ == "__main__":
    main()
