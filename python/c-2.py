def main():
    a, b = map(int, input().split())
    if a < b:
        if b - a > 2:
            print("No")
        else:
            print("Yes")
    else:
        if a - b > 3:
            print("No")
        else:
            print("Yes")


if __name__ == "__main__":
    main()
