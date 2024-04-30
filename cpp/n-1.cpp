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
    string ss, tt;
    cin >> ss;
    cin >> tt;

    vector<string> circu = {"AB", "BC", "CD", "DE", "EA", "AE", "ED", "DC", "CB", "BA"};

    int circu_cnt = 0;

    for (int i = 0; i < circu.size(); i++)
    {
        if (ss == circu[i])
        {
            circu_cnt++;
        }
        if (tt == circu[i])
        {
            circu_cnt++;
        }
    }

    if (circu_cnt == 2)
    {
        cout << "Yes" << endl;
    }
    else if (circu_cnt == 1)
    {
        cout << "No" << endl;
    }
    else
    {
        cout << "Yes" << endl;
    }

    return 0;
}
