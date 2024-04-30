def main():
    n, s, k = map(int, input().split())

    sum_pq = 0

    for _ in range(0, n):
        p, q = map(int, input().split())
        sum_pq += p * q

    if sum_pq >= s:
        print(sum_pq)
    else:
        print(sum_pq + k)


if __name__ == "__main__":
    main()
