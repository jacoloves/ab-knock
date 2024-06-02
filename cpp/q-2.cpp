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
  int n, m, k;
  cin >> n >> m >> k;
  vector<int> ks(m, 0);
  vector<int> r(m, 0);
  for (int i = 0; i < m; i++) {
    int c;
    cin >> c;
    for (int j = 0; j < c; j++) {
      int a;
      cin >> a;
      a--;
      ks[i] |= (1 << a);
    }
    string s;
    cin >> s;
    if (s == "o") {
      r[i] = 1;
    } else {
      r[i] = 0;
    }
  }

  int res = 0;
  for (int i = 0; i < (1 << n); i++) {
    bool jud = true;
    for (int j = 0; j < m; j++) {
      int ok = __builtin_popcount(i & ks[j]);
      if (ok >= k && r[j] == 0) {
        jud = false;
        break;
      }
      if (ok < k && r[j] == 1) {
        jud = false;
        break;
      }
    }
    if (jud) {
      res++;
    }
  }
  cout << res << endl;

  return 0;
}
