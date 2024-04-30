def main():
    v1 = map(int, input().split())
    v2 = map(int, input().split())

    v1_sum = sum(v1)
    v2_sum = sum(v2)

    print(v1_sum - v2_sum + 1)

if __name__ == "__main__":
    main()
