#include <algorithm>     // min, max, swap, sort, reverse, lower_bound, upper_bound
#include <bitset>        // bitset
#include <cctype>        // isupper, islower, isdigit, toupper, tolower
#include <cstdint>       // int64_t, int*_t
#include <cstdio>        // printf
#include <deque>         // deque
#include <iostream>      // cout, endl, cin
#include <map>           // map
#include <queue>         // queue, priority_queue
#include <set>           // set
#include <stack>         // stack
#include <string>        // string, to_string, stoi
#include <tuple>         // tuple, make_tuple
#include <unordered_map> // unordered_map
#include <unordered_set> // unordered_set
#include <utility>       // pair, make_pair
#include <vector>        // vector

using namespace std;
typedef long long ll;

using P = pair<int, int>;

int main()
{
    int n;
    cin >> n;

    char v1[n][n];
    char v2[n][n];

    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < n; j++)
        {
            cin >> v1[i][j];
        }
    }

    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < n; j++)
        {
            cin >> v2[i][j];
        }
    }

    int x = 0;
    int y = 0;

    bool flag = false;

    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < n; j++)
        {
            if (v1[i][j] != v2[i][j])
            {
                x = i + 1;
                y = j + 1;
                flag = true;
            }

            if (flag)
            {
                break;
            }
        }
        if (flag)
        {
            break;
        }
    }

    cout << x << " " << y << endl;

    return 0;
}
