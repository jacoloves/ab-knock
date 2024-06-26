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

int MAX = 919;
int main() {
  int n;
  cin >> n;

  for (int i = n; i <= MAX; i++) {
    int ans = i;
    int mbase = i;
    int base = mbase % 10;
    mbase /= 10;
    int m1 = mbase % 10;
    mbase /= 10;
    int m2 = mbase % 10;

    if (m1 * m2 == base) {
      cout << ans << endl;
      return 0;
    }
  }
  return 0;
}
