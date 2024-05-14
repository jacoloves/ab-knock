def main():
    s = input()
    t = input()

    si = 0
    ans = []

    for i in range(len(t)):
        if s[si] == t[i]:
            ans.append(i + 1)
            si += 1

    for i in range(len(ans)):
        print(ans[i], end=" ")
    print()


if __name__ == "__main__":
    main()
