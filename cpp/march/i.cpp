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

  int x = 0;
  int y = 0;

  for (int i = 0; i < n; i++) {
    int a, b;
    cin >> a >> b;

    x += a;
    y += b;
  }

  if (x > y) {
    cout << "Takahashi" << endl;
  } else if (x < y) {
    cout << "Aoki" << endl;
  } else {
    cout << "Draw" << endl;
  }

  return 0;
}
