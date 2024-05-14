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
  vector<int> D(n);

  for (auto &&e : D)
    cin >> e;

  int cnt = 0;
  for (int m = 1; m <= n; ++m) {
    string month = to_string(m);
    for (int d = 1; d <= D[m - 1]; ++d) {
      string date = month + to_string(d);
      if (size(set<char>(date.begin(), date.end())) == 1)
        cnt++;
    }
  }

  cout << cnt << endl;
  return 0;
}
