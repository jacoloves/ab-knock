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
    int n;
    cin >> n;

    vector<int> a(n);

    for (int i=0; i<n; i++) {
        cin >> a[i];
    }

    vector<int> b(n-1);

    for (int i=0; i<n-1; i++) {
        b[i] = a[i] * a[i+1];
    }

    for (int i=0; i<n-1; i++) {
        cout << b[i] << " ";
    } 

    cout << endl;
    return 0; 
}