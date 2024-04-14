def main():
    input_s = input()
    n = int(input_s)

    ans_str = ""
    for i in range(1, n + 1):
        if i % 3 == 0:
            ans_str += "x"
        else:
            ans_str += "o"

    print(ans_str)


if __name__ == "__main__":
    main()
