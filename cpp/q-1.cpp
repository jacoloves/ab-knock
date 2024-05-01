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
    int k, g, m;
    cin >> k >> g >> m;

    int glass = 0;
    int mag = 0;
    int ml;

    for (int i = 1; i <= k; i++)
    {
        if (glass == g)
        {
            glass = 0;
        }
        else if (mag == 0)
        {
            mag = m;
        }
        else
        {
            ml = min(g - glass, mag);
            glass += ml, mag -= ml;
        }
    }

    cout << glass << " " << mag << endl;
    return 0;
}
