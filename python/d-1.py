def main():
    s = input()

    ss = set()

    for i in range(len(s)):
        for j in range(1, len(s) - i + 1):
            substr = s[i:i + j]
            ss.add(substr)

    print(len(ss))

if __name__ == "__main__":
    main()
