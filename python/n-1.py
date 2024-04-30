def main():
    ss = input()
    tt = input()

    circu = ["AB", "BC", "CD", "DE", "EA", "AE", "ED", "DC", "CB", "BA"]

    circu_int = 0
    for e in circu:
        if ss == e:
            circu_int += 1
        
        if tt == e:
            circu_int += 1

    if circu_int == 2:
        print("Yes")
    elif circu_int == 1:
        print("No")
    else:
        print("Yes")


if __name__ == "__main__":
    main()
