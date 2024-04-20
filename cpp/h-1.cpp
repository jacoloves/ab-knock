#include <algorithm> // min, max, swap, sort, reverse, lower_bound, upper_bound
#include <bitset>    // bitset
#include <cctype>    // isupper, islower, isdigit, toupper, tolower
#include <cstdint>   // int64_t, int*_t
#include <cstdio>    // printf
#include <deque>     // deque
#include <iostream>  // cout, endl, cin
#include <map>       // map
#include <queue>     // queue, priority_queue
#include <set>       // set
#include <stack>     // stack
#include <string>    // string, to_string, stoi
#include <tuple>     // tuple, make_tuple
#include <unordered_map> // unordered_map
#include <unordered_set> // unordered_set
#include <utility>       // pair, make_pair
#include <vector>        // vector

using namespace std;
typedef long long ll;

const string tmp = "wbwbwwbwbwbw";

using P = pair<int, int>;

int main() { 
    int wh, br;
    cin >> wh >> br;
    for (int i = 0; i < (int) tmp.size(); i++) {
        int cnt_wh = 0, cnt_br = 0;
        for (int j = 0; j < wh + br; j++) {
            if (tmp[(i + j) % tmp.size()] == 'w') ++cnt_wh;
            else ++cnt_br;
        }
        if (wh == cnt_wh && br == cnt_br) {
            cout << "Yes" << endl;
            return 0;
        }
    }

    cout << "No" << endl;
    return 0; 
}
