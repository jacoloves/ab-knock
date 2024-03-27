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
  int h, w, n;
  cin >> h >> w >> n;

  vector<vector<char>> grid(h, vector<char>(w, '.'));

  int x = 0, y = 0, dir = 0;

  while (n--) {
    if (grid[x][y] == '.') {
      grid[x][y] = '#';
      dir = (dir + 1) % 4;
    } else {
      grid[x][y] = '.';
      dir = (dir + 3) % 4;
    }

    if (dir == 0)
      x = (x - 1 + h) % h;
    else if (dir == 1)
      y = (y + 1) % w;
    else if (dir == 2)
      x = (x + 1) % h;
    else
      y = (y - 1 + w) % w;
  }

  for (int i = 0; i < h; i++) {
    for (int j = 0; j < w; j++) {
      cout << grid[i][j];
    }
    cout << endl;
  }

  return 0;
}
