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
    int n, q;
    cin >> n >> q;

    vector<int> v(q);

    for (int i = 0; i < q; i++)
    {
        cin >> v[i];
    }

    map<int, int> mp;

    for (int i = 0; i < q; i++)
    {
        mp[v[i]]++;
    }

    int cnt = 0;
    for (auto p : mp)
    {
        if (p.second % 2 != 0)
        {
            cnt++;
        }
    }

    cout << n - cnt << endl;

    return 0;
}
