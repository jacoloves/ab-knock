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

ll sq(ll x) { return x * x; }

int main() {
  int n;
  cin >> n;

  vector<ll> x(n), y(n);
  for (int i = 0; i < n; i++) {
    cin >> x[i] >> y[i];
  }

  for (int i = 0; i < n; i++) {
    ll m = 0, id = -1;
    for (int j = 0; j < n; j++) {
      if (i == j)
        continue;
      ll d = sq(x[i] - x[j]) + sq(y[i] - y[j]);
      if (d > m) {
        m = d;
        id = j;
      }
    }

    cout << id + 1 << endl;
  }

  return 0;
}
