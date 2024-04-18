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

using P = pair<int, int>;

int main() { 
    string s;
    cin >> s;

    map<char, int> mp;

    for (int i = 0; i < s.size(); i++) {
        mp[s[i]]++;
    }

    map<int, int> mp2;

    for (auto p : mp) {
       mp2[mp[p.first]]++;
    }
    
    for (auto p : mp2) {
       if (p.second != 2) {
            cout << "No" << endl;
            return 0;
       }
    }

    cout << "Yes" << endl;

    return 0; 
}
