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
  vector<int> cnt(24);

  for (int i = 0; i < n; i++) {
    int w, x;
    cin >> w >> x;
    cnt[x] += w;
  }
  int ans = 0;
  for (int i = 0; i < 24; i++) {
    int sum = 0;
    for (int j = 0; j < 9; j++) {
      sum += cnt[(i + j) % 24];
    }
    ans = max(ans, sum);
  }

  cout << ans << endl;
  return 0;
}
