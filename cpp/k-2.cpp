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

  vector<pair<string, int>> p1;

  int s1 = 0;

  for (int i = 0; i < n; i++) {
    string s;
    int v;
    cin >> s >> v;
    p1.emplace_back(s, v);
    s1 += v;
  }

  sort(p1.begin(), p1.end());

  int mod_n = s1 % n;

  cout << p1[mod_n].first << endl;

  return 0;
}
