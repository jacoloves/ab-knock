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
    string s;
    cin >> s;

    string first = s.substr(0, 3);
    string second = s.substr(3, 5);

    int second_i = stoi(second);

    if (second_i == 316)
    {
        cout << "No" << endl;
    }
    else if (second_i >= 1 && second_i <= 349)
    {
        cout << "Yes" << endl;
    }
    else
    {
        cout << "No" << endl;
    }

    return 0;
}
