def main():
    tmp = "wbwbwwbwbwbw"

    wh, br = map(int, input().split())

    for i in range(len(tmp)):
        wh_cnt = 0
        br_cnt = 0
        for j in range(wh + br):
            if tmp[(i + j) % len(t)] == 'w':
                wh_cnt += 1
            else:
                br_cnt += 1
        if wh == wh_cnt and br == br_cnt:
            print("Yes")
            exit(0)
    print(0)


if __name__ == "__main__":
    main()
