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
    int n, l;
    cin >> n >> l;

    vector<int> a(n);

    for (int i = 0; i < n; i++)
    {
        cin >> a[i];
    }

    int cnt = 0;
    for (auto e : a)
    {
        if (e >= l)
        {
            cnt++;
        }
    }

    cout << cnt << endl;
    return 0;
}
