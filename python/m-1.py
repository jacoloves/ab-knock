def main():
    n = int(input())
    
    v1 = [input() for _ in range(n)]
    v2 = [input() for _ in range(n)]

    x = 0
    y = 0

    for i in range(n):
        for j in range(0, n):
            if v1[i][j] != v2[i][j]:
                x = i + 1
                y = j + 1

    print(x, y)


if __name__ == "__main__":
    main()
