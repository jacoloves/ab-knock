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
  const int MOD = 1e8;
  int n;
  cin >> n;
  vector<ll> x(n);
  for (int i = 0; i < n; i++) {
    cin >> x[i];
  }

  sort(x.begin(), x.end());

  ll result = 0;
  for (int i = 0; i < n; i++) {
    result += x[i] * ll(n - 1);
  }

  int r = n - 1;
  for (int i = 0; i < n; i++) {
    while (r >= 0 && x[i] + x[r] >= MOD)
      r--;
    result -= ll(n - max(r, i) - 1) * MOD;
  }

  cout << result << endl;

  return 0;
}
