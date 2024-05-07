def main():
    s = input()
    n = len(s)

    for i in range(n):
        print("{} ".format(s[i]), end="")

    print()


if __name__ == "__main__":
    main()
