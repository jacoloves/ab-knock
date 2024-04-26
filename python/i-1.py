def main():
    s = input()
    second = s[3:6]

    second_i = int(second)

    if second_i == 316:
        print("No")
    elif second_i >= 1 and second_i <= 349:
        print("Yes")
    else:
        print("No")

if __name__ == "__main__":
    main()
