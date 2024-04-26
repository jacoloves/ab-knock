def main():
    s = input()

    for i in range(0, 101):
        for j in range(0, 101):
            for k in range(0, 101):
                if 'A' * i + 'B' * j + 'C' * k == s:
                    print("Yes")
                    exit()
                
    print("No")


if __name__ == "__main__":
    main()
