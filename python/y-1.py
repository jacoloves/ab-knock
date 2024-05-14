def main():
    n, x, y, z = map(int, input().split())
    max_num = max(x, y)
    min_num = min(x, y)

    if min_num < z and z < max_num:
        print("Yes")
    else:
        print("No")


if __name__ == "__main__":
    main()
