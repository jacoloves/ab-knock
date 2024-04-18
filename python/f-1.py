def main():
    s = input()

    mp = {}
    for c in s:
        if c in mp:
            mp[c] += 1
        else:
            mp[c] = 1

    mp2 = {}
    for cnt in mp.values():
        if cnt in mp2:
            mp2[cnt] += 1
        else:
            mp2[cnt] = 1

    for cnt in mp2.values():
        if cnt != 2:
            print("No")
            return
    
    print("Yes")

if __name__ == "__main__":
    main()
