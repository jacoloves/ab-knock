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
  int a, b;
  cin >> a >> b;

  vector<bool> vb(3);

  vb[a - 1] = true;
  vb[b - 1] = true;

  int cnt = 0;
  for (auto e : vb) {
    if (e)
      cnt++;
  }

  if (cnt == 2) {
    for (int i = 0; i < 3; i++) {
      if (!vb[i]) {
        cout << i + 1 << endl;
      }
    }
  } else {
    cout << -1 << endl;
  }
}
