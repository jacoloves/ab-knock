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
  int n, m;
  cin >> n >> m;
  vector<int> a(m);

  for (int i = 0; i < m; i++) {
    cin >> a[i];
  }

  vector<vector<int>> x(n, vector<int>(m));

  for (int i = 0; i < n; i++) {
    for (int j = 0; j < m; j++) {
      cin >> x[i][j];
    }
  }

  vector<int> b;

  for (int j = 0; j < m; j++) {
    int tmp = 0;
    for (int i = 0; i < n; i++) {
      tmp += x[i][j];
    }
    b.push_back(tmp);
  }

  for (int i = 0; i < m; i++) {
    if (a[i] > b[i]) {
      cout << "No" << endl;
      return 0;
    }
  }

  cout << "Yes" << endl;
  return 0;
}
