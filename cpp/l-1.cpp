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
    vector<int> v1(9);
    vector<int> v2(8);

    for (int i = 0; i < 9; i++)
    {
        cin >> v1[i];
    }

    for (int i = 0; i < 8; i++)
    {
        cin >> v2[i];
    }

    int v1_sum = 0;
    int v2_sum = 0;

    for (int i = 0; i < 9; i++)
    {
        v1_sum += v1[i];
    }

    for (int i = 0; i < 8; i++)
    {
        v2_sum += v2[i];
    }

    cout << (v1_sum - v2_sum) + 1 << endl;
    return 0;
}
