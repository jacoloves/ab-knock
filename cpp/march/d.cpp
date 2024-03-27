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

  vector<ll> q;
  vector<ll> a;

  for (int i = 0; i < n; i++) {
    int x;
    ll k;
    cin >> x >> k;

    if (x == 1) {
      q.push_back(k);
    } else {
      reverse(q.begin(), q.end());
      a.push_back(q[k - 1]);
      reverse(q.begin(), q.end());
    }
  }

  for (auto v = a.begin(); v != a.end(); v++) {
    cout << *v << endl;
  }
  return 0;
}
